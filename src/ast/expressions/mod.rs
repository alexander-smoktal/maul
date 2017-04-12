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
    StringConstant(String),
    NumberConstant(f32),
    BooleanConstant(bool)
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
    lexer.try_to_parse(assignment::parse_var)
        .or_else(|_| lexer.try_to_parse(function::parse_funcall))
        .or(Err(error::Error::new(lexer.head_or_eof(), "Failed to parse prefix expression")))
}

pub fn parse_exp(lexer: &mut lexer::Lexer) -> Result<Expression, error::Error> {
    Result::Ok(Expression::Stub)
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
            Some(tokens::TokenType::Id(ref string)) => {
                assignment::parse_var(lexer).ok()
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
