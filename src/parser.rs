use std::vec;
use std::error::Error;

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

struct TokenStream(Vec<LexerToken>);

impl TokenStream {
    fn accept(&mut self, token_type: TokenType) -> Option<LexerToken> {
        unimplemented!();
    }

    fn is_eof(&self) {
        unimplemented!();
    }

    fn expect(&mut self, token_type: TokenType) -> Result<LexerToken, &Error> {
        unimplemented!();
    }
}

fn parse_stream(token_stream: Vec<LexerToken>) -> Result<String, String> {
    // while the parse stream still has stuff, keep the parse stack
    // responses to the lookahead and stack should be shift or reduce
    let a = factor::Id("hello".to_string());
    unimplemented!();
}