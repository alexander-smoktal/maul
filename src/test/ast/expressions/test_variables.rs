use super::utils::parse_string;
use ast::rules;

#[test]
// var_suffix ::= ‘[’ exp ‘]’ [var_suffix] | ‘.’ Name [var_suffix]
// var ::=  Name [var_suffix] | functioncall var_suffix | ‘(’ exp ‘)’ var_suffix
fn test_var() {
    assert_eq!(
        parse_string("variable", rules::var),
        r#"[Single(Id("variable"))]"#
    );
    assert_eq!(
        parse_string("(nil)[nil]", rules::var),
        "[Single(Indexing { object: Nil, index: Nil })]"
    );
    assert_eq!(
        parse_string("(nil).func", rules::var),
        r#"[Single(Indexing { object: Nil, index: String("func") })]"#
    );
    assert_eq!(
        parse_string("variable[nil]", rules::var),
        r#"[Single(Indexing { object: Id("variable"), index: Nil })]"#
    );
    assert_eq!(
        parse_string("variable.func", rules::var),
        r#"[Single(Indexing { object: Id("variable"), index: String("func") })]"#
    );
}

#[test]
#[should_panic]
fn test_invalid_var() {
    parse_string("(nil)", rules::var);
}

#[test]
fn test_var_recursive() {
    assert_eq!(parse_string("variable[nil].func", rules::var), r#"[Single(Indexing { object: Indexing { object: Id("variable"), index: Nil }, index: String("func") })]"#);
    assert_eq!(parse_string("variable.func[nil]", rules::var), r#"[Single(Indexing { object: Indexing { object: Id("variable"), index: String("func") }, index: Nil })]"#);
}

#[test]
fn test_varlist() {
    assert_eq!(parse_string("var1, var2, var3[nil].func", rules::varlist), r#"[Repetition([Id("var1"), Id("var2"), Indexing { object: Indexing { object: Id("var3"), index: Nil }, index: String("func") }])]"#);
}

#[test]
fn test_assignment() {
    assert_eq!(parse_string("var = 7", rules::stat), r#"[Single(Assignment { varlist: [Id("var")], explist: [Number(7.0)] })]"#);
    assert_eq!(parse_string("var1, var2 = 7, false", rules::stat), r#"[Single(Assignment { varlist: [Id("var1"), Id("var2")], explist: [Number(7.0), Boolean(false)] })]"#);
    assert_eq!(parse_string(r#"var1.data, var2["key"] = 7, false"#, rules::stat),
        r#"[Single(Assignment { varlist: [Indexing { object: Id("var1"), index: String("data") }, Indexing { object: Id("var2"), index: String("key") }], explist: [Number(7.0), Boolean(false)] })]"#);
    assert_eq!(parse_string("var1, var2, var3[nil].func = -7, object:method(), 11 - 3 + 5", rules::stat),
        r#"[Single(Assignment { varlist: [Id("var1"), Id("var2"), Indexing { object: Indexing { object: Id("var3"), index: Nil }, index: String("func") }], explist: [Unop(MINUS, Number(7.0)), Funcall { object: Id("object"), args: [], method: Some(String("method")) }, Binop(PLUS, Binop(MINUS, Number(11.0), Number(3.0)), Number(5.0))] })]"#);
}

#[test]
fn test_varlist_more_vars() {
    assert_eq!(parse_string("var1, var2 = 7", rules::stat), r#"[Single(Assignment { varlist: [Id("var1"), Id("var2")], explist: [Number(7.0)] })]"#);
}

#[test]
fn test_varlist_more_expressions() {
    assert_eq!(parse_string("var1 = 7, false", rules::stat), r#"[Single(Assignment { varlist: [Id("var1")], explist: [Number(7.0), Boolean(false)] })]"#);
}

#[test]
fn test_local_assignment() {
    assert_eq!(parse_string("local var1, var2 = 7, false", rules::stat), r#"[Single(Local(Assignment { varlist: [String("var1"), String("var2")], explist: [Number(7.0), Boolean(false)] }))]"#);
    assert_eq!(parse_string("local var1, var2 = 7", rules::stat), r#"[Single(Local(Assignment { varlist: [String("var1"), String("var2")], explist: [Number(7.0)] }))]"#);
    assert_eq!(parse_string("local var1, var2", rules::stat), r#"[Single(Local(Assignment { varlist: [String("var1"), String("var2")], explist: [] }))]"#);
}

#[test]
#[should_panic]
fn test_local_assignment_invalid1() {
    parse_string("local var1, var2 =, false", rules::stat);
}

#[test]
#[should_panic]
fn test_local_assignment_invalid2() {
    parse_string("local var1, var2 =", rules::stat);
}

#[test]
#[should_panic]
fn test_local_assignment_invalid3() {
    parse_string(r#"local var1, var2["key"]"#, rules::block);
}