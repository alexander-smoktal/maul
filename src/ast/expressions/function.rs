use std::collections::VecDeque;

use ast::expressions;
use ast::stack;

#[derive(Debug)]
pub struct Funcname {
    names: VecDeque<Box<expressions::Expression>>,
    /// If pass self pointer as first argument
    this: bool
}
impl expressions::Expression for Funcname {}

impl Funcname {
    pub fn new(stack: &mut stack::Stack) {
        let (method_name, mut names) = stack_unpack!(stack, optional, repetition);

        match method_name {
            Some(method) => {
                names.push_back(method);

                stack.push_single(Box::new(Funcname {
                    names,
                    this: true
                }))
            },
            _ => {
                stack.push_single(Box::new(Funcname {
                    names,
                    this: false
                }))
            }
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Box<expressions::Expression>,
}
impl expressions::Expression for Function {}

#[derive(Debug)]
pub struct Funcall {
    pub function: Box<expressions::Expression>,
    pub args: Box<expressions::Expression>,
}
impl expressions::Expression for Funcall {}
