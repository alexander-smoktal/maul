pub mod lexer;
#[macro_use]
pub mod parser;
pub mod expressions;
#[macro_use]
pub mod stack;

use std::fmt::{Debug, Formatter, Error};
use std::collections::VecDeque;

pub struct AST {
    expressions: VecDeque<Box<expressions::Expression>>,
}

impl AST {
    pub fn new(source_code: String) -> Self {
        /*let parser = parser::Parser::new(source_code);
        let stack = stack::Stack::new();

        parser::rules::exp(&mut parser, &mut stack)*/

        AST {
            expressions: VecDeque::new()// stack.get_repetition()
        }
    }
}

impl Debug for AST {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "{:?}", self.expressions)
    }
}
