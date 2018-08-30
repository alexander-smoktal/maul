/*
pub mod function;
pub mod variables;
pub mod tables;
pub mod labels;
*/

#[macro_use]
pub mod utils;
pub mod blocks;
pub mod expression;
pub mod primitives;
pub mod operators;
pub mod statements;

use std::vec::Vec;

use error;
use ast::lexer::tokens;

const DEBUG: bool = false;