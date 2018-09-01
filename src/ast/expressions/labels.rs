use super::expression;
use ast::stack;

#[derive(Debug)]
pub struct Label(pub Box<expression::Expression>);
impl expression::Expression for Label {}

impl Label {
    pub fn new(stack: &mut stack::Stack) {
        let (_, name, _) = stack_unpack!(stack, single, single, single);
        stack.push_single(Box::new(Label(name)))
    }
}

#[derive(Debug)]
pub struct Goto(pub Box<expression::Expression>);
impl expression::Expression for Goto {}
