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
    actual: Option<TokenType>
}

impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(actual_type) = &self.actual {
            write!(f, "Expected token {:?} but found {:?}", self.expected, actual_type)
        }
        else {
            write!(f, "Expected token {:?} but found EOF", self.expected)
        }
    }
}

impl Error for UnexpectedTokenError {

}

struct TokenStream<I: Iterator<Item = LexerToken>> (Peekable<I>);

impl<I: Iterator<Item = LexerToken>> TokenStream<I> {
    fn accept(&mut self, expected: TokenType) -> Option<LexerToken> {
        if let Some(top) = self.0.peek() {
            if top.token_type == expected {
                Some(self.0.next().unwrap())
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    fn is_eof(&mut self) -> bool {
        self.0.peek().is_none()
    }

    fn expect(&mut self, expected: TokenType) -> Result<LexerToken, UnexpectedTokenError> {
        if let Some(result) = self.accept(expected) {
            Ok(result)
        }
        else {
            let actual: Option<TokenType>;

            if let Some(top) = self.0.peek() {
                actual = Some(top.token_type);
            }
            else {
                actual = None;
            }

            let error = UnexpectedTokenError {
                actual: actual,
                expected: expected
            };
            return Err(error)
        }
    }
}

fn parse_stream(token_stream: Vec<LexerToken>) -> Result<String, String> {
    unimplemented!();
}