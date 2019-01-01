use super::utils::parse_string;
use crate::ast::rules;

#[test]
fn test_table_fields() {
    assert_eq!(
        parse_string(r#"["Key"] = true"#, rules::field),
        r#"[Single(TableField { key: Some(String("Key")), value: Boolean(true) })]"#
    );
    assert_eq!(
        parse_string("Key = false", rules::field),
        r#"[Single(TableField { key: Some(String("Key")), value: Boolean(false) })]"#
    );
    assert_eq!(
        parse_string("7", rules::field),
        "[Single(TableField { key: None, value: Number(7.0) })]"
    );
}

#[test]
fn test_table() {
    assert_eq!(parse_string(r#"{["Key"] = true, Key = false, 7}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(String("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true, Key = false, 7,}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(String("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true; Key = false; 7}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(String("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true; Key = false; 7;}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(String("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true; Key = false, 7}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(String("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
    assert_eq!(parse_string(r#"{["Key"] = true, Key = false; 7}"#, rules::tableconstructor),
        r#"[Single(Table([TableField { key: Some(String("Key")), value: Boolean(true) }, TableField { key: Some(String("Key")), value: Boolean(false) }, TableField { key: None, value: Number(7.0) }]))]"#);
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
    parse_string(
        r#"{["Key"] = true Key = false, 7}"#,
        rules::tableconstructor,
    );
}

#[test]
#[should_panic]
fn test_invalid3() {
    parse_string(r#"{["Key"] = true Key = false, 7"#, rules::tableconstructor);
}
