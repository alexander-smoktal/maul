use std::collections::VecDeque;

use ast::expressions;
use ast::lexer::tokens;
use ast::parser;
use ast::stack;

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
}
