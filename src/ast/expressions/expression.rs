use std::collections::VecDeque;
pub use std::ops;

use ast::expressions;
use ast::stack;

#[derive(Debug)]
pub struct Expressions(VecDeque<Box<expressions::Expression>>);
impl expressions::Expression for Expressions {}

impl Expressions {
    pub fn new(stack: &mut stack::Stack) {
        let tail = stack.pop_repetition();

        stack.push_single(Box::new(Expressions(tail)));
    }
}
