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
        Ok(exp!(util::Expressions(vec![
            exp!(util::Expressions(vec![
                exp!(variables::Assignment(
                    exp!(variables::Id(vec!["one".to_string()])),
                    exp!(primitives::Number(8f64)),
                )),
            ])),
            exp!(Statement::Return(exp!(util::Expressions(vec![
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
        Ok(exp!(blocks::DoBlock(exp!(util::Expressions(vec![
            exp!(util::Expressions(vec![
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
            block: exp!(blocks::DoBlock(exp!(util::Expressions(vec![
                exp!(util::Expressions(vec![
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
            block: exp!(util::Expressions(vec![
                exp!(util::Expressions(vec![
                    Box::new(variables::Assignment(
                        exp!(variables::Id(vec!["one".to_string()])),
                        exp!(primitives::Number(8f64)),
                    )),
                ])),
            ])),
            condition: exp!(primitives::Boolean(false)),
        }))
    )
}
