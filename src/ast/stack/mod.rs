use std::collections::VecDeque;

use crate::ast::expressions;

const DEBUG: bool = false;

#[derive(Debug)]
pub enum Element {
    Single(Box<dyn expressions::Expression>),
    Repetition(VecDeque<Box<dyn expressions::Expression>>),
    Optional(Option<Box<dyn expressions::Expression>>),
}

#[derive(Debug, Default)]
pub struct Stack {
    elements: Vec<Element>,
}

impl Stack {
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn peek(&self) -> &Element {
        self.elements.last().unwrap()
    }

    pub fn pop_single(&mut self) -> Box<dyn expressions::Expression> {
        if self.elements.is_empty() {
            panic!("Invalid grammar. No elements on stack");
        }

        match self.elements.pop().unwrap() {
            Element::Single(expression) => {
                debug_parser!("Stack pop: {:?}", expression);
                expression
            }
            element => panic!(format!(
                "Expected single element on stack. Got {:?}\nStack: {:?}",
                element, self
            )),
        }
    }

    pub fn pop_repetition(&mut self) -> VecDeque<Box<dyn expressions::Expression>> {
        if self.elements.is_empty() {
            panic!("Invalid grammar. No elements on stack");
        }

        match self.elements.pop().unwrap() {
            Element::Repetition(expressions) => {
                debug_parser!("Stack pop: {:?}", expressions);
                expressions
            }
            element => panic!(format!(
                "Expected repetition vector on stack. Got {:?}\nStack: {:?}",
                element, self
            )),
        }
    }

    pub fn pop_optional(&mut self) -> Option<Box<dyn expressions::Expression>> {
        if self.elements.is_empty() {
            panic!("Invalid grammar. No elements on stack");
        }

        match self.elements.pop().unwrap() {
            Element::Optional(expression) => {
                debug_parser!("Stack pop: {:?}", expression);
                expression
            }
            element => panic!(format!(
                "Expected optional element on stack. Got {:?}\nStack: {:?}",
                element, self
            )),
        }
    }

    pub fn push_single(&mut self, expression: Box<dyn expressions::Expression>) {
        self.elements.push(Element::Single(expression));
        debug_parser!("Stack push: {:?}", self.peek())
    }

    pub fn push_repetition(&mut self, expressions: VecDeque<Box<dyn expressions::Expression>>) {
        self.elements.push(Element::Repetition(expressions));
        debug_parser!("Stack push: {:?}", self.peek())
    }

    pub fn push_optional(&mut self, expression: Option<Box<dyn expressions::Expression>>) {
        self.elements.push(Element::Optional(expression));
        debug_parser!("Stack push: {:?}", self.peek())
    }
}

#[cfg(test)]
impl ::std::cmp::PartialEq<&'static str> for Stack {
    fn eq(&self, other: &&'static str) -> bool {
        format!("{:?}", self.elements) == *other
    }
}

#[macro_export]
macro_rules! stack_pop {
    ($stack: expr, single) => {
        $stack.pop_single()
    };
    ($stack: expr, repetition) => {
        $stack.pop_repetition()
    };
    ($stack: expr, optional) => {
        $stack.pop_optional()
    };
}

#[macro_export]
macro_rules! stack_unpack {
    ($stack: expr, $($types: ident),+) => {
        ($(stack_pop!($stack, $types)), +)
    }
}
