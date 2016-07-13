use std::vec::Vec;

use super::{Expressions, id};

#[allow(dead_code, unused_variables)]
pub struct Function {
    args: Vec<id::Id>,
    body: Expressions
}