pub mod lexer;
pub mod parser;
pub mod expressions;

use std::fmt::{Debug, Formatter, Error};

pub struct AST {
    expressions: Box<expressions::expression::Expression>
}

impl AST {
    pub fn new(source_code: String) -> Self {
        parser::Parser::new(source_code)
    }

    fn add_expression(&mut self, exp: Box<expressions::expression::Expression>) {
        self.expressions = exp;
    }
}

impl Debug for AST {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "{:?}", self.expressions)
    }
}
