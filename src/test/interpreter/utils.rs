use ast::parser;
use ast::stack;
use ast::rules;

use interpreter::types;
use interpreter::environment;

#[allow(dead_code)]
pub fn interpret(source_code: &str) -> (types::Type, environment::Environment)
{
    interpret_rule(source_code, rules::chunk)
}

pub fn interpret_rule<F>(source_code: &str, func: F) -> (types::Type, environment::Environment) where
    F: Fn(&mut parser::Parser, &mut stack::Stack) -> bool
{
    let mut parser = parser::Parser::new(source_code.to_string());
    let mut stack = stack::Stack::new();

    func(&mut parser, &mut stack);

    assert!(parser.peek().is_none(), format!("Parser contains tokens after parsing: {:?}", parser));

    let mut env = environment::Environment::new(None);

    (stack.pop_single().eval(&mut env), env)
}