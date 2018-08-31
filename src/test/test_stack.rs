use ast::stack;
use ast::expressions::*;
use std::boxed::Box;

#[test]
fn test_stack_unpack() {
    let mut stack = stack::Stack::new();

    stack.push_single(Box::new(primitives::Nil));
    stack.push_repetition(vec![Box::new(primitives::Nil) as Box<Expression>, Box::new(primitives::Nil) as Box<Expression>]);
    stack.push_optional(Some(Box::new(primitives::Nil)));

    println!("{:?}", stack);

    let (optional, repetition, single) = stack_unpack!(stack, optional, repetition, single);
    println!("{:?}, {:?}, {:?}", single, repetition, optional);

    assert!(false);
}