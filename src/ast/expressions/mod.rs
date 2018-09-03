use ast::lexer::tokens;

#[macro_use]
pub mod utils;
pub mod function;
pub mod blocks;
pub mod expression;
pub mod primitives;
pub mod operators;
pub mod statements;
pub mod labels;
pub mod variables;
pub mod tables;

use std::fmt::Debug;
use std::cmp::{PartialEq, Eq};

pub trait Expression: Debug {
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

// Struct for debugging. Wraps terminal. Basically Noop
#[derive(Debug)]
pub struct Terminal(pub tokens::Keyword);

impl Expression for Terminal {}
