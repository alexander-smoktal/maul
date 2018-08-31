use super::utils;
pub use std::ops;

use super::*;

// TODO: Remove this
pub use super::Expression as Expression;

#[derive(Debug)]
pub struct Expressions {
    pub head: Box<Expression>,
    pub tail: Option<Box<Expression>>
}
impl expression::Expression for Expressions {}

impl Expressions {
    pub fn new(head: Box<Expression>, _comma: Option<Box<Expression>>, tail: Option<Box<Expression>>) -> Option<Box<Expression>> {
        utils::some_expression(Expressions {
            head,
            tail
        })
    }
}
