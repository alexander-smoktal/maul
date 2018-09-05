use ast::expressions;
use ast::stack;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Funcname {
    pub object: VecDeque<Box<expressions::Expression>>,
    pub method: Option<Box<expressions::Expression>>,
}
impl expressions::Expression for Funcname {}

impl Funcname {
    pub fn new(stack: &mut stack::Stack) {
        let (method, object) = stack_unpack!(stack, optional, repetition);

        stack.push_single(Box::new(Funcname { object, method }))
    }
}

#[derive(Debug)]
pub struct Closure {
    params: Option<Box<expressions::Expression>>,
    body: Box<expressions::Expression>,
}
impl expressions::Expression for Closure {}

impl Closure {
    // ‘(’ [parlist] ‘)’ block end
    pub fn new(stack: &mut stack::Stack) {
        // We've remove braces before
        let (_end, body, params) = stack_unpack!(stack, single, single, optional);

        stack.push_single(Box::new(Closure { params, body }))
    }
}

#[derive(Debug)]
pub struct FunctionParameters {
    pub namelist: Option<VecDeque<Box<expressions::Expression>>>,
    pub varargs: bool,
}
impl expressions::Expression for FunctionParameters {}
impl FunctionParameters {
    /// Each new name in parameters list will append itself to the parameters list
    pub fn new_name(stack: &mut stack::Stack) {
        let (name, _comma, mut namelist) = stack_unpack!(stack, single, single, repetition);

        namelist.push_back(name);
        stack.push_repetition(namelist);
    }

    /// This is ellipsis after varargs. Accoridng to grammar we can get here in two ways:
    /// - after namelist;
    /// - after another ellipsis.
    /// Second one is invalid. So we check if we have repetitions on top. And later construct parameters itself.
    pub fn new_namelist_varargs(stack: &mut stack::Stack) {
        // Pop ellipsis and comma
        let (_ellipsis, _comma) = stack_unpack!(stack, single, single);

        match stack.peek() {
            // This is valid case. We have varargs after namelist
            stack::Element::Repetition(_) => {
                let namelist = stack.pop_repetition();

                stack.push_single(Box::new(FunctionParameters {
                    namelist: Some(namelist),
                    varargs: true,
                }))
            }
            // Already had ellipsis after namelist. Invalid syntax
            _ => panic!(
                "Invalid syntax. Expected ')' to close namelist. Got {:?}",
                stack.peek()
            ),
        }
    }

    /// Final namelist function. We either had namelist or namelist followed by ellipsis.VecDeque
    /// In case of ellipsis, we already have proper expression on stack. In case we have namelist on stack,
    /// we create new function parameters object.
    pub fn new_namelist(stack: &mut stack::Stack) {
        match stack.peek() {
            // This is in case we had only namelist
            stack::Element::Repetition(_) => {
                let namelist = stack.pop_repetition();

                stack.push_single(Box::new(FunctionParameters {
                    namelist: Some(namelist),
                    varargs: false,
                }))
            }
            // Already had ellipsis after namelist, which properly created parameters. Ignore
            _ => {}
        }
    }

    pub fn new_single_varargs(stack: &mut stack::Stack) {
        // Ellipsis
        stack.pop_single();
        stack.push_single(Box::new(FunctionParameters {
            namelist: None,
            varargs: true,
        }))
    }
}

#[derive(Debug)]
pub struct Funcall {
    pub object: Box<expressions::Expression>,
    pub args: VecDeque<Box<expressions::Expression>>,
    pub method: Option<Box<expressions::Expression>>,
}
impl expressions::Expression for Funcall {}

impl Funcall {
    pub fn new(stack: &mut stack::Stack) {
        let (args, object) = stack_unpack!(stack, repetition, single);

        stack.push_single(Box::new(Funcall {
            object,
            args,
            method: None,
        }))
    }

    pub fn new_self(stack: &mut stack::Stack) {
        let (args, method, _colon, object) =
            stack_unpack!(stack, repetition, single, single, single);

        stack.push_single(Box::new(Funcall {
            object,
            args,
            method: Some(method),
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
