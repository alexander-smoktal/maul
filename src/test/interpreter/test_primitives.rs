use super::utils::interpret_epression;

#[test]
fn test_primitives() {
    let (val, mut _env) = interpret_epression("nil");
    assert_eq!(val, "Nil");

    let (val, _env) = interpret_epression("true");
    assert_eq!(val, "Boolean(true)");

    let (val, _env) = interpret_epression("false");
    assert_eq!(val, "Boolean(false)");

    let (val, _env) = interpret_epression("42.4");
    assert_eq!(val, "Number(42.4)");

    let (val, _env) = interpret_epression(r#""Hello world""#);
    assert_eq!(val, r#"String("Hello world")"#);
}