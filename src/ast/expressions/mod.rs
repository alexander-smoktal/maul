use crate::ast::lexer::tokens;

#[macro_use]
pub mod utils;
pub mod blocks;
pub mod expression;
pub mod function;
pub mod labels;
pub mod operators;
pub mod primitives;
pub mod statements;
pub mod tables;
pub mod variables;

use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;

use crate::interpreter;

pub trait Expression: Debug + interpreter::Eval {
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
