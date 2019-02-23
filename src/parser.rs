use crate::lexer::LexerToken;
use crate::lexer::TokenType;
use crate::lexer::SumOp;
use crate::lexer::MulOp;
use crate::lexer::RelOp;

use crate::token_stream::TokenStream;
use crate::token_stream::UnexpectedTokenError;

enum BinOp {
    Sum(SumOp),
    Mult(MulOp),
    Rel(RelOp)
}

enum Factor {
    Id(String),
    Numeric(f64)
}

struct FuncDecl {
    name: String,
    args: Vec<String>
}


enum AstExprNode {
    Node{
        left: Box<AstExprNode>,
        op_type: BinOp
    },
    Terminal(Factor)
}

impl BinOp {
    fn from_token_type(token_type: TokenType, token: &LexerToken) -> Option<BinOp> {
        match token_type {
            TokenType::SUM_OP => {
                Some(BinOp::Sum(token.sum_op.unwrap()))
            }
            TokenType::MUL_OP => {
                Some(BinOp::Mult(token.mul_op.unwrap()))
            }
            TokenType::REL_OP => {
                Some(BinOp::Rel(token.rel_op.unwrap()))
            }
            _ => {
                None
            }
        }
    }
}

trait ConstructsAst<I: Iterator<Item = LexerToken>> {
    fn construct_ast(&self, token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError>;
}

struct BinOpConstructor<'a, I: Iterator<Item = LexerToken>> {
    next: &'a ConstructsAst<I>,
    op_type: TokenType
}

impl<'a, I: Iterator<Item = LexerToken>> ConstructsAst<I> for BinOpConstructor<'a, I> {
    fn construct_ast(&self, token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> {
        if let Some(op) = token_stream.accept(self.op_type) {
            let ret = AstExprNode::Node {
                op_type: BinOp::from_token_type(self.op_type, &op).unwrap(), 
                left: self.construct_ast(token_stream)?
            };
            return Ok(Box::new(ret))
        }

        return Ok(self.next.construct_ast(token_stream)?)
    }
}

struct FactorTerminalConstructor {
    // Zero size struct to hold trait for returning a terminating "Factor" node
}

impl<I: Iterator<Item = LexerToken>> ConstructsAst<I> for FactorTerminalConstructor {
    fn construct_ast(&self, token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> {
        let factor: Factor;
        if let Some(id) = token_stream.accept(TokenType::IDENTIFIER) {
            factor = Factor::Id(id.label.unwrap());
        }
        else {
            let numeric = token_stream.expect(TokenType::NUMBER)?;
            factor = Factor::Numeric(numeric.number.unwrap());
        }
        return Ok(Box::new(AstExprNode::Terminal(factor)))
    }
}

fn get_func_decl<I>(token_stream: &mut TokenStream<I>) -> Result<FuncDecl, UnexpectedTokenError> where I: Iterator<Item = LexerToken>{
    let name_token = token_stream.expect(TokenType::IDENTIFIER)?;
    let name: String = name_token.label.unwrap();
    let _: LexerToken = token_stream.expect(TokenType::L_PAREN)?;
    let mut args: Vec<String> = Vec::new();

    let mut continuing_list: bool = token_stream.accept(TokenType::R_PAREN).is_none();

    while continuing_list {
        let arg_token: LexerToken = token_stream.expect(TokenType::IDENTIFIER)?;
        let arg_name = arg_token.label.unwrap();
        args.push(arg_name);

        continuing_list = token_stream.accept(TokenType::COMMA).is_some();
    }

    if args.len() > 0 {
        let _: LexerToken = token_stream.expect(TokenType::R_PAREN)?;
    }

    let result = FuncDecl {
        name: name,
        args: args
    };
    return Ok(result);
}

fn parse_stream(token_stream: Vec<LexerToken>) -> Result<String, UnexpectedTokenError> {
    let mut stream = TokenStream(token_stream.iter().cloned().peekable());

    if let Some(def_token) = stream.accept(TokenType::DEF) {
        let func = get_func_decl(&mut stream)?;
    }
    else if let Some(ext) = stream.accept(TokenType::EXTERN) {
        let func = get_func_decl(&mut stream)?;
    }
    else {

    }
    unimplemented!();
}