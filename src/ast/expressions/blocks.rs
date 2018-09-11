use std::collections::VecDeque;

use ast::expressions;
use ast::stack;

#[derive(Debug)]
pub struct Block {
    statements: VecDeque<Box<expressions::Expression>>,
    retstat: Option<Box<expressions::Expression>>,
}
impl expressions::Expression for Block {}
impl Block {
    pub fn new(stack: &mut stack::Stack) {
        let (retstat, statements) = stack_unpack!(stack, optional, repetition);

        stack.push_single(Box::new(Block {
            statements,
            retstat,
        }));
    }
}

#[derive(Debug)]
pub struct DoBlock(pub Box<expressions::Expression>);
impl expressions::Expression for DoBlock {}

impl DoBlock {
    pub fn new(stack: &mut stack::Stack) {
        let (_end, block, _do) = stack_unpack!(stack, single, single, single);

        stack.push_single(Box::new(DoBlock(block)))
    }
}

#[derive(Debug)]
pub struct WhileBlock {
    pub condition: Box<expressions::Expression>,
    pub block: Box<expressions::Expression>,
}
impl expressions::Expression for WhileBlock {}

impl WhileBlock {
    pub fn new(stack: &mut stack::Stack) {
        let (_end, block, _do, condition, _while) = stack_unpack!(stack, single, single, single, single, single);

        stack.push_single(Box::new(WhileBlock {
            condition,
            block
        }))
    }
}

#[derive(Debug)]
pub struct RepeatBlock {
    pub block: Box<expressions::Expression>,
    pub condition: Box<expressions::Expression>,
}
impl expressions::Expression for RepeatBlock {}

impl RepeatBlock {
    pub fn new(stack: &mut stack::Stack) {
        let (condition, _until, block, _repeat) = stack_unpack!(stack, single, single, single, single);

        stack.push_single(Box::new(RepeatBlock {
            block,
            condition
        }))
    }
}

// We could make typedef for 'while' and 'repeat', but can't implement trait for type
#[derive(Debug)]
pub struct Condition {
    pub condition: Box<expressions::Expression>,
    pub block: Box<expressions::Expression>,
}

#[derive(Debug)]
pub struct IfBlock {
    pub conditions: Vec<Condition>,
    pub elseblock: Option<Box<expressions::Expression>>,
}
impl expressions::Expression for IfBlock {}
