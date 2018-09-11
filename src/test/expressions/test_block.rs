use super::utils::parse_string;
use ast::rules;


#[test]
fn test_do_block() {
    assert_eq!(parse_string("do one = one + 8 end", rules::stat),
        r#"[Single(DoBlock(Block { statements: [Assignment(Id("one"), Binop(PLUS, Id("one"), Number(8.0)))], retstat: None }))]"#);
}

#[test]
fn test_while_block() {
    assert_eq!(parse_string("while true do one = one * 8; return 10 end", rules::stat),
        r#"[Single(WhileBlock { condition: Boolean(true), block: Block { statements: [Assignment(Id("one"), Binop(MUL, Id("one"), Number(8.0))), Terminal(SEMICOLONS)], retstat: Some(Return(Some(Expressions([Number(10.0)])))) } })]"#);
}

#[test]
fn test_repeat_block() {
    assert_eq!(parse_string("repeat one = 42; break until false", rules::stat),
        r#"[Single(RepeatBlock { block: Block { statements: [Assignment(Id("one"), Number(42.0)), Terminal(SEMICOLONS), Break], retstat: None }, condition: Boolean(false) })]"#);
}


#[test]
fn test_simple_if_block() {
    assert_eq!(parse_string("if true then x =7 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [Assignment(Id("x"), Number(7.0))], retstat: None } }], else_block: None })]"#);
}

#[test]
fn test_if_elseif_block() {
    assert_eq!(parse_string("if true then x =7 elseif false then x= 8 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [Assignment(Id("x"), Number(7.0))], retstat: None } }, IfCondition { condition: Boolean(false), block: Block { statements: [Assignment(Id("x"), Number(8.0))], retstat: None } }], else_block: None })]"#);
}

#[test]
fn test_if_else_block() {
    assert_eq!(parse_string("if true then x =7 else x= 8 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [Assignment(Id("x"), Number(7.0))], retstat: None } }], else_block: Some(Block { statements: [Assignment(Id("x"), Number(8.0))], retstat: None }) })]"#);
}

#[test]
fn test_if_elseif_else_block() {
    assert_eq!(parse_string("if true then x =7 elseif false then x= 8 else x = 1 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [Assignment(Id("x"), Number(7.0))], retstat: None } }, IfCondition { condition: Boolean(false), block: Block { statements: [Assignment(Id("x"), Number(8.0))], retstat: None } }], else_block: Some(Block { statements: [Assignment(Id("x"), Number(1.0))], retstat: None }) })]"#);
}

#[test]
fn test_empty_if_blocks() {
    assert_eq!(parse_string("if true then elseif false then x= 8 else x = 1 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [], retstat: None } }, IfCondition { condition: Boolean(false), block: Block { statements: [Assignment(Id("x"), Number(8.0))], retstat: None } }], else_block: Some(Block { statements: [Assignment(Id("x"), Number(1.0))], retstat: None }) })]"#);
}

#[test]
#[should_panic]
fn test_invalid_if() {
    println!("{:?}", parse_string("if then elseif false then x= 8 else x = 1 end", rules::stat));
}
