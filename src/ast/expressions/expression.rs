pub use std::ops;
use std::collections::VecDeque;

use ast::stack;

// TODO: Remove this
pub use super::Expression as Expression;

#[derive(Debug)]
pub struct Expressions(VecDeque<Box<Expression>>);
impl Expression for Expressions {}

impl Expressions {
    pub fn new(stack: &mut stack::Stack) {
        let (mut tail, head) = stack_unpack!(stack, repetition, single);
        tail.push_front(head);

        stack.push_single(Box::new(Expressions(tail)));
    }
}
