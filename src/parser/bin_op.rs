use std::fmt;

use crate::lexer::SumOp;
use crate::lexer::MulOp;
use crate::lexer::RelOp;

use crate::token_stream::TokenStream;

use crate::lexer::TokenType;
use crate::lexer::LexerToken;

use crate::token_stream::UnexpectedTokenError;

use crate::graphviz::CreatesGraphviz;

#[derive(Debug)]
pub enum BinOp {
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
pub enum Factor {
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


#[derive(Debug)]
pub enum AstExprNode {
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
                format!("( )")
            }
        }
    }

    fn get_connections(&self) -> Vec<&CreatesGraphviz> {
        match self {
            AstExprNode::Terminal(terminal) => {
                if let Factor::Id{id: _, optional_call} = terminal {
                    let mut ret: Vec<&CreatesGraphviz> = Vec::new();
                    if let Some(arglist) = optional_call {
                        for arg in arglist {
                            ret.push(arg.as_ref())
                        }
                    }
                    return ret;
                }
                else {
                    return vec![]
                }
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
            let factor = Factor::Numeric(token.number.unwrap());
            result = Box::new(AstExprNode::Terminal(factor))
        }
        TokenType::L_PAREN => {
            let expression = expression(token_stream)?;
            let _ = token_stream.expect(TokenType::R_PAREN)?;
            result = Box::new(AstExprNode::SubNode(expression));
        }
        _ => {
            unreachable!()
        }
    }
    return Ok(result)
}

type AstConstructor<I> = fn (&mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError>;

fn construct_ast_inner<I>(token_stream: &mut TokenStream<I>, op_type: TokenType, next_constructor: AstConstructor<I>, current_constructor: AstConstructor<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    let left: Box<AstExprNode> = next_constructor(token_stream)?;
    if let Some(op) = token_stream.accept(op_type) {
        let ret = AstExprNode::Node {
            left: left,
            op_type: BinOp::from_token_type(op_type, &op).unwrap(), 
            next: current_constructor(token_stream)?
        };
        return Ok(Box::new(ret))
    }

    return Ok(left)
}

fn mult_expr<I>(token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    construct_ast_inner(token_stream, TokenType::MUL_OP, factor, mult_expr)
}

fn sum_expr<I>(token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    construct_ast_inner(token_stream, TokenType::SUM_OP, mult_expr, sum_expr)
}

fn rel_expr<I>(token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    construct_ast_inner(token_stream, TokenType::REL_OP, sum_expr, rel_expr)
}

pub fn expression<I>(token_stream: &mut TokenStream<I>) -> Result<Box<AstExprNode>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    rel_expr(token_stream)
}

fn get_optional_call<I>(token_stream: &mut TokenStream<I>) -> Result<Option<Vec<Box<AstExprNode>>>, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    if token_stream.accept(TokenType::L_PAREN).is_some() {
        let mut arglist = Vec::<Box<AstExprNode>>::new(); 
        if token_stream.accept(TokenType::R_PAREN).is_none() {
            let mut continue_list = true;
            while continue_list {
                arglist.push(expression(token_stream)?);
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