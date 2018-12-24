use super::utils::parse_string;
use crate::ast::rules;


#[test]
fn test_do_block() {
    assert_eq!(parse_string("do one = one + 8 end", rules::stat),
        r#"[Single(DoBlock(Block { statements: [Assignment { varlist: [Id("one")], explist: [Binop(PLUS, Id("one"), Number(8.0))] }], retstat: None }))]"#);
}

#[test]
fn test_while_block() {
    assert_eq!(parse_string("while true do one = one * 8; return 10 end", rules::stat),
        r#"[Single(WhileBlock { condition: Boolean(true), block: Block { statements: [Assignment { varlist: [Id("one")], explist: [Binop(MUL, Id("one"), Number(8.0))] }, Terminal(SEMICOLONS)], retstat: Some(Return(Some(Expressions([Number(10.0)])))) } })]"#);
}

#[test]
fn test_repeat_block() {
    assert_eq!(parse_string("repeat one = 42; break until false", rules::stat),
        r#"[Single(RepeatBlock { block: Block { statements: [Assignment { varlist: [Id("one")], explist: [Number(42.0)] }, Terminal(SEMICOLONS), Break], retstat: None }, condition: Boolean(false) })]"#);
}


#[test]
fn test_simple_if_block() {
    assert_eq!(parse_string("if true then x =7 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(7.0)] }], retstat: None } }], else_block: None })]"#);
}

#[test]
fn test_if_elseif_block() {
    assert_eq!(parse_string("if true then x =7 elseif false then x= 8 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(7.0)] }], retstat: None } }, IfCondition { condition: Boolean(false), block: Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(8.0)] }], retstat: None } }], else_block: None })]"#);
}

#[test]
fn test_if_else_block() {
    assert_eq!(parse_string("if true then x =7 else x= 8 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(7.0)] }], retstat: None } }], else_block: Some(Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(8.0)] }], retstat: None }) })]"#);
}

#[test]
fn test_if_elseif_else_block() {
    assert_eq!(parse_string("if true then x =7 elseif false then x= 8 else x = 1 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(7.0)] }], retstat: None } }, IfCondition { condition: Boolean(false), block: Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(8.0)] }], retstat: None } }], else_block: Some(Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(1.0)] }], retstat: None }) })]"#);
}

#[test]
fn test_empty_if_blocks() {
    assert_eq!(parse_string("if true then elseif false then x= 8 else x = 1 end", rules::stat),
        r#"[Single(IfBlock { conditions: [IfCondition { condition: Boolean(true), block: Block { statements: [], retstat: None } }, IfCondition { condition: Boolean(false), block: Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(8.0)] }], retstat: None } }], else_block: Some(Block { statements: [Assignment { varlist: [Id("x")], explist: [Number(1.0)] }], retstat: None }) })]"#);
}

#[test]
#[should_panic]
fn test_invalid_if() {
    println!("{:?}", parse_string("if then elseif false then x= 8 else x = 1 end", rules::stat));
}

#[test]
fn test_numerical_for() {
    assert_eq!(parse_string("for x = 7, x == 6 do break end", rules::stat),
        r#"[Single(NumericalForBlock { init: Assignment { varlist: [Id("x")], explist: [Number(7.0)] }, limit: Binop(EQ, Id("x"), Number(6.0)), step: None, block: Block { statements: [Break], retstat: None } })]"#);
    assert_eq!(parse_string("for x = 7, x == 6, -1 do break end", rules::stat),
        r#"[Single(NumericalForBlock { init: Assignment { varlist: [Id("x")], explist: [Number(7.0)] }, limit: Binop(EQ, Id("x"), Number(6.0)), step: Some(Unop(MINUS, Number(1.0))), block: Block { statements: [Break], retstat: None } })]"#);
}

#[test]
fn test_generic_for() {
    assert_eq!(parse_string("for x in xlist do break end", rules::stat),
        r#"[Single(GenericForBlock { namelist: [Id("x")], explist: [Id("xlist")], block: Block { statements: [Break], retstat: None } })]"#);
    assert_eq!(parse_string("for x, y in xlist do break end", rules::stat),
        r#"[Single(GenericForBlock { namelist: [Id("x"), Id("y")], explist: [Id("xlist")], block: Block { statements: [Break], retstat: None } })]"#);
    assert_eq!(parse_string("for x in xlist, 0, 1 do break end", rules::stat),
        r#"[Single(GenericForBlock { namelist: [Id("x")], explist: [Id("xlist"), Number(0.0), Number(1.0)], block: Block { statements: [Break], retstat: None } })]"#);
    assert_eq!(parse_string("for x,y in xlist, 0, 1 do break end", rules::stat),
        r#"[Single(GenericForBlock { namelist: [Id("x"), Id("y")], explist: [Id("xlist"), Number(0.0), Number(1.0)], block: Block { statements: [Break], retstat: None } })]"#);
}