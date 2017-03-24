mod tests;

use super::lexer::Lexer;
use super::expressions;

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
}

/// I'll try to make this recursive descendant parser, buy hey... nobody is perfect
impl Parser {
    pub fn new(input: String) -> super::AST {
        let mut lexer = Lexer::new(input);

        let mut ast = super::AST { expressions: Vec::new() };

        if let Some(result) = expressions::Expression::from_lexer(&mut lexer) {
            ast.add_expression(result);
        }

        ast
    }
}