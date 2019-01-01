use crate::ast::expressions::{self, primitives, statements, tables, variables};
use crate::ast::stack;

use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct Closure {
    pub params: VecDeque<Box<expressions::Expression>>,
    pub varargs: bool,
    pub body: Rc<Box<expressions::Expression>>,
}
impl expressions::Expression for Closure {}

/// Closure expression.
/// Note: Our parameter parser already remove braces and in case we have no parameters, pushed empty
/// namelist and Nona as vargargs on top of stask, so we just need to pick them up.
impl Closure {
    // ‘(’ [parlist] ‘)’ block end
    pub fn new(stack: &mut stack::Stack) {
        // We've remove braces before
        let (_end, body, ellipsis, params, _function) =
            stack_unpack!(stack, single, single, optional, repetition, single);

        stack.push_single(Box::new(Closure {
            params,
            varargs: ellipsis.is_some(),
            body: Rc::new(body),
        }));
    }
}

#[derive(Debug)]
pub struct Function;

/// Function, which is basically an assignment of a closure to a name.
/// Also performs rearrangment of parameters and method name if it's a method.
impl Function {
    pub fn new(stack: &mut stack::Stack) {
        // This looks like a lot of terminals, but we need them to desugar method
        let (_end, body, ellipsis, mut params, methodname, mut object, _function) =
            stack_unpack!(stack, single, single, optional, repetition, optional, single, single);

        // Check if have method name. If so, we make another indexing and add `self` argument
        if let Some(method) = methodname {
            // Function name
            object = Box::new(tables::Indexing {
                object,
                index: method,
            });

            // Prepend `self` parameter to the parameters
            params.push_front(Box::new(primitives::String("self".to_string())));
        }

        let closure = Box::new(Closure {
            params,
            varargs: ellipsis.is_some(),
            body: Rc::new(body),
        }) as Box<expressions::Expression>;

        stack.push_single(Box::new(variables::Assignment {
            varlist: vec![object].into(),
            explist: vec![closure].into(),
        }));
    }
}

#[derive(Debug)]
pub struct FunctionParameters;

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

        let namelist = stack.pop_repetition();

        FunctionParameters::finalize_parameters_parsing(stack, namelist, true);
    }

    /// Final namelist function. We either had namelist or namelist followed by ellipsis.VecDeque
    /// In case of ellipsis, we already have proper expression on stack. In case we have namelist on stack,
    /// we create new function parameters object.
    pub fn new_namelist(stack: &mut stack::Stack) {
        match stack.peek() {
            // This is in case we had only namelist
            stack::Element::Repetition(_) => {
                let namelist = stack.pop_repetition();

                FunctionParameters::finalize_parameters_parsing(stack, namelist, false);
            }
            // Already had ellipsis after namelist, which properly created parameters. Ignore
            _ => {}
        }
    }

    pub fn new_single_varargs(stack: &mut stack::Stack) {
        // Ellipsis
        stack.pop_single();
        FunctionParameters::finalize_parameters_parsing(stack, VecDeque::new(), true);
    }

    /// Helper function to push parameters and indicator of varargs
    /// After method execution, stack will contains parameters list and optional ellipsis expression
    fn finalize_parameters_parsing(
        stack: &mut stack::Stack,
        namelist: VecDeque<Box<expressions::Expression>>,
        varargs: bool,
    ) {
        stack.push_repetition(namelist);
        stack.push_optional(if varargs {
            Some(Box::new(statements::Statement::Ellipsis))
        } else {
            None
        });
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
