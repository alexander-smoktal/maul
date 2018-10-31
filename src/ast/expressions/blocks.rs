use std::collections::VecDeque;

use ast::expressions;
use ast::expressions::variables;
use ast::stack;

use interpreter;

#[derive(Debug)]
pub struct Block {
    pub statements: VecDeque<Box<expressions::Expression>>,
    pub retstat: Option<Box<expressions::Expression>>,
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
impl interpreter::Eval for WhileBlock {}
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
    pub block: Box<expressions::Expression>,
    pub condition: Box<expressions::Expression>,
}
impl interpreter::Eval for RepeatBlock {}
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
    pub condition: Box<expressions::Expression>,
    pub block: Box<expressions::Expression>,
}
impl interpreter::Eval for IfCondition {}
impl expressions::Expression for IfCondition {}

impl IfCondition {
    // {elseif exp then block}
    pub fn new_elseif(stack: &mut stack::Stack) {
        let (block, _then, condition, _elseif) =
            stack_unpack!(stack, single, single, single, single);

        stack.push_single(Box::new(IfCondition {
            condition,
            block
        }))
    }
}

#[derive(Debug)]
pub struct IfBlock {
    pub conditions: VecDeque<Box<expressions::Expression>>,
    pub else_block: Option<Box<expressions::Expression>>,
}
impl interpreter::Eval for IfBlock {}
impl expressions::Expression for IfBlock {}

impl IfBlock {
    // if exp then block {elseif exp then block} [else block] end |
    pub fn new(stack: &mut stack::Stack) {
        let (_end, else_block, mut conditions, block, _then, condition, _if) =
            stack_unpack!(stack, single, optional, repetition, single, single, single, single);

            let primary_condition = Box::new(IfCondition {
                condition,
                block
            });

            conditions.push_front(primary_condition);

            stack.push_single(Box::new(IfBlock {
                conditions,
                else_block
            }))
    }
}

#[derive(Debug)]
pub struct NumericalForBlock {
    pub init: Box<expressions::Expression>,
    pub limit: Box<expressions::Expression>,
    pub step: Option<Box<expressions::Expression>>,
    pub block: Box<expressions::Expression>,
}
impl interpreter::Eval for NumericalForBlock {}
impl expressions::Expression for NumericalForBlock {}

impl NumericalForBlock {
    // for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
    pub fn new(stack: &mut stack::Stack) {
        let (_end, block, _do, step, limit, _comma, init_value, _assign, init_name, _for)
            = stack_unpack!(stack, single, single, single, optional, single, single, single, single, single, single);

        stack.push_single(Box::new(NumericalForBlock {
            init: Box::new(variables::Assignment {
                varlist: VecDeque::from(vec![init_name]),
                explist: VecDeque::from(vec![init_value])
            }),
            limit,
            step,
            block
        }))
    }
}

#[derive(Debug)]
pub struct GenericForBlock {
    pub namelist: VecDeque<Box<expressions::Expression>>,
    pub explist: VecDeque<Box<expressions::Expression>>,
    pub block: Box<expressions::Expression>
}
impl interpreter::Eval for GenericForBlock {}
impl expressions::Expression for GenericForBlock {}

impl GenericForBlock {
    // for namelist in explist do block end |
    pub fn new(stack: &mut stack::Stack) {
        let (_end, block, _do, explist, _in, namelist, _for)
            = stack_unpack!(stack, single, single, single, repetition, single, repetition, single);

        stack.push_single(Box::new(GenericForBlock {
            namelist,
            explist,
            block
        }))
    }
}

#[derive(Debug)]
pub struct Local(Box<expressions::Expression>);
impl interpreter::Eval for Local {}
impl expressions::Expression for Local {}

impl Local {
    pub fn new(stack: &mut stack::Stack) {
        let (expression, _local) = stack_unpack!(stack, single, single);

        stack.push_single(Box::new(Local(expression)))
    }
}