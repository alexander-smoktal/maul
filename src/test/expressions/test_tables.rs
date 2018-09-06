use super::utils::parse_string;
use ast::rules;

#[test]
fn test_table_fields() {
    assert_eq!(
        parse_string(r#"["Key"] = true"#, rules::field), r#"[Single(TableField { key: Some(String("Key")), value: Boolean(true) })]"#
    );
    assert_eq!(
        parse_string("Key = false", rules::field), r#"[Single(TableField { key: Some(Id("Key")), value: Boolean(false) })]"#
    );
    assert_eq!(
        parse_string("7", rules::field), "[Single(TableField { key: None, value: Number(7.0) })]"
    );
}

#[test]
fn test_table() {
    assert_eq!(parse_string(r#"{["Key"] = true, Key = false, 7}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(Id("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true, Key = false, 7,}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(Id("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true; Key = false; 7}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(Id("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true; Key = false; 7;}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(Id("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true; Key = false, 7}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(Id("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true, Key = false; 7}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(Id("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
}

#[test]
#[should_panic]
fn test_invalid0() {
    parse_string(r#"{["Key"] = true, Key = }"#, rules::tableconstructor);
}

#[test]
#[should_panic]
fn test_invalid1() {
    parse_string(r#"{["Key"] = true, , 7}"#, rules::tableconstructor);
}

#[test]
#[should_panic]
fn test_invalid2() {
    parse_string(r#"{["Key"] = true Key = false, 7}"#, rules::tableconstructor);
}

#[test]
#[should_panic]
fn test_invalid3() {
    parse_string(r#"{["Key"] = true Key = false, 7"#, rules::tableconstructor);
}

/*#[test]
#[should_panic]
fn test_table_invalid() {
    assert_eq!(
        tables::parse_table_constructor(&mut make_lexer("1 + 3")),
        Ok(exp!(common::Noop))
    )
}

#[test]
fn test_table_constructor_simple() {
    assert_eq!(
        tables::parse_table_constructor(&mut make_lexer("{7}")),
        Ok(exp!(tables::TableConstructor(
            vec![exp!(primitives::Number(7f64))],
        )))
    );
}

#[test]
fn test_table_constructor_assignment() {
    assert_eq!(
        tables::parse_table_constructor(&mut make_lexer("{ Sashok = false }")),
        Ok(exp!(tables::TableConstructor(vec![
            exp!(variables::Assignment(
                exp!(variables::Id(vec!["Sashok".to_string()])),
                exp!(primitives::Boolean(false)),
            )),
        ])))
    );
}

#[test]
fn test_table_constructor_index_assignment() {
    assert_eq!(
        tables::parse_table_constructor(&mut make_lexer("{ [true] = false }")),
        Ok(exp!(tables::TableConstructor(vec![
            exp!(variables::Assignment(
                exp!(primitives::Boolean(true)),
                exp!(primitives::Boolean(false)),
            )),
        ])))
    );
}*/
