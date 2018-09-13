use ast::parser;
use ast::stack;
use ast::rules;

use interpreter::types;
use interpreter::environment;

pub fn interpret(source_code: &str) -> (types::Type, environment::Environment)
{
    let mut parser = parser::Parser::new(source_code.to_string());
    let mut stack = stack::Stack::new();

    rules::chunk(&mut parser, &mut stack);

    assert!(parser.peek().is_none(), format!("Parser contains tokens after parsing: {:?}", parser));

    let mut env = environment::Environment::new(None);

    (stack.pop_single().eval(&mut env), env)
}

pub fn interpret_epression(source_code: &str) -> (types::Type, environment::Environment)
{
    let mut parser = parser::Parser::new(source_code.to_string());
    let mut stack = stack::Stack::new();

    rules::exp(&mut parser, &mut stack);

    assert!(parser.peek().is_none(), format!("Parser contains tokens after parsing: {:?}", parser));

    let mut env = environment::Environment::new(None);

    (stack.pop_single().eval(&mut env), env)
}