use crate::ast::expressions;
use crate::ast::stack;

use crate::interpreter;

#[derive(Debug)]
pub struct Label(pub Box<expressions::Expression>);
impl interpreter::Eval for Label {}
impl expressions::Expression for Label {}

impl Label {
    pub fn new(stack: &mut stack::Stack) {
        let (_, name, _) = stack_unpack!(stack, single, single, single);
        stack.push_single(Box::new(Label(name)))
    }
}

#[derive(Debug)]
pub struct Goto(pub Box<expressions::Expression>);
impl interpreter::Eval for Goto {}
impl expressions::Expression for Goto {}

impl Goto {
    pub fn new(stack: &mut stack::Stack) {
        let (name, _goto) = stack_unpack!(stack, single, single);
        stack.push_single(Box::new(Goto(name)))
    }
}
