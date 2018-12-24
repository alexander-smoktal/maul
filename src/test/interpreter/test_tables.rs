use std::ops::Deref;

use crate::ast::rules;
use crate::interpreter::types::{Type::{self, Table}};

use super::utils::interpret_rule;

// tableconstructor ::= ‘{’ [fieldlist] ‘}’
// fieldlist ::= field [fieldlist_prefix]
// field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
#[test]
fn test_simple_table() {
    if let (Table { border, .. }, _) = interpret_rule("{}", rules::tableconstructor) {
        assert_eq!(border, 0);
    } else {
        panic!()
    }

    if let (Table { border, map, .. }, _) = interpret_rule("{1}", rules::tableconstructor) {
        assert_eq!(border, 1);
        assert_eq!(map.get(&Type::Number(1f64)).unwrap().borrow().deref(), &Type::Number(1f64));
    } else {
        panic!()
    }

    if let (Table { border, map, .. }, _) = interpret_rule("{1, 2}", rules::tableconstructor) {
        assert_eq!(border, 2);
        assert_eq!(map.get(&Type::Number(1f64)).unwrap().borrow().deref(), &Type::Number(1f64));
        assert_eq!(map.get(&Type::Number(2f64)).unwrap().borrow().deref(), &Type::Number(2f64));
    } else {
        panic!()
    }

    if let (Table { border, map, .. }, _) = interpret_rule("{1; 3}", rules::tableconstructor) {
        assert_eq!(border, 2);
        assert_eq!(map.get(&Type::Number(1f64)).unwrap().borrow().deref(), &Type::Number(1f64));
        assert_eq!(map.get(&Type::Number(2f64)).unwrap().borrow().deref(), &Type::Number(3f64));
    } else {
        panic!()
    }
}

#[test]
fn test_name_table() {
    let (val, _) = interpret_rule(r#"{Hello = 1}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, map, .. } = val {
        assert_eq!(border, 0);
        assert_eq!(map.get(&Type::String("Hello".to_string())).unwrap().borrow().deref(), &Type::Number(1f64));
    } else {
        panic!()
    }

    let (val, _) = interpret_rule(r#"{Hello = 1, 2}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, map, .. } = val {
        assert_eq!(border, 1);
        assert_eq!(map.get(&Type::String("Hello".to_string())).unwrap().borrow().deref(), &Type::Number(1f64));
        assert_eq!(map.get(&Type::Number(1f64)).unwrap().borrow().deref(), &Type::Number(2f64));
    } else {
        panic!()
    }

    let (val, _) = interpret_rule(r#"{Hello = 1; world = false}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, map, .. } = val {
        assert_eq!(border, 0);
        assert_eq!(map.get(&Type::String("Hello".to_string())).unwrap().borrow().deref(), &Type::Number(1f64));
        assert_eq!(map.get(&Type::String("world".to_string())).unwrap().borrow().deref(), &Type::Boolean(false));
    } else {
        panic!()
    }
}

// field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
#[test]
fn test_bracket_table() {
    let (val, _) = interpret_rule(r#"{[1] = 1}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, map, .. } = val {
        assert_eq!(border, 1);
        assert_eq!(map.get(&Type::Number(1f64)).unwrap().borrow().deref(), &Type::Number(1f64));
    } else {
        panic!()
    }

    let (val, _) = interpret_rule(r#"{["Hello"] = 1, 2}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, map, .. } = val {
        assert_eq!(border, 1);
        assert_eq!(map.get(&Type::String("Hello".to_string())).unwrap().borrow().deref(), &Type::Number(1f64));
        assert_eq!(map.get(&Type::Number(1f64)).unwrap().borrow().deref(), &Type::Number(2f64));
    } else {
        panic!()
    }

    let (val, _) = interpret_rule(r#"{[{}] = 1; world = false}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, map, .. } = val {
        assert_eq!(border, 0);
        assert_eq!(map.get(&Type::String("world".to_string())).unwrap().borrow().deref(), &Type::Boolean(false));
    } else {
        panic!()
    }
}

#[test]
fn test_table_border() {
    let (val, _) = interpret_rule(r#"{[1] = 1}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, .. } = val {
        assert_eq!(border, 1);
    } else {
        panic!()
    }

    let (val, _) = interpret_rule(r#"{[1] = 1, [2] = 1}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, .. } = val {
        assert_eq!(border, 2);
    } else {
        panic!()
    }

    let (val, _) = interpret_rule(r#"{[2] = 1, [1] = 1}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, .. } = val {
        assert_eq!(border, 2);
    } else {
        panic!()
    }

    let (val, _) = interpret_rule(r#"{[1] = 1, [3] = 1}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, .. } = val {
        assert_eq!(border, 1);
    } else {
        panic!()
    }

    let (val, _) = interpret_rule(r#"{[1] = 1, [3] = 1, [2] = 1}"#, rules::tableconstructor);
    println!("{:?}", val);
    if let Table { border, .. } = val {
        assert_eq!(border, 3);
    } else {
        panic!()
    }
}

#[test]
#[should_panic(expected = "Runtime error: Cannot use `nil` as a table key")]
fn test_table_invalid_id() {
    interpret_rule(r#"{[hello] = 1}"#, rules::tableconstructor);
}
