mod function;
mod id;

use std::vec::Vec;

pub enum Expression {
    Function(function::Function),
    Id(id::Id)
}

pub type Expressions = Vec<Expression>;