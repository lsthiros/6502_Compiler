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
    REL_OP,
    BIN_OP
}

enum BinOp {
    MULTIPLY,
    DIVIDE,
    ADD,
    SUBTRACT,
}

enum RelOp {
    LESS_THAN,
    LESS_THAN_EQ,
    EQUAL,
    GREATER_THAN,
    GREATER_THAN_EQ
}

struct LexerToken {
    token_type: TokenType,
    label: Option<String>,
    number: Option<f64>,
    rel_op: Option<RelOp>,
    bin_op: Option<BinOp>
}

impl LexerToken {
    fn from_single(token_type: TokenType) -> LexerToken {
        LexerToken {
            token_type: token_type,
            label: None,
            number: None,
            rel_op: None,
            bin_op: None
        }
    }

    fn from_bin_op(bin_op:BinOp) -> LexerToken {
        LexerToken {
            token_type: TokenType::BIN_OP,
            label: None,
            number: None,
            rel_op: None,
            bin_op: Some(bin_op)
        }
    }
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

// Pass in a character
enum StateResponse {
    CONTINUE,
    BACKTRACE,
    DONE
}

fn processState(state: LexerStateDescriptor, cur_char: char) -> Result<(StateResponse, LexerStateDescriptor, Option<LexerToken>), &'static str> {
    let next_state: LexerStateDescriptor;

    match state {
        LexerStateDescriptor::START => {
            if cur_char.is_alphabetic() {
                // Starts an IDENTIFIER
            }
            match cur_char {
                '(' => {
                    // This is a L_PAREN token
                    let resp = LexerToken::from_single(TokenType::L_PAREN);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                ')' => {
                    // R_PAREN token
                    let resp = LexerToken::from_single(TokenType::R_PAREN);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                ',' => {
                    // COMMA token
                    let resp = LexerToken::from_single(TokenType::COMMA);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                '+' => {
                    // PLUS token
                    let resp = LexerToken::from_bin_op(BinOp::ADD);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                '-' => {
                    // MINUS token
                    let resp = LexerToken::from_bin_op(BinOp::SUBTRACT);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                '/' => {
                    // DIVIDE token
                    let resp = LexerToken::from_bin_op(BinOp::SUBTRACT);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                '*' => {
                    // MULTIPLY token
                    let resp = LexerToken::from_bin_op(BinOp::MULTIPLY);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                '>' => {
                    // Transition to GT
                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::GT, None))
                }
                '<' => {
                    // Transition to LT
                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::LT, None))
                }
                '=' => {
                    // Transition to EQ
                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::EQ, None))
                }
                _ => {
                    return Err("Unrecognised character")
                }
            }
            // Starting point. Encompases single character tokens.
        }
        LexerStateDescriptor::IDENTIFIER => {
            // Alphanumeric identifier. includes keywords
        }
        LexerStateDescriptor::NUMERIC => {
            // A number literal. Could be a float or int
        }
        LexerStateDescriptor::NUMERIC_DOT => {
            // A number literal with a period. Another number must
            // follow. This will be a float.
        }
        LexerStateDescriptor::NUMERIC_FLOAT => {
            // A full float literal.
        }
        LexerStateDescriptor::EQ => {
            // An equal sign. Could be assignment or equality
        }
        LexerStateDescriptor::GT => {
            // GT sign. Could be followed by an equal.
        }
        LexerStateDescriptor::LT => {
            // LT sign. Could be followed by an equal.
        }
        _ => {
            println!("This should never happen");
        }
    }
    Err("Hahahaha whoops")
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

            }
            else if state.state == LexerStateDescriptor::START {
                state.state = LexerStateDescriptor::ACCEPT;
            }
            else {
                state.state = LexerStateDescriptor::ABORT;
            }
    }
}