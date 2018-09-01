use std::collections::VecDeque;
use std::iter::{IntoIterator, FromIterator};

use ast::expressions;

const DEBUG: bool = true;

#[derive(Debug)]
pub enum Element {
    Single(Box<expressions::Expression>),
    Repetition(VecDeque<Box<expressions::Expression>>),
    Optional(Option<Box<expressions::Expression>>)
}

#[derive(Debug)]
pub struct Stack {
    elements: Vec<Element>
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            elements: vec![]
        }
    }

    pub fn peek(&self) -> &Element {
        self.elements.last().unwrap()
    }

    pub fn pop_single(&mut self) -> Box<expressions::Expression> {
        if self.elements.is_empty() {
            panic!("Invalid grammar. No elements on stack");
        }

        match self.elements.pop().unwrap() {
            Element::Single(expression) => {
                debug_parser!("Stack pop: {:?}", expression);
                expression
            },
            element => panic!(format!("Expected single element on stack. Got {:?}", element))
        }
    }

    pub fn pop_repetition(&mut self) -> VecDeque<Box<expressions::Expression>> {
        if self.elements.is_empty() {
            panic!("Invalid grammar. No elements on stack");
        }

        match self.elements.pop().unwrap() {
            Element::Repetition(expressions) => {
                debug_parser!("Stack pop: {:?}", expressions);
                expressions
            },
            element => panic!(format!("Expected repetition vector on stack. Got {:?}", element))
        }
    }

    pub fn pop_optional(&mut self) -> Option<Box<expressions::Expression>> {
        if self.elements.is_empty() {
            panic!("Invalid grammar. No elements on stack");
        }

        match self.elements.pop().unwrap() {
            Element::Optional(expression) => {
                debug_parser!("Stack pop: {:?}", expression);
                expression
            },
            element => panic!(format!("Expected optional element on stack. Got {:?}", element))
        }
    }

    pub fn push_single(&mut self, expression: Box<expressions::Expression>) {
        self.elements.push(Element::Single(expression));
        debug_parser!("Stack push: {:?}", self.peek())
    }

    pub fn push_repetition<I>(&mut self, expressions: I)
        where I: IntoIterator <Item = Box<expressions::Expression>, IntoIter = ::std::vec::IntoIter<Box<expressions::Expression>>> {
        self.elements.push(Element::Repetition(VecDeque::from_iter(expressions)));
        debug_parser!("Stack push: {:?}", self.peek())
    }

    pub fn push_optional(&mut self, expression: Option<Box<expressions::Expression>>) {
        self.elements.push(Element::Optional(expression));
        debug_parser!("Stack push: {:?}", self.peek())
    }
}

impl ::std::cmp::PartialEq<&'static str> for Stack {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self.elements) == other.to_string()
    }
}

#[macro_export]
macro_rules! stack_pop {
    ($stack: expr, single) => { $stack.pop_single() };
    ($stack: expr, repetition) => { $stack.pop_repetition() };
    ($stack: expr, optional) => { $stack.pop_optional() };
}

#[macro_export]
macro_rules! stack_unpack {
    ($stack: expr, $($types: ident),+) => {
        ($(stack_pop!($stack, $types)), +)
    }
}