use crate::ast::parser;
use crate::ast::rules;
use crate::ast::stack;
use crate::utils;

use crate::interpreter::environment;
use crate::interpreter::types;

#[allow(dead_code)]
pub fn interpret(source_code: &str) -> (types::Type, utils::Shared<environment::Environment>) {
    interpret_rule(source_code, rules::chunk)
}

pub fn interpret_rule<F>(
    source_code: &str,
    func: F,
) -> (types::Type, utils::Shared<environment::Environment>)
where
    F: Fn(&mut parser::Parser, &mut stack::Stack) -> bool,
{
    let mut parser = parser::Parser::new(source_code.to_string());
    let mut stack = stack::Stack::default();

    func(&mut parser, &mut stack);

    assert!(
        parser.peek().is_none(),
        format!("Parser contains tokens after parsing: {:?}", parser)
    );

    let mut env = utils::Shared::new(environment::Environment::new(
        None,
        environment::BreakFlag::None,
    ));

    (stack.pop_single().eval(&mut env), env)
}

pub fn interpret_rule_env<F>(
    source_code: &str,
    func: F,
    env: &mut utils::Shared<environment::Environment>,
) -> (types::Type, utils::Shared<environment::Environment>)
where
    F: Fn(&mut parser::Parser, &mut stack::Stack) -> bool,
{
    let mut parser = parser::Parser::new(source_code.to_string());
    let mut stack = stack::Stack::default();

    func(&mut parser, &mut stack);

    assert!(
        parser.peek().is_none(),
        format!("Parser contains tokens after parsing: {:?}", parser)
    );
    let result = stack.pop_single().eval(env);

    (result, env.clone())
}
