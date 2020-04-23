use std::collections::VecDeque;
pub use std::ops;

use crate::ast::expressions;
use crate::ast::stack;

#[derive(Debug)]
pub struct Expressions(pub VecDeque<Box<dyn expressions::Expression>>);
impl expressions::Expression for Expressions {}

impl Expressions {
    pub fn new(stack: &mut stack::Stack) {
        let tail = stack.pop_repetition();

        stack.push_single(Box::new(Expressions(tail)));
    }
}
