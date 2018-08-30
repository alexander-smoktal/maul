use std::fmt::Debug;
use std::cmp::{PartialEq, Eq};
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
pub struct Expressions {
    head: Box<Expression>,
    tail: Option<Box<Expression>>
}
impl expression::Expression for Expressions {}

impl Expressions {
    pub fn new(head: Box<Expression>, _comma: Option<Box<Expression>>, tail: Option<Box<Expression>>) -> Option<Box<Expression>> {
        utils::some_expression(Expressions {
            head, 
            tail
        })
    }
}
