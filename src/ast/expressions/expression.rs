use std::fmt::Debug;
use std::cmp::{PartialEq, Eq};

use super::*;

pub trait Expression: Debug {
    fn into_expressions(self: Box<Self>) -> Box<common::Expressions> {
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

pub fn from_parser(parser: &mut parser::Parser) -> Option<Box<Expression>> {
    expr_rule(parser)
}
