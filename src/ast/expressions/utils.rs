use super::*;
use ast::parser;
use ast::expressions::expression;

pub fn keyword(keyword: tokens::Keyword) -> impl FnMut(&mut parser::Parser) -> Option<Box<expression::Expression>> {
    move |ref mut parser| {
        let parser_pos = parser.position();

        parser
        .next()
        .cloned()
        .and_then(|token|
            if token.keyword() == Some(keyword.clone()) {
                Some(Box::new(primitives::Noop) as Box<expression::Expression>)
            } else {
                parser.rollback(parser_pos);
                None
            })
    }
}

pub fn some_expression<E: expression::Expression + 'static>(expression: E) -> Option<Box<expression::Expression>> {
    log_debug!("Made expression: {:?}", expression);
    Some(Box::new(expression))
}
