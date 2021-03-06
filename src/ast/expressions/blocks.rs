use std::collections::VecDeque;

use crate::ast::expressions;
use crate::ast::stack;

use crate::interpreter;

#[derive(Debug)]
pub struct Block {
    pub statements: VecDeque<Box<dyn expressions::Expression>>,
    pub retstat: Option<Box<dyn expressions::Expression>>,
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
pub struct DoBlock(pub Box<dyn expressions::Expression>);
impl expressions::Expression for DoBlock {}

impl DoBlock {
    pub fn new(stack: &mut stack::Stack) {
        let (_end, block, _do) = stack_unpack!(stack, single, single, single);

        stack.push_single(Box::new(DoBlock(block)))
    }
}

#[derive(Debug)]
pub struct WhileBlock {
    pub condition: Box<dyn expressions::Expression>,
    pub block: Box<dyn expressions::Expression>,
}
impl expressions::Expression for WhileBlock {}

impl WhileBlock {
    pub fn new(stack: &mut stack::Stack) {
        let (_end, block, _do, condition, _while) =
            stack_unpack!(stack, single, single, single, single, single);

        stack.push_single(Box::new(WhileBlock { condition, block }))
    }
}

#[derive(Debug)]
pub struct RepeatBlock {
    pub block: Box<dyn expressions::Expression>,
    pub condition: Box<dyn expressions::Expression>,
}
impl expressions::Expression for RepeatBlock {}

impl RepeatBlock {
    pub fn new(stack: &mut stack::Stack) {
        let (condition, _until, block, _repeat) =
            stack_unpack!(stack, single, single, single, single);

        stack.push_single(Box::new(RepeatBlock { block, condition }))
    }
}

// TODO: Remove publicity
#[derive(Debug)]
pub struct IfCondition {
    pub condition: Box<dyn expressions::Expression>,
    pub block: Box<dyn expressions::Expression>,
}
impl expressions::Expression for IfCondition {}

impl IfCondition {
    // {elseif exp then block}
    pub fn new_elseif(stack: &mut stack::Stack) {
        let (block, _then, condition, _elseif) =
            stack_unpack!(stack, single, single, single, single);

        stack.push_single(Box::new(IfCondition { condition, block }))
    }
}

#[derive(Debug)]
pub struct IfBlock {
    pub conditions: VecDeque<Box<dyn expressions::Expression>>,
    pub else_block: Option<Box<dyn expressions::Expression>>,
}
impl expressions::Expression for IfBlock {}

impl IfBlock {
    // if exp then block {elseif exp then block} [else block] end |
    pub fn new(stack: &mut stack::Stack) {
        let (_end, else_block, mut conditions, block, _then, condition, _if) =
            stack_unpack!(stack, single, optional, repetition, single, single, single, single);

        let primary_condition = Box::new(IfCondition { condition, block });

        conditions.push_front(primary_condition);

        stack.push_single(Box::new(IfBlock {
            conditions,
            else_block,
        }))
    }
}

#[derive(Debug)]
pub struct NumericalForBlock {
    pub var_name: Box<dyn expressions::Expression>,
    pub init_value: Box<dyn expressions::Expression>,
    pub limit: Box<dyn expressions::Expression>,
    pub step: Option<Box<dyn expressions::Expression>>,
    pub block: Box<dyn expressions::Expression>,
}
impl expressions::Expression for NumericalForBlock {}

impl NumericalForBlock {
    // for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
    pub fn new(stack: &mut stack::Stack) {
        let (_end, block, _do, step, limit, _comma, init_value, _assign, var_name, _for) = stack_unpack!(
            stack, single, single, single, optional, single, single, single, single, single, single
        );

        stack.push_single(Box::new(NumericalForBlock {
            var_name,
            init_value,
            limit,
            step,
            block,
        }))
    }
}

#[derive(Debug)]
pub struct GenericForBlock {
    pub namelist: VecDeque<Box<dyn expressions::Expression>>,
    pub explist: VecDeque<Box<dyn expressions::Expression>>,
    pub block: Box<dyn expressions::Expression>,
}
impl interpreter::Eval for GenericForBlock {}
impl expressions::Expression for GenericForBlock {}

impl GenericForBlock {
    // for namelist in explist do block end |
    pub fn new(stack: &mut stack::Stack) {
        let (_end, block, _do, explist, _in, namelist, _for) =
            stack_unpack!(stack, single, single, single, repetition, single, repetition, single);

        stack.push_single(Box::new(GenericForBlock {
            namelist,
            explist,
            block,
        }))
    }
}

#[derive(Debug)]
pub struct Local(pub Box<dyn expressions::Expression>);
impl expressions::Expression for Local {}

impl Local {
    pub fn new(stack: &mut stack::Stack) {
        let (expression, _local) = stack_unpack!(stack, single, single);

        stack.push_single(Box::new(Local(expression)))
    }
}
