mod function;
mod id;
mod variables;
mod statements;

use std::vec::Vec;

use ast::lexer;
use ast::lexer::tokens;
use self::statements::Statement;

pub trait Expression {}

pub type Expressions = Vec<Box<Expression>>;

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

impl Expression {
    pub fn from_lexer(lexer: &mut lexer::Lexer) -> Option<Box<Expression>> {
        let expression: Box<Expression> = match lexer[0].clone().token {
            tokens::TokenType::Keyword(ref keyword) => {
                match keyword {
                    &tokens::Keyword::COLONS => Box::new(Statement::Break),
                    &tokens::Keyword::FUNCTION => {
                        Box::new(function::Function::from_lexer(lexer.skip(1)))
                    }
                    _ => panic!("Unexpected keyword: {:?}", keyword),
                }
            }
            tokens::TokenType::Id(ref string) => {
                panic!("Unexpected Id: {}", string);
            }
            tokens::TokenType::String(ref string) => {
                panic!("Unexpected Id: {}", string);
            }
            tokens::TokenType::Number(ref string) => {
                panic!("Unexpected Id: {}", string);
            }
        };

        return Some(expression);
    }
}

pub struct Stub;

impl Expression for Stub {}