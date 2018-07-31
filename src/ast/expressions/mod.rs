/*
pub mod function;
pub mod variables;
pub mod tables;
pub mod statements;
pub mod labels;
pub mod operators;*/

#[macro_use]
mod utils;
pub mod blocks;
pub mod expression;
pub mod primitives;

use std::vec::Vec;

use error;
use ast::lexer::tokens;

const DEBUG: bool = false;


