/*mod utils;
pub mod function;
pub mod variables;
pub mod tables;
pub mod statements;
pub mod labels;
pub mod primitives;
pub mod operators;*/

pub mod blocks;
pub mod expression;

use std::vec::Vec;

use error;
use ast::lexer::tokens;

const DEBUG: bool = false;

use ast::parser;

pub type ParseResult = Result<Box<expression::Expression>, error::Error>;

pub mod common {
    use super::expression;

    #[derive(Debug)]
    pub struct Noop;
    impl expression::Expression for Noop {}

    #[derive(Debug)]
    pub struct Expressions(pub Vec<Box<expression::Expression>>);

    impl Expressions {
        pub fn prepend(&mut self, exp: Box<expression::Expression>) {
            let mut new_expressions = vec![exp];
            new_expressions.append(&mut self.0);

            self.0 = new_expressions
        }

        pub fn append(&mut self, exp: Box<expression::Expression>) {
            self.0.push(exp)
        }
    }

    impl expression::Expression for Expressions {
        fn into_expressions(self: Box<Self>) -> Box<Expressions> {
            self as Box<Expressions>
        }
    }
}

rule!(expr_rule, parser::keyword(tokens::Keyword::AND));

// prefixexp ::= var | functioncall | ‘(’ exp ‘)’

// exp ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef |
//          prefixexp | tableconstructor | exp binop exp | unop exp


// explist ::= exp {‘,’ exp}

