use std::iter::Peekable;
use std::error::Error;
use std::fmt;

use crate::lexer::TokenType;
use crate::lexer::LexerToken;

#[derive(Debug)]
pub struct UnexpectedTokenError {
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

pub struct TokenStream<I: Iterator<Item = LexerToken>> (pub Peekable<I>);

impl<I: Iterator<Item = LexerToken>> TokenStream<I> {
    pub fn accept(&mut self, expected: TokenType) -> Option<LexerToken> {
        if let Some(top) = self.0.peek() {
            if top.token_type == expected {
                Some(self.0.next().unwrap().clone())
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    pub fn multi(&mut self, types: Vec<TokenType>) -> Option<LexerToken> {
        if self.is_eof() {
            return None;
        }

        let mut itt = types.iter();
        let top_token: &LexerToken = self.0.peek().unwrap();
        if types.contains(&top_token.token_type) {
            return Some(self.0.next().unwrap().clone());
        }
        else {
            return None;
        }
    }

    pub fn is_eof(&mut self) -> bool {
        self.0.peek().is_none()
    }

    fn get_actual(&mut self) -> Option<TokenType> {
        if let Some(top) = self.0.peek() {
            return Some(top.token_type);
        }
        else {
            return None;
        }
    }

    pub fn expect_multi(&mut self, types: Vec<TokenType>) -> Result<LexerToken, UnexpectedTokenError> {
        if let Some(result) = self.multi(types) {
            return Ok(result);
        }
        else {
            let error = UnexpectedTokenError {
                actual: self.get_actual(),
                // TODO: Update the error to support multiple expected types
                expected: TokenType::SUM_OP,
            };
            return Err(error)
        }
    }

    pub fn expect(&mut self, expected: TokenType) -> Result<LexerToken, UnexpectedTokenError> {
        if let Some(result) = self.accept(expected) {
            Ok(result)
        }
        else {
            let error = UnexpectedTokenError {
                actual: self.get_actual(),
                expected: expected
            };
            return Err(error)
        }
    }
}