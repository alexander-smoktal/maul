use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::expressions;
use crate::ast::lexer::tokens::{self, Keyword};
use crate::ast::parser;
use crate::ast::rules;
use crate::ast::stack;

use crate::interpreter::{types, environment, cache};

/// Id contains cached value for an environment.
/// During evalueation, we first check if we have value in cache and if no , we set it after evaluation.
#[derive(Clone)]
pub struct Id {
    pub id: String,
    /// We need interior mutability to update cache
    pub cache: RefCell<cache::Cache>
}
impl expressions::Expression for Id {}

impl Id {
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if let Some(tokens::Token {
            token: tokens::TokenType::Id(id),
            ..
        }) = parser.peek().cloned()
        {
            parser.shift();
            stack.push_single(Box::new(Id {id, cache: RefCell::new(cache::Cache::default())}));
            true
        } else {
            false
        }
    }

    pub fn rule_string_id(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if let Some(tokens::Token {
            token: tokens::TokenType::Id(string),
            ..
        }) = parser.peek().cloned()
        {
            parser.shift();
            stack.push_single(Box::new(expressions::primitives::String(string)));
            true
        } else {
            false
        }
    }

    pub fn get_cached(&self, env: &environment::Environment) -> Option<types::Type> {
        self.cache.borrow().get(env.id())
    }

    pub fn set_cached(&self, env: &environment::Environment, value: &Rc<RefCell<types::Type>>) {
        self.cache.borrow_mut().set(env.id(), value)
    }
}

impl ::std::fmt::Debug for Id {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Id({:?})", self.id)
    }
}

#[derive(Debug)]
pub struct Assignment {
    pub varlist: VecDeque<Box<dyn expressions::Expression>>,
    pub explist: VecDeque<Box<dyn expressions::Expression>>,
}
impl expressions::Expression for Assignment {}

impl Assignment {
    pub fn new(stack: &mut stack::Stack) {
        let (explist, _assignment, varlist) = stack_unpack!(stack, repetition, single, repetition);

        stack.push_single(Box::new(Assignment { varlist, explist }))
    }

    pub fn rule_local(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if let Some(Keyword::ASSIGN) = parser.peek().and_then(|token| token.keyword()) {
            // If we have assignment
            parser.shift();

            if !rules::explist(parser, stack) {
                return false;
            }

            let (explist, varlist) = stack_unpack!(stack, repetition, repetition);

            stack.push_single(Box::new(Assignment { varlist, explist }))
        } else {
            // No assignment. Just namelist
            let varlist = stack.pop_repetition();

            stack.push_single(Box::new(Assignment {
                varlist,
                explist: VecDeque::new(),
            }))
        }

        true
    }
}
