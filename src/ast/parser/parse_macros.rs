//trace_macros!(true);

#[macro_export]
macro_rules! debug_parser {
    ($($params: expr), +) => {
        if DEBUG {
            println!($($params,) +);
        }
    };
}

#[macro_export]
macro_rules! or {
    [$($parse_funcs: expr),+] => {
        |parser: &mut parser::Parser| -> Option<Box<expression::Expression>> {
            $(
                let result = $parse_funcs(parser);

                if result.is_some() {
                    debug_parser!("Or statement rule {} accepted expression {:?}. Parser state {:?}", stringify!($parse_funcs), result, parser);
                    parser.shift();
                    return result
                } else {
                    debug_parser!("Or statement rule {} didn't accept parser input {:?}", stringify!($parse_funcs), parser);
                }
            )+;

            debug_parser!("Or statement fails");
            None
        }
    }
}

#[macro_export]
macro_rules! and {
    [($($parse_funcs: expr),+) => $nandler_func: expr] => {
        |parser: &mut parser::Parser| -> Option<Box<expression::Expression>> {

            let results = ($(match $parse_funcs(parser) {
                Some(expression) => {
                    debug_parser!("And statement rule {} accepted expression {:?}. Parser state {:?}", stringify!($parse_funcs), expression, parser);
                    parser.shift();
                    expression
                }
                _ => {
                    debug_parser!("And statement rule {} didn't accept parser input {:?}", stringify!($parse_funcs), parser);
                    return None
                }
            }), +);

            match ops::Fn::call(&$nandler_func, results) {
                expression @ Some(_) => {
                    debug_parser!("And handling function successfully handled expression and returned {:?}", expression);
                    expression
                }
                _ => {
                    debug_parser!("And handling function failed to process expressions");
                    return None
                }
            }
        }
    };
}

#[macro_export]
macro_rules! optional {
    ($parse_func:expr) => {
        |parser: &mut parser::Parser| -> Option<Option<Box<expression::Expression>>> {
            if let Some(result) = $parse_func(parser) {
                Some(Some(result))
            } else {
                Some(None)
            }
        }
    }
}

#[macro_export]
macro_rules! rule {
    ($name: ident, $parse_func:expr) => {
        pub fn $name(parser: &mut parser::Parser) -> Option<Box<expression::Expression>> {
            debug_parser!("Executing rule {}", stringify!($name));

            $parse_func(parser)
        }
    };
}

#[macro_export]
macro_rules! terminal {
    ($keyword: expr) => {
        |parser: &mut parser::Parser| -> Option<Box<expression::Expression>> {
            if let Some(token) = parser.peek().cloned() {
                if token.keyword() == Some($keyword) {
                    parser.shift();
                    Some(Box::new(operators::Noop) as Box<expression::Expression>)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}