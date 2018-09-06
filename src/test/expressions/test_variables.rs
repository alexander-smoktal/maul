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
        r#"[Single(Indexing { object: Nil, index: Id("func") })]"#
    );
    assert_eq!(
        parse_string("variable[nil]", rules::var),
        r#"[Single(Indexing { object: Id("variable"), index: Nil })]"#
    );
    assert_eq!(
        parse_string("variable.func", rules::var),
        r#"[Single(Indexing { object: Id("variable"), index: Id("func") })]"#
    );
}

#[test]
#[should_panic]
fn test_invalid_var() {
    parse_string("(nil)", rules::var);
}

#[test]
fn test_var_recursive() {
    assert_eq!(parse_string("variable[nil].func", rules::var), r#"[Single(Indexing { object: Indexing { object: Id("variable"), index: Nil }, index: Id("func") })]"#);
    assert_eq!(parse_string("variable.func[nil]", rules::var), r#"[Single(Indexing { object: Indexing { object: Id("variable"), index: Id("func") }, index: Nil })]"#);
}

#[test]
fn test_varlist() {
    assert_eq!(parse_string("var1, var2, var3[nil].func", rules::varlist), r#"[Repetition([Id("var1"), Id("var2"), Indexing { object: Indexing { object: Id("var3"), index: Nil }, index: Id("func") }])]"#);
}
