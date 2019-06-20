#![allow(clippy::float_cmp)]

use crate::interpreter::types;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_single_type_match() {
    let num1 = types::Type::Number(32f64);
    let mut val = match_type!(&num1,
        types::Type::Number(value) => value,
        _ => &-1f64
    );

    assert_eq!(val, &32f64);

    let num2 = types::Type::Reference(Rc::new(RefCell::new(types::Type::Number(32f64))));
    val = match_type!(&num2,
        types::Type::Number(value) => value,
        _ => &-1f64
    );

    assert_eq!(val, &32f64);
}

#[test]
fn test_multiple_type_match() {
    let num1 = types::Type::Number(32f64);
    let num11 = types::Type::Number(18f64);
    let mut val = match_type!((&num1, &num11),
        (types::Type::Number(value1), types::Type::Number(value2)) => value1 + value2,
        _ => -1f64
    );

    assert_eq!(val, 50f64);

    let num2 = types::Type::Reference(Rc::new(RefCell::new(types::Type::Number(32f64))));
    let num21 = types::Type::Reference(Rc::new(RefCell::new(types::Type::Number(18f64))));
    val = match_type!((&num1, &num21),
        (types::Type::Number(value1), types::Type::Number(value2)) => value1 + value2,
        _ => -1f64
    );

    assert_eq!(val, 50f64);

    val = match_type!((&num11, &num2),
        (types::Type::Number(value1), types::Type::Number(value2)) => value1 + value2,
        _ => -1f64
    );

    assert_eq!(val, 50f64);

    val = match_type!((&num2, &num21),
        (types::Type::Number(value1), types::Type::Number(value2)) => value1 + value2,
        _ => -1f64
    );

    assert_eq!(val, 50f64);
}
