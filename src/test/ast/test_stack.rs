use std::collections::VecDeque;

use crate::ast::expressions::*;
use crate::ast::stack;
use std::boxed::Box;

#[test]
fn test_stack_unpack() {
    let mut stack = stack::Stack::default();

    stack.push_single(Box::new(primitives::Nil));
    stack.push_repetition(
        vec![
            Box::new(primitives::Nil) as Box<Expression>,
            Box::new(primitives::Nil) as Box<Expression>,
        ].into_iter()
        .collect::<VecDeque<Box<Expression>>>(),
    );
    stack.push_optional(Some(Box::new(primitives::Nil)));

    println!("{:?}", stack);

    let (optional, repetition, single) = stack_unpack!(stack, optional, repetition, single);
    println!("{:?}, {:?}, {:?}", single, repetition, optional);

    assert_eq!(
        format!("{:?}, {:?}, {:?}", single, repetition, optional),
        "Nil, [Nil, Nil], Some(Nil)"
    );
}
