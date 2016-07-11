use std::collections::HashMap;
use std::string;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    AND,
    BREAK,
    DO,
    ELSE,
    ELSEIF,
    END,
    FALSE,
    FOR,
    FUNCTION,
    GOTO,
    IF,
    IN,
    LOCAL,
    NIL,
    NOT,
    OR,
    REPEAT,
    RETURN,
    THEN,
    TRUE,
    UNTIL,
    WHILE,

    // +     -     *     /     %     ^     #
    // &     ~     |     <<    >>    //
    // ==    ~=    <=    >=    <     >     =
    // (     )     {     }     [     ]     ::
    // ;     :     ,     .     ..    ...
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    POW,
    HASH,
    SAND,
    TILDA,
    SOR,
    SHLEFT,
    SHRIGHT,
    EQ,
    NEQ,
    LEQ,
    GEQ,
    LESS,
    GREATER,
    ASSIGN,
    LBRACE,
    RBRACE,
    LCBRACKET,
    RCBRACKET,
    LSBRACKET,
    RSBRACKET,
    PATH,
    COLONS,
    SEMICOLONS,
    COMMA,
    DOT,
    DOT2,
    DOT3,
}

pub fn get_token_table() -> HashMap<String, Keyword> {
    return string_hash_map![("and", Keyword::AND),
                            ("break", Keyword::BREAK),
                            ("do", Keyword::DO),
                            ("else", Keyword::ELSE),
                            ("elseif", Keyword::ELSEIF),
                            ("end", Keyword::END),
                            ("false", Keyword::FALSE),
                            ("for", Keyword::FOR),
                            ("function", Keyword::FUNCTION),
                            ("goto", Keyword::GOTO),
                            ("if", Keyword::IF),
                            ("in", Keyword::IN),
                            ("local", Keyword::LOCAL),
                            ("nil", Keyword::NIL),
                            ("not", Keyword::NOT),
                            ("or", Keyword::OR),
                            ("repeat", Keyword::REPEAT),
                            ("return", Keyword::RETURN),
                            ("then", Keyword::THEN),
                            ("true", Keyword::TRUE),
                            ("until", Keyword::UNTIL),
                            ("while", Keyword::WHILE)];
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Keyword(Keyword),
    Id(string::String),
    String(string::String),
    Number(string::String),
}

#[derive(PartialEq, Debug)]
pub struct Token {
    token: TokenType,
    row: usize,
    column: usize,
}

impl Token {
    pub fn new(token: TokenType, row: usize, column: usize) -> Token {
        return Token {
            token: token,
            row: row,
            column: column,
        };
    }
}
