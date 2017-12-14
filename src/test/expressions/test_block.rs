use ast::expressions::*;
use ast::expressions::statements::*;

use super::utils::*;

#[test]
fn test_block() {
    assert_eq!(
        blocks::parse_block(&mut make_lexer(
            "one = 8
                                                    return nil, false, true;",
        )),
        Ok(exp!(common::Expressions(vec![
            exp!(common::Expressions(vec![
                exp!(variables::Assignment(
                    exp!(variables::Id(vec!["one".to_string()])),
                    exp!(primitives::Number(8f64)),
                )),
            ])),
            exp!(Statement::Return(exp!(common::Expressions(vec![
                exp!(primitives::Nil),
                exp!(primitives::Boolean(false)),
                exp!(primitives::Boolean(true)),
            ])))),
        ])))
    )
}

#[test]
fn test_do_block() {
    assert_eq!(
        blocks::parse_do_block(&mut make_lexer("do one = 8 end")),
        Ok(exp!(blocks::DoBlock(exp!(common::Expressions(vec![
            exp!(common::Expressions(vec![
                Box::new(variables::Assignment(
                    exp!(variables::Id(vec!["one".to_string()])),
                    exp!(primitives::Number(8f64)),
                )),
            ])),
        ])))))
    )
}

#[test]
fn test_while_block() {
    assert_eq!(
        blocks::parse_while_block(&mut make_lexer("while true do one = 8 end")),
        Ok(exp!(blocks::WhileBlock {
            condition: exp!(primitives::Boolean(true)),
            block: exp!(blocks::DoBlock(exp!(common::Expressions(vec![
                exp!(common::Expressions(vec![
                    Box::new(variables::Assignment(
                        exp!(variables::Id(vec!["one".to_string()])),
                        exp!(primitives::Number(8f64)),
                    )),
                ])),
            ])))),
        }))
    )
}

#[test]
fn test_repeat_block() {
    assert_eq!(
        blocks::parse_repeat_block(&mut make_lexer("repeat one = 8 until false")),
        Ok(exp!(blocks::RepeatBlock {
            block: exp!(common::Expressions(vec![
                exp!(common::Expressions(vec![
                    exp!(variables::Assignment(
                        exp!(variables::Id(vec!["one".to_string()])),
                        exp!(primitives::Number(8f64)),
                    )),
                ])),
            ])),
            condition: exp!(primitives::Boolean(false)),
        }))
    )
}

// If blocks
#[test]
fn test_simple_if_block() {
    assert_eq!(
        blocks::parse_if_block(&mut make_lexer("if true then x =7 end")),
        Ok(exp!(blocks::IfBlock {
            conditions: vec![
                blocks::Condition {
                    condition: exp!(primitives::Boolean(true)),
                    block: exp!(common::Expressions(vec![
                        exp!(common::Expressions(vec![
                            exp!(variables::Assignment(
                                exp!(variables::Id(vec!["x".to_string()])),
                                exp!(primitives::Number(7f64)),
                            )),
                        ])),
                    ])),
                },
            ],
            elseblock: None,
        }))
    )
}

#[test]
fn test_if_elseif_block() {
    assert_eq!(
        blocks::parse_if_block(&mut make_lexer(
            "if true then x =7 elseif false then x= 8 end",
        )),
        Ok(exp!(blocks::IfBlock {
            conditions: vec![
                blocks::Condition {
                    condition: exp!(primitives::Boolean(true)),
                    block: exp!(common::Expressions(vec![
                        exp!(common::Expressions(vec![
                            exp!(variables::Assignment(
                                exp!(variables::Id(vec!["x".to_string()])),
                                exp!(primitives::Number(7f64)),
                            )),
                        ])),
                    ])),
                },
                blocks::Condition {
                    condition: exp!(primitives::Boolean(false)),
                    block: exp!(common::Expressions(vec![
                        exp!(common::Expressions(vec![
                            exp!(variables::Assignment(
                                exp!(variables::Id(vec!["x".to_string()])),
                                exp!(primitives::Number(8f64)),
                            )),
                        ])),
                    ])),
                },
            ],
            elseblock: None,
        }))
    )
}

#[test]
fn test_if_else_block() {
    assert_eq!(
        blocks::parse_if_block(&mut make_lexer("if true then x =7 else x= 8 end")),
        Ok(exp!(blocks::IfBlock {
            conditions: vec![
                blocks::Condition {
                    condition: exp!(primitives::Boolean(true)),
                    block: exp!(common::Expressions(vec![
                        exp!(common::Expressions(vec![
                            exp!(variables::Assignment(
                                exp!(variables::Id(vec!["x".to_string()])),
                                exp!(primitives::Number(7f64)),
                            )),
                        ])),
                    ])),
                },
            ],
            elseblock: Some(exp!(common::Expressions(vec![
                exp!(common::Expressions(vec![
                    exp!(variables::Assignment(
                        exp!(variables::Id(vec!["x".to_string()])),
                        exp!(primitives::Number(8f64)),
                    )),
                ])),
            ]))),
        }))
    )
}

#[test]
fn test_if_elseif_else_block() {
    assert_eq!(
        blocks::parse_if_block(&mut make_lexer(
            "if true then x =7 elseif false then x= 8 else x = 1 end",
        )),
        Ok(exp!(blocks::IfBlock {
            conditions: vec![
                blocks::Condition {
                    condition: exp!(primitives::Boolean(true)),
                    block: exp!(common::Expressions(vec![
                        exp!(common::Expressions(vec![
                            exp!(variables::Assignment(
                                exp!(variables::Id(vec!["x".to_string()])),
                                exp!(primitives::Number(7f64)),
                            )),
                        ])),
                    ])),
                },
                blocks::Condition {
                    condition: exp!(primitives::Boolean(false)),
                    block: exp!(common::Expressions(vec![
                        exp!(common::Expressions(vec![
                            exp!(variables::Assignment(
                                exp!(variables::Id(vec!["x".to_string()])),
                                exp!(primitives::Number(8f64)),
                            )),
                        ])),
                    ])),
                },
            ],
            elseblock: Some(exp!(common::Expressions(vec![
                exp!(common::Expressions(vec![
                    exp!(variables::Assignment(
                        exp!(variables::Id(vec!["x".to_string()])),
                        exp!(primitives::Number(1f64)),
                    )),
                ])),
            ]))),
        }))
    )
}
