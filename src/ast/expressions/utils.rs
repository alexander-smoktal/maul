use ast::expressions::expression;

pub fn some_expression<E: expression::Expression + 'static>(expression: E) -> Option<Box<expression::Expression>> {
    log_debug!("Made expression: {:?}", expression);
    Some(Box::new(expression))
}

#[macro_export]
macro_rules! make_keyword_rule {
    [$fn_name: ident, $(($keyword: pat, $output: expr)),+] => {
        pub fn $fn_name(parser: &mut parser::Parser) -> Option<Box<expression::Expression>> {
            match parser.peek().cloned() {
                $(Some(tokens::Token { token: tokens::TokenType::Keyword($keyword), ..}) => {
                    parser.shift();
                    utils::some_expression($output)
                })+,
                _ => None
            }
        }
    };
}