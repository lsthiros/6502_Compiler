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

fn get_factor<I>(token_stream: &mut TokenStream<I>) -> Result<Factor, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    if let Some(id) = token_stream.accept(TokenType::IDENTIFIER) {
        return Ok(Factor::Id(id.label.unwrap()))
    }
    let numeric = token_stream.expect(TokenType::NUMBER)?;
    return Ok(Factor::Numeric(numeric.number.unwrap()))
}

fn get_term<I>(token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    if let Some(op) = token_stream.accept(TokenType::MUL_OP) {
        let ret = AstExprNode::Node {
            op_type: BinOp::Mult(op.mul_op.unwrap()),
            left: get_term(token_stream)?
        };
        return Ok(Box::new(ret))
    }

    let terminal: AstExprNode = AstExprNode::Terminal(get_factor(token_stream)?);
    return Ok(Box::new(terminal))
}

fn get_sum<I>(token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    if let Some(op) = token_stream.accept(TokenType::SUM_OP) {
        let ret = AstExprNode::Node {
            op_type: BinOp::from_token_type(TokenType::SUM_OP, &op).unwrap(), 
            left: get_term(token_stream)?
        };
        return Ok(Box::new(ret))
    }

    return Ok(get_sum(token_stream)?)
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