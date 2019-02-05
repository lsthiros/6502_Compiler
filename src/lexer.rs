use std::option;

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
    type: TokenType,
    label: Option<String>,
    number: Option<f64>,
    relType: Option<RelOp>,
}

struct LexerState {
    state: LexerState,
}

fn lexString(lexString: String) {
    let mut state: LexerStateDescriptor = START;

}