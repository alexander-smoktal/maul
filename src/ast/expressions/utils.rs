use super::*;
use ast::parser;
use ast::expressions::expression;

pub fn terminal(keyword: tokens::Keyword) -> impl FnMut(&mut parser::Parser) -> Option<Box<expression::Expression>> {
    move |ref mut parser| {
        let parser_pos = parser.position();

        parser
        .next()
        .cloned()
        .and_then(|token|
            if token.keyword() == Some(keyword.clone()) {
                Some(Box::new(operators::Noop) as Box<expression::Expression>)
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

#[macro_export]
macro_rules! make_keyword_rule {
    [$fn_name: ident, $(($keyword: pat, $output: expr)),+] => {
        pub fn $fn_name(parser: &mut parser::Parser) -> Option<Box<expression::Expression>> {
            match parser.next() { 
                $(Some(&tokens::Token { token: tokens::TokenType::Keyword($keyword), ..}) => {
                    utils::some_expression($output)
                })+,
                _ => None
            }
        }
    };
}