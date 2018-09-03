pub use std::ops;
use std::collections::VecDeque;

use ast::expressions;
use ast::stack;

#[derive(Debug)]
pub struct Expressions(VecDeque<Box<expressions::Expression>>);
impl expressions::Expression for Expressions {}

impl Expressions {
    pub fn new(stack: &mut stack::Stack) {
        let (mut tail, head) = stack_unpack!(stack, repetition, single);
        tail.push_front(head);

        stack.push_single(Box::new(Expressions(tail)));
    }
}
