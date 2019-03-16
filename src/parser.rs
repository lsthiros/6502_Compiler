use crate::lexer::LexerToken;
use crate::lexer::TokenType;
use crate::lexer::SumOp;
use crate::lexer::MulOp;
use crate::lexer::RelOp;

use crate::token_stream::TokenStream;
use crate::token_stream::UnexpectedTokenError;

use crate::graphviz::CreatesGraphviz;
use crate::graphviz::emit_graph;

#[derive(Debug)]
enum BinOp {
    Sum(SumOp),
    Mult(MulOp),
    Rel(RelOp)
}

#[derive(Debug)]
enum Factor {
    Id(String),
    Numeric(f64)
}

struct FuncDecl {
    name: String,
    args: Vec<String>
}


#[derive(Debug)]
enum AstExprNode {
    Node{
        left: Box<AstExprNode>,
        op_type: BinOp,
        next: Box<AstExprNode>
    },
    Terminal(Factor)
}

impl CreatesGraphviz for AstExprNode {

    fn get_name(&self) -> String {
        return String::from("hullo");
    }

    fn get_connections(&self) -> Vec<&CreatesGraphviz> {
        match self {
            AstExprNode::Terminal(_) => {
                return vec![];
            }
            AstExprNode::Node {
                left, op_type, next
            } => {
                let _ = op_type;
                return vec![left.as_ref(), next.as_ref()];
            }
        }
    }
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
        let left: Box<AstExprNode> = self.next.construct_ast(token_stream)?;
        if let Some(op) = token_stream.accept(self.op_type) {
            let ret = AstExprNode::Node {
                left: left,
                op_type: BinOp::from_token_type(self.op_type, &op).unwrap(), 
                next: self.construct_ast(token_stream)?
            };
            return Ok(Box::new(ret))
        }

        return Ok(left)
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

pub fn parse_stream(token_stream: &Vec<LexerToken>) -> Result<String, UnexpectedTokenError> {
    let mut stream = TokenStream(token_stream.iter().cloned().peekable());

    let term_parse = FactorTerminalConstructor {};
    let mult_parse = BinOpConstructor::<std::iter::Cloned::<std::slice::Iter::<'_, LexerToken>>> {
        op_type: TokenType::MUL_OP,
        next: &term_parse
    };
    let sum_parse = BinOpConstructor::<std::iter::Cloned::<std::slice::Iter::<'_, LexerToken>>> {
        op_type: TokenType::SUM_OP,
        next: &mult_parse
    };

    let node: Box<AstExprNode> = sum_parse.construct_ast(&mut stream)?;
    let result = emit_graph(node.as_ref());
    // if let Some(def_token) = stream.accept(TokenType::DEF) {
    //     let func = get_func_decl(&mut stream)?;
    // }
    // else if let Some(ext) = stream.accept(TokenType::EXTERN) {
    //     let func = get_func_decl(&mut stream)?;
    // }
    // else {
// 
    // }
    println!("{:?}", result);
    unimplemented!();
}