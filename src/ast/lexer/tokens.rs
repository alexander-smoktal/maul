use std::collections::HashMap;

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

impl Keyword {
    // unop ::= ‘-’ | not | ‘#’ | ‘~’
    pub fn is_unop(&self) -> bool {
        match self.clone() {
            Keyword::MINUS
                | Keyword::NOT
                | Keyword::HASH
                | Keyword::TILDA => true,
            _ => false
        }
    }

    // binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ |
    //            ‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ |
    //            ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ |
    //            and | or
    pub fn is_binop(&self) -> bool {
        match self.clone() {
            Keyword::PLUS
                | Keyword::MINUS
                | Keyword::MUL
                | Keyword::DIV
                | Keyword::POW
                | Keyword::MOD
                | Keyword::SAND
                | Keyword::TILDA
                | Keyword::SOR
                | Keyword::SHRIGHT
                | Keyword::SHLEFT
                | Keyword::DOT2
                | Keyword::LESS
                | Keyword::LEQ
                | Keyword::GREATER
                | Keyword::GEQ
                | Keyword::EQ
                | Keyword::NEQ
                | Keyword::AND
                | Keyword::OR => true,
            _ => false
        }
    }
}

impl PartialEq<Token> for Keyword {
    fn eq(&self, token: &Token) -> bool {
        token.token == TokenType::from(self.clone())
    }
}

pub fn get_token_table() -> HashMap<String, Keyword> {
    string_hash_map![("and", Keyword::AND),
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
                     ("while", Keyword::WHILE)]
}

pub fn get_operator_table() -> HashMap<String, Keyword> {
    string_hash_map![("+", Keyword::PLUS),
                     ("-", Keyword::MINUS),
                     ("*", Keyword::MUL),
                     ("/", Keyword::DIV),
                     ("%", Keyword::MOD),
                     ("^", Keyword::POW),
                     ("#", Keyword::HASH),
                     ("&", Keyword::SAND),
                     ("~", Keyword::TILDA),
                     ("|", Keyword::SOR),
                     ("<<", Keyword::SHLEFT),
                     (">>", Keyword::SHRIGHT),
                     ("==", Keyword::EQ),
                     ("~=", Keyword::NEQ),
                     ("<=", Keyword::LEQ),
                     (">=", Keyword::GEQ),
                     ("<", Keyword::LESS),
                     (">", Keyword::GREATER),
                     ("=", Keyword::ASSIGN),
                     ("(", Keyword::LBRACE),
                     (")", Keyword::RBRACE),
                     ("[", Keyword::LSBRACKET),
                     ("]", Keyword::RSBRACKET),
                     ("{", Keyword::LCBRACKET),
                     ("}", Keyword::RCBRACKET),
                     ("::", Keyword::PATH),
                     (";", Keyword::SEMICOLONS),
                     (":", Keyword::COLONS),
                     (",", Keyword::COMMA),
                     (".", Keyword::DOT),
                     ("..", Keyword::DOT2),
                     ("...", Keyword::DOT3)]
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Keyword(Keyword),
    Id(String),
    String(String),
    Number(f64),
    None
}

impl From<Keyword> for TokenType {
    fn from(keyword: Keyword) -> Self {
        TokenType::Keyword(keyword)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub row: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token: TokenType, row: usize, column: usize) -> Token {
        return Token {
            token: token,
            row: row,
            column: column,
        };
    }

    pub fn eof() -> Token {
        Token::new(TokenType::None, 0, 0)
    }

    pub fn id(&self) -> Option<String> {
        match self.token {
            TokenType::Id(ref id) => Some(id.clone()),
            _ => None
        }
    }

    pub fn keyword(&self) -> Option<Keyword> {
        match self.token {
            TokenType::Keyword(ref keyword) => Some(keyword.clone()),
            _ => None
        }
    }
}

impl Into<TokenType> for Token {
    fn into(self) -> TokenType {
        self.token.clone()
    }
}


