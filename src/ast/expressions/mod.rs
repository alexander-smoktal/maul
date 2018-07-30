/*
pub mod function;
pub mod variables;
pub mod tables;
pub mod statements;
pub mod labels;
pub mod operators;*/

pub mod blocks;
pub mod expression;
pub mod primitives;
mod utils;

use std::vec::Vec;

use error;
use ast::lexer::tokens;

const DEBUG: bool = false;


