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

/*
#[test]
fn test_simple_if_block() {
        blocks::parse_if_block(&mut make_lexer("if true then x =7 end")),
}

#[test]
fn test_if_elseif_block() {
            "if true then x =7 elseif false then x= 8 end",
}

#[test]
fn test_if_else_block() {
        blocks::parse_if_block(&mut make_lexer("if true then x =7 else x= 8 end")),
}

#[test]
fn test_if_elseif_else_block() {
            "if true then x =7 elseif false then x= 8 else x = 1 end",
}*/
