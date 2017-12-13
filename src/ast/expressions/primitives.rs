use super::*;

use std::string::String as StdString;

#[derive(Debug, Clone)]
pub struct String(pub StdString);
impl expression::Expression for String {}

#[derive(Debug, Clone)]
pub struct Number(pub f64);
impl expression::Expression for Number {}

#[derive(Debug, Clone)]
pub struct Boolean(pub bool);
impl expression::Expression for Boolean {}

#[derive(Debug, Clone)]
pub struct Nil;
impl expression::Expression for Nil {}