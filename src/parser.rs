use std::vec;
use std::iter;
use std::iter::Peekable;
use std::error::Error;
use std::fmt;

use crate::lexer::LexerToken;
use crate::lexer::TokenType;

enum mul_op {
    Multiply,
    Divide
}

enum sum_op {
    Add,
    Subtract
}

enum factor {
    Id(String),
    Numeric(f64)
}

struct func_decl {
    id: String,
    args: Vec<String>
}

enum parser_state {
    Start,
    Extern,
    Def
}

#[derive(Debug)]
struct UnexpectedTokenError {
    expected: TokenType,
    actual: TokenType
}

impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expected token {:?} but found {:?}", self.expected, self.actual)
    }
}

impl Error for UnexpectedTokenError {

}

struct TokenStream<I: Iterator<Item = LexerToken>> (Peekable<I>);

impl<I: Iterator<Item = LexerToken>> TokenStream<I> {
    fn accept(&mut self) -> Option<LexerToken> {
        unimplemented!()
    }

    fn is_eof(&self) {
        unimplemented!();
    }

    fn expect(&mut self, token_type: TokenType) -> Result<LexerToken, UnexpectedTokenError> {
        unimplemented!();
    }
}

fn parse_stream(token_stream: Vec<LexerToken>) -> Result<String, String> {
    unimplemented!();
}