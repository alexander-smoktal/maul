pub mod lexer;
#[macro_use]
pub mod stack;
#[macro_use]
pub mod parser;
pub mod expressions;

use std::fmt::{Debug, Formatter, Error};
use std::collections::VecDeque;

pub struct AST {
    expressions: VecDeque<Box<expressions::Expression>>,
}

impl AST {
    pub fn new(source_code: String) -> Self {
        let mut parser = parser::Parser::new(source_code);
        let mut stack = stack::Stack::new();

        parser::rules::exp(&mut parser, &mut stack);

        AST {
            expressions: stack.pop_repetition()
        }
    }
}

impl Debug for AST {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "{:?}", self.expressions)
    }
}
