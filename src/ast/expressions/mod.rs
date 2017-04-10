pub mod function;
pub mod assignment;
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
        varname: assignment::Id,
        expression: Box<Expression>,
    },
    Function {
        params: assignment::Id,
        body: Expressions,
    },
    Indexing {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    St(statements::Statement),
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
    Result::Ok(Expression::Stub)
}

pub fn parse_exp(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    Result::Ok(Expression::Stub)
}

impl Expression {
    pub fn from_lexer(lexer: &mut lexer::Lexer) -> Option<Expression> {
        let expression: Expression = match lexer.get(0).clone().token {
            tokens::TokenType::Keyword(ref keyword) => {
                match keyword {
                    &tokens::Keyword::COLONS => Expression::St(Statement::Break),
                    &tokens::Keyword::FUNCTION => function::parse_funcdef(lexer.skip(1)).unwrap(),
                    _ => panic!("Unexpected keyword: {:?}", keyword),
                }
            }
            tokens::TokenType::Id(ref string) => {
                assignment::parse_var(lexer).unwrap()
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
