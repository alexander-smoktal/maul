use ast::lexer::Lexer;
use ast::expressions::*;

pub fn make_lexer(code: &str) -> Lexer {
    Lexer::new(code.to_owned())
}

pub fn make_assignment(ids: Vec<&str>, exp: Box<expression::Expression>) -> Box<expression::Expression> {
    let sids = ids.iter().map(|s| String::from(*s)).collect();

    Box::new(variables::Assignment(Box::new(variables::Id(sids)), exp))
}
