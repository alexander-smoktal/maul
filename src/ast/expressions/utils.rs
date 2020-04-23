use crate::ast::expressions;

pub fn some_expression<E: expressions::Expression + 'static>(
    expression: E,
) -> Option<Box<dyn expressions::Expression>> {
    log_debug!("Made expression: {:?}", expression);
    Some(Box::new(expression))
}

#[macro_export]
macro_rules! make_keyword_rule {
    [$fn_name: ident, $(($keyword: pat, $output: expr)),+] => {
        pub fn $fn_name(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
            match parser.peek().cloned() {
                $(Some(tokens::Token { token: tokens::TokenType::Keyword($keyword), ..}) => {
                    parser.shift();
                    stack.push_single(Box::new($output));
                    true
                })+,
                _ => false
            }
        }
    };
}
