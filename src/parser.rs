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

struct FuncDecl {
    name: String,
    args: Vec<String>
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
    unimplemented!();
}