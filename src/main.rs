pub mod utils;
pub mod ast;

pub use ast::lexer_module::lexer;
use ast::parser_module::parser;

// To avoid warnings in tests
#[allow(dead_code, unused_variables)]
fn main()
{
    let _ = parser::Parser::new("and Hello or World\n while".to_owned()).create_AST();
    // let iter = parser.into_iter();
}
