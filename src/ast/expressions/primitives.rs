use super::*;

use std::string::String as StdString;

#[derive(Debug)]
pub struct String(pub StdString);
impl expression::Expression for String {}

#[derive(Debug)]
pub struct Number(pub f64);
impl expression::Expression for Number {}

#[derive(Debug)]
pub struct Boolean(pub bool);
impl expression::Expression for Boolean {}

#[derive(Debug)]
pub struct Nil;
impl expression::Expression for Nil {}