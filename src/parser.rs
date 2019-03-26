use std::fmt;

use crate::lexer::LexerToken;
use crate::lexer::TokenType;
use crate::lexer::SumOp;
use crate::lexer::MulOp;
use crate::lexer::RelOp;

use crate::token_stream::TokenStream;
use crate::token_stream::UnexpectedTokenError;

use crate::graphviz::CreatesGraphviz;
use crate::graphviz::Graphviz;

#[derive(Debug)]
enum BinOp {
    Sum(SumOp),
    Mult(MulOp),
    Rel(RelOp)
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinOp::Sum(sum_op) => {
                write!(f, "{}", sum_op)
            }
            BinOp::Mult(mul_op) => {
                write!(f, "{}", mul_op)
            }
            BinOp::Rel(rel_op) => {
                write!(f, "{}", rel_op)
            }
        }
    }
}

#[derive(Debug)]
enum Factor {
    Id{
        id: String,
        optional_call: Option<Vec<Box<AstExprNode>>>
    },
    Numeric(f64)
}

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Factor::Id{
                id, optional_call
            } => {
                if optional_call.is_none() {
                    write!(f, "Id: {}", id)
                }
                else {
                    write!(f, "Call: {}", id)
                }
            }
            Factor::Numeric(numeric) => {
                write!(f, "{}", numeric)
            }
        }
    }
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
    SubNode(Box<AstExprNode>),
    Terminal(Factor)
}

impl CreatesGraphviz for AstExprNode {

    fn get_name(&self) -> String {
        match self {
            AstExprNode::Terminal(terminal) => {
                format!("{}", terminal)
            }
            AstExprNode::Node {
                left: _, op_type, next: _
            } => {
                format!("{}", op_type)
            }
            AstExprNode::SubNode(_) => {
                format!("()")
            }
        }
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
            AstExprNode::SubNode(sub_node) => {
                return vec![sub_node.as_ref()];
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


fn factor<I>(token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    let result: Box<AstExprNode>;
    let token = token_stream.expect_multi(&vec![TokenType::IDENTIFIER, TokenType::NUMBER, TokenType::L_PAREN])?;
    match token.token_type {
        TokenType::IDENTIFIER => {
            let optional_call: Option<Vec<Box<AstExprNode>>> = get_optional_call(token_stream)?;
            let factor = Factor::Id{
                id: token.label.unwrap(),
                optional_call: optional_call};
            result = Box::new(AstExprNode::Terminal(factor))
        }
        TokenType::NUMBER => {
            let numeric = token_stream.expect(TokenType::NUMBER)?;
            let factor = Factor::Numeric(numeric.number.unwrap());
            result = Box::new(AstExprNode::Terminal(factor))
        }
        TokenType::L_PAREN => {
            let expression = sum_expr(token_stream)?;
            let _ = token_stream.expect(TokenType::R_PAREN)?;
            result = Box::new(AstExprNode::SubNode(expression));
        }
        _ => {
            unreachable!()
        }
    }
    return Ok(result)
}

type NextConstructor<I> = fn (&mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError>;

fn construct_ast_inner<I>(token_stream: &mut TokenStream<I>, op_type: TokenType, next_constructor: NextConstructor<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    let left: Box<AstExprNode> = next_constructor(token_stream)?;
    if let Some(op) = token_stream.accept(op_type) {
        let ret = AstExprNode::Node {
            left: left,
            op_type: BinOp::from_token_type(op_type, &op).unwrap(), 
            next: next_constructor(token_stream)?
        };
        return Ok(Box::new(ret))
    }

    return Ok(left)
}

fn mult_expr<I>(token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    construct_ast_inner(token_stream, TokenType::MUL_OP, factor)
}

fn sum_expr<I>(token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    construct_ast_inner(token_stream, TokenType::SUM_OP, mult_expr)
}

fn get_optional_call<I>(token_stream: &mut TokenStream<I>) -> Result<Option<Vec<Box<AstExprNode>>>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    if token_stream.accept(TokenType::L_PAREN).is_some() {
        let mut arglist = Vec::<Box<AstExprNode>>::new(); 
        if token_stream.accept(TokenType::R_PAREN).is_none() {
            let mut continue_list = true;
            while continue_list {
                arglist.push(sum_expr(token_stream)?);
                continue_list = token_stream.accept(TokenType::COMMA).is_some();
            }
        }
        let _ = token_stream.expect(TokenType::R_PAREN)?;
        return Ok(Some(arglist));
    }
    else {
        Ok(None)
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

    let node: Box<AstExprNode> = sum_expr(&mut stream)?;
    let result = Graphviz::from(node.as_ref() as &CreatesGraphviz);

    result.write_file(String::from("./a.out"));
    return Ok(String::from("Ok"));
}