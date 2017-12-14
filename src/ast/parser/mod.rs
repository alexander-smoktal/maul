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

        let mut ast = super::AST { expressions: Box::new(expressions::common::Noop) };

        for expression in expressions::expression::from_lexer(&mut lexer) {
            ast.add_expression(expression);
        }

        ast
    }
}
