use ast::lexer::Lexer;
use ast::expressions::*;

pub fn make_lexer(code: &str) -> Lexer {
    Lexer::new(code.to_owned())
}

pub fn make_assignment(ids: Vec<&str>, exp: Expression) -> Expression {
    let sids = ids.iter().map(|s| String::from(*s)).collect();

    Expression::Assignment(Box::new(Expression::Id(sids)), Box::new(exp))
}
