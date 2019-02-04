enum LexerState {
    START,
    IDENTIFIER,
    NUMERIC,
    NUMERIC_DOT,
    NUMERIC_FLOAT,
    GT,
    LT,
    EQ
}

enum TokenTypes {
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