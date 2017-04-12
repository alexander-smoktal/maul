pub mod function;
pub mod variables;
pub mod statements;

use std::vec::Vec;

use error;
use ast::lexer;
use ast::lexer::tokens;

use self::statements::Statement;

#[derive(PartialEq, Debug)]
pub enum Expression {
    Stub,
    Id(String),
    Assignment {
        varname: variables::Id,
        expression: Box<Expression>,
    },
    Function {
        params: variables::Id,
        body: Expressions,
    },
    Indexing {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    St(statements::Statement),
    String(String),
    Number(f32),
    Boolean(bool),
    Nil
}

pub type Expressions = Vec<Expression>;

// chunk ::= block

// block ::= {stat} [retstat]

// stat ::=  ‘;’ |
//     varlist ‘=’ explist |
//     functioncall |
//     label |
//     break |
//     goto Name |
//     do block end |
//     while exp do block end |
//     repeat block until exp |
//     if exp then block {elseif exp then block} [else block] end |
//     for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
//     for namelist in explist do block end |
//     function funcname funcbody |
//     local function Name funcbody |
//     local namelist [‘=’ explist]


// prefixexp ::= var | functioncall | ‘(’ exp ‘)’
pub fn parse_prefixexp(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    lexer.try_to_parse(variables::parse_var)
        .or_else(|_| lexer.try_to_parse(function::parse_funcall))
        .or(Err(error::Error::new(lexer.head_or_eof(), "Failed to parse prefix expression")))
}
// exp ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef |
//          prefixexp | tableconstructor | exp binop exp | unop exp
pub fn parse_exp(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    if let Some(token) = lexer.head().cloned() {
        match token.token {
            tokens::TokenType::Keyword(tokens::Keyword::NIL) => Ok(Expression::Nil),
            tokens::TokenType::Keyword(tokens::Keyword::FALSE) => Ok(Expression::Boolean(false)),
            tokens::TokenType::Keyword(tokens::Keyword::TRUE) => Ok(Expression::Boolean(true)),
            tokens::TokenType::Keyword(tokens::Keyword::DOT3) => Ok(Expression::St(statements::Statement::Ellipsis)),
            tokens::TokenType::Number(number) => Ok(Expression::Number(number.parse::<f32>().unwrap())),
            tokens::TokenType::String(string) => Ok(Expression::String(string)),
            _ => Err(error::Error::new(lexer.head_or_eof(), "Invalid expression"))
        }
    } else {
        return Err(error::Error::new(lexer.head_or_eof(), "Expected expression"))
    }
}

impl Expression {
    pub fn from_lexer(lexer: &mut lexer::Lexer) -> Option<Expression> {
        match lexer.head().cloned().map(|x: tokens::Token| x.token) {
            Some(tokens::TokenType::Keyword(ref keyword)) => {

                match keyword {
                    &tokens::Keyword::COLONS => Some(Expression::St(Statement::Break)),
                    &tokens::Keyword::FUNCTION => {
                        function::parse_funcdef(lexer.skip(1)).ok()
                    }
                    _ => panic!("Unexpected keyword: {:?}", keyword),
                }
            },
            Some(tokens::TokenType::Id(_)) => {
                variables::parse_var(lexer).ok()
            },
            Some(tokens::TokenType::String(ref string)) => {
                panic!("Unexpected Id: {}", string);
            },
            Some(tokens::TokenType::Number(ref string)) => {
                panic!("Unexpected Id: {}", string);
            },
            _ => panic!("Unexpected Id")
        }
    }
}
