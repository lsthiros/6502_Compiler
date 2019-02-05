use std::str::Chars;

#[derive(PartialEq)]
enum LexerStateDescriptor {
    START,
    IDENTIFIER,
    NUMERIC,
    NUMERIC_DOT,
    NUMERIC_FLOAT,
    GT,
    LT,
    EQ,
    ACCEPT,
    ABORT
}

enum TokenType {
    IDENTIFIER,
    DEF,
    IF,
    THEN,
    ELSE,
    RETURN,
    L_PAREN,
    R_PAREN,
    COMMA,
    ASSIGN,
    REL_OP
}

enum RelOp {
    LESS_THAN,
    LESS_THAN_EQ,
    EQUAL,
    GREATER_THAN,
    GREATER_THAN_EQ
}

struct LexerToken {
    tokenType: TokenType,
    label: Option<String>,
    number: Option<f64>,
    relType: Option<RelOp>,
}

struct LexerState {
    state: LexerStateDescriptor,
    latest: Option<char>,
    position: u32,
    backtrace: bool
}

impl LexerState {
    // Get the next character of the text stream
    // If the stream is empty, return None
    fn next(&mut self, mut token_iterator: &Chars) -> Option<char> {
        if self.backtrace {
            self.backtrace = false;
            return self.latest;
        }

        let next_char: Option<char> = token_iterator.next();
        self.latest = next_char;
        return next_char;
    }

    fn backtrace(&mut self) {
        self.backtrace = true;
    }
}

pub fn lex_string(lex_string: String) {
    let mut state = LexerState {
        state: LexerStateDescriptor::START,
        latest: None,
        position: 0,
        backtrace: false
    };

    let mut cur_str: Vec<char>;
    let mut iterator = lex_string.chars();

    while state.state != LexerStateDescriptor::ABORT &&
        state.state != LexerStateDescriptor::ACCEPT {

            if let Some(cur_char) = state.next(&iterator) {
                match state.state {
                    LexerStateDescriptor::START => {

                    }
                    _ => {
                        println!("This should never happen");
                    }
                }
            }
            else if state.state == LexerStateDescriptor::START {
                state.state = LexerStateDescriptor::ACCEPT;
            }
            else {
                state.state = LexerStateDescriptor::ABORT;
            }
    }
}