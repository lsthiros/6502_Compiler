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
    EQ
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
    BIN_OP,
    NUMBER
}

enum BinOp {
    MULTIPLY,
    DIVIDE,
    ADD,
    SUBTRACT
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

    fn from_rel_op(rel_op: RelOp) -> LexerToken {
        LexerToken {
            token_type: TokenType::REL_OP,
            label: None,
            number: None,
            rel_op: Some(rel_op),
            bin_op: None
        }
    }

    fn from_label(label: String) -> LexerToken {
        LexerToken {
            token_type: TokenType::IDENTIFIER,
            label: Some(label),
            number: None,
            rel_op: None,
            bin_op: None
        }
    }

    fn from_number(number: f64) -> LexerToken {
        LexerToken {
            token_type: TokenType::NUMBER,
            label: None,
            number: Some(number),
            rel_op: None,
            bin_op: None
        }
    }
}

struct LexerState <'a>{
    state: LexerStateDescriptor,
    itt: Chars<'a>,
    latest: Option<char>,
    position: u32,
    backtrace: bool
}

impl<'a> LexerState<'a> {
    // Get the next character of the text stream
    // If the stream is empty, return None
    fn next(&mut self) -> Option<char> {
        if self.backtrace {
            self.backtrace = false;
            return self.latest;
        }

        let next_char: Option<char> = self.itt.next();
        self.latest = next_char;
        return next_char;
    }

    fn backtrace(&mut self) {
        self.backtrace = true;
    }
}

#[derive(PartialEq)]
enum StateResponse {
    CONTINUE,
    BACKTRACE,
    DONE
}

fn process_state(state: LexerStateDescriptor, cur_char: char, id: &mut Vec<char>) -> Result<(StateResponse, LexerStateDescriptor, Option<LexerToken>), &'static str> {
    let next_state: LexerStateDescriptor;

    match state {
        LexerStateDescriptor::START => {
            // Starting point. Encompases single character tokens.
            if cur_char.is_alphabetic() {
                // Starts an IDENTIFIER
                id.push(cur_char);
                return Ok((StateResponse::CONTINUE, LexerStateDescriptor::IDENTIFIER, None))
            }
            else if cur_char.is_numeric() {
                // Starts a NUMERIC
                id.push(cur_char);
                return Ok((StateResponse::CONTINUE, LexerStateDescriptor::NUMERIC, None))
            }
            else if cur_char.is_whitespace() {
                // Ignore whitespace
                return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, None))
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
        }
        LexerStateDescriptor::IDENTIFIER => {
            // Alphanumeric identifier. includes keywords
            if cur_char.is_alphanumeric() {
                id.push(cur_char);
            } else {
                let full_id: String = id.iter().collect();
                id.clear();
                match full_id.as_str() {
                    "def" => {
                        let resp = LexerToken::from_single(TokenType::DEF);
                        return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                    }
                    "return" => {
                        let resp = LexerToken::from_single(TokenType::DEF);
                        return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))

                    }
                    "if" => {
                        let resp = LexerToken::from_single(TokenType::IF);
                        return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                    }
                    "then" => {
                        let resp = LexerToken::from_single(TokenType::THEN);
                        return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                    }
                    "else" => {
                        let resp = LexerToken::from_single(TokenType::ELSE);
                        return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                    }
                    _ => {
                        let resp = LexerToken::from_label(full_id);
                        return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                    }
                }
            }
        }
        LexerStateDescriptor::NUMERIC => {
            // A number literal. Could be a float or int
            if cur_char.is_numeric() {
                id.push(cur_char);
                return Ok((StateResponse::CONTINUE, LexerStateDescriptor::NUMERIC, None))
            }
            else if cur_char == '.' {
                id.push(cur_char);
                return Ok((StateResponse::CONTINUE, LexerStateDescriptor::NUMERIC_DOT, None))
            }
            else {
                let full_id: String = id.iter().collect();
                id.clear();
                if let Ok(new_number) = full_id.parse::<f64>() {
                    let resp = LexerToken::from_number(new_number);
                    return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                }
                else {
                    return Err("Could not construct integer value");
                }
            }
        }
        LexerStateDescriptor::NUMERIC_DOT => {
            // A number literal with a period. Another number must
            // follow. This will be a float.
            if cur_char.is_numeric() {
                return Ok((StateResponse::CONTINUE, LexerStateDescriptor::NUMERIC_FLOAT, None))
            }
            else {
                return Err("Error lexing a floating point constant, expected numbers after '.'");
            }
        }
        LexerStateDescriptor::NUMERIC_FLOAT => {
            // A full float literal.
            if cur_char.is_numeric() {
                id.push(cur_char);
                return Ok((StateResponse::CONTINUE, LexerStateDescriptor::NUMERIC_FLOAT, None))
            }
            else {
                let full_id: String = id.iter().collect();
                id.clear();
                if let Ok(new_number) = full_id.parse::<f64>() {
                    let resp = LexerToken::from_number(new_number);
                    return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                }
                else {
                    return Err("Could not construct float value");
                }
            }
        }
        LexerStateDescriptor::EQ => {
            // An equal sign. Could be assignment or equality
            match cur_char {
                '=' => {
                    // Equality comparitor
                    let resp = LexerToken::from_rel_op(RelOp::EQUAL);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                '<' => {
                    // LTE comparator
                    let resp = LexerToken::from_rel_op(RelOp::LESS_THAN_EQ);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                '>' => {
                    // GTE comparator
                    let resp = LexerToken::from_rel_op(RelOp::GREATER_THAN_EQ);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                _ => {
                    // Assignment token.
                    // Backtrace and continue
                    let resp = LexerToken::from_single(TokenType::ASSIGN);

                    return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                }
            }
        }
        LexerStateDescriptor::GT => {
            // GT sign. Could be followed by an equal.
            match cur_char {
                '=' => {
                    // GTE comparator
                    let resp = LexerToken::from_rel_op(RelOp::GREATER_THAN_EQ);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                _ => {
                    // GT comparator
                    // Backtrace and continue
                    let resp = LexerToken::from_rel_op(RelOp::LESS_THAN);

                    return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                }
            }
        }
        LexerStateDescriptor::LT => {
            // LT sign. Could be followed by an equal.
            match cur_char {
                '=' => {
                    // LTE comparator
                    let resp = LexerToken::from_rel_op(RelOp::LESS_THAN_EQ);

                    return Ok((StateResponse::CONTINUE, LexerStateDescriptor::START, Some(resp)))
                }
                _ => {
                    // LT comparator
                    // Backtrace and continue
                    let resp = LexerToken::from_rel_op(RelOp::LESS_THAN);

                    return Ok((StateResponse::BACKTRACE, LexerStateDescriptor::START, Some(resp)))
                }
            }
        }
    }
    Err("Hahahaha whoops")
}

pub fn lex_string(lex_string: String) {
    let mut state = LexerState {
        state: LexerStateDescriptor::START,
        itt: lex_string.chars(),
        latest: None,
        position: 0,
        backtrace: false
    };

    let mut cur_str: Vec<char>;
    let mut response: StateResponse = StateResponse::CONTINUE;

    while response != StateResponse::DONE {

    }
}