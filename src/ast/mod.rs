pub mod lexer;
pub mod parser;
pub mod expressions;

pub struct AST {
    expressions: expressions::Expressions,
}

impl AST {
    pub fn new(source_code: String) -> Self {
        let parser = parser::Parser::new(source_code);
        let mut ast = AST { expressions: Vec::new() };

        if let Some(result) = expressions::Expression::from_parser(&parser) {
            ast.add_expression(result);
        }

        ast
    }

    fn add_expression(&mut self, exp: expressions::Expression) {
        self.expressions.push(exp);
    }
}