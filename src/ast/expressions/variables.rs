use std::collections::VecDeque;

use ast::expressions;
use ast::lexer::tokens::{self, Keyword};
use ast::parser;
use ast::stack;
use ast::rules;

#[derive(Debug, Clone)]
pub struct Id(pub String);
impl expressions::Expression for Id {}

impl Id {
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if let Some(tokens::Token {
            token: tokens::TokenType::Id(string),
            ..
        }) = parser.peek().cloned()
        {
            parser.shift();
            stack.push_single(Box::new(Id(string)));
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
}

#[derive(Debug)]
pub struct Assignment {
    pub varlist: VecDeque<Box<expressions::Expression>>,
    pub explist: VecDeque<Box<expressions::Expression>>,
}
impl expressions::Expression for Assignment {}

impl Assignment {
    pub fn new(stack: &mut stack::Stack) {
        let (explist, _assignment, varlist) = stack_unpack!(stack, repetition, single, repetition);

        stack.push_single(Box::new(Assignment {
            varlist,
            explist
        }))
    }

    pub fn rule_local(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if let Some(Keyword::ASSIGN) = parser.peek().and_then(|token| token.keyword()) {
            // If we have assignment
            parser.shift();

            if !rules::explist(parser, stack) {
                return false;
            }

            let (explist, varlist) = stack_unpack!(stack, repetition, repetition);

            stack.push_single(Box::new(Assignment {
                varlist,
                explist
            }))
        } else {
            // No assignment. Just namelist
            let varlist = stack.pop_repetition();

            stack.push_single(Box::new(Assignment {
                varlist,
                explist: VecDeque::new()
            }))
        }

        true
    }
}
