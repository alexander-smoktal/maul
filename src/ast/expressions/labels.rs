use ast::expressions;
use ast::stack;

#[derive(Debug)]
pub struct Label(pub Box<expressions::Expression>);
impl expressions::Expression for Label {}

impl Label {
    pub fn new(stack: &mut stack::Stack) {
        let (_, name, _) = stack_unpack!(stack, single, single, single);
        stack.push_single(Box::new(Label(name)))
    }
}

#[derive(Debug)]
pub struct Goto(pub Box<expressions::Expression>);
impl expressions::Expression for Goto {}
