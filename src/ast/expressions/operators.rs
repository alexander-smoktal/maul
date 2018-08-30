use super::*;
use ast::parser;
use ast::lexer::tokens;

// binop ::= `+´  |  `-´  |  `*´  |  `/´  |  `^´  |  `%´  |  `..´  |
//                 `<´  |  `<=´  |  `>´  |  `>=´  |  `==´  |  `~=´  |
//                 and  |  or
#[derive(Debug)]
pub struct Binop(
    pub tokens::Keyword,
    pub Box<expression::Expression>,
    pub Box<expression::Expression>
);
impl expression::Expression for Binop {}

// unop ::= `-´  |  not  |  `#´
#[derive(Debug)]
pub struct Unop(pub tokens::Keyword, pub Box<expression::Expression>);
impl expression::Expression for Unop {}

#[derive(Debug)]
pub struct Noop;
impl expression::Expression for Noop {}

impl Noop {
    make_keyword_rule![semi, (tokens::Keyword::SEMICOLONS, Noop)];
}
