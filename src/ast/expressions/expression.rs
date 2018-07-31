use std::fmt::Debug;
use std::cmp::{PartialEq, Eq};
use super::primitives;
use ast::parser;
use super::utils;
pub use std::ops;

use super::*;

pub trait Expression: Debug {
    fn into_expressions(self: Box<Self>) -> Box<Expressions> {
        panic!("Found conversion to expressions list for invalid type")
    }

    fn clone(&self) -> Box<Expression> {
        panic!("Trying to clone expression, which can't be cloned")
    }
}

impl PartialEq for Box<Expression> {
    fn eq(&self, other: &Box<Expression>) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl Eq for Box<Expression> {}

// prefixexp ::= var | functioncall | ‘(’ exp ‘)’

pub type ParseResult = Result<Box<expression::Expression>, error::Error>;

#[derive(Debug)]
pub struct Expressions(pub Vec<Box<expression::Expression>>);

impl Expressions {
    fn new(left: Box<Expression>, _comma: Box<Expression>, right: Box<Expression>) -> Option<Box<Expression>> {
        utils::some_expression(Expressions(vec![left, right]))
    }

    // explist ::= exp {‘,’ exp}
    rule!(rule, or![
        and![(Expressions::exp, utils::terminal(tokens::Keyword::COMMA), Expressions::rule) => Expressions::new],
        Expressions::exp
    ]);

    // exp ::=  nil | false | true | Numeral | LiteralString | ‘...’ | functiondef |
    //          prefixexp | tableconstructor | exp binop exp | unop exp
    rule!(exp, or![
        primitives::Nil::rule,
        primitives::Boolean::rule,
        primitives::Number::rule,
        primitives::String::rule
    ]);
}

impl expression::Expression for Expressions {
    fn into_expressions(self: Box<Self>) -> Box<Expressions> {
        self as Box<Expressions>
    }
}
