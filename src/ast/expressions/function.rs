use std::collections::VecDeque;

use ast::expressions;
use ast::stack;

#[derive(Debug)]
pub struct Funcname {
    names: VecDeque<Box<expressions::Expression>>,
    /// If pass self pointer as first argument
    this: bool // TODO: Rework to have optional method name
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
    pub args: VecDeque<Box<expressions::Expression>>,
    pub method: Option<Box<expressions::Expression>>
}
impl expressions::Expression for Funcall {}

impl Funcall {
    pub fn new(stack: &mut stack::Stack) {
        let (args, function) = stack_unpack!(stack, repetition, single);

        stack.push_single(Box::new(Funcall {
            function,
            args,
            method: None
        }))
    }

    pub fn new_self(stack: &mut stack::Stack) {
         let (args, method, _colon, function) = stack_unpack!(stack, repetition, single, single, single);

         stack.push_single(Box::new(Funcall {
            function,
            args,
            method: Some(method)
        }))
    }

    pub fn new_args(stack: &mut stack::Stack) {
        let _rbrace = stack.pop_single();
        
        if let stack::Element::Repetition(_) = stack.peek() {
            // Had some args
            let (arguments, _lbrace) = stack_unpack!(stack, repetition, single);
            stack.push_repetition(arguments);
        } else {
            // No args. Push empty vec
            let _lbrace = stack.pop_single();
            stack.push_repetition(VecDeque::new());
        }
    }
}
