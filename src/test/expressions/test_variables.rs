use ast::expressions::*;
use ast::parser::*;

#[test]
// var_suffix ::= ‘[’ exp ‘]’ [var_suffix] | ‘.’ Name [var_suffix]
// var ::=  Name [var_suffix] | functioncall var_suffix | ‘(’ exp ‘)’ var_suffix
fn test_var() {
    let mut parser = Parser::new("variable".to_string());
    assert_eq!(rules::var(&mut parser), sexp!(expression::Expressions { head: exp!(variables::Id("variable".to_string())), tail: None }));

    parser = Parser::new("(nil)[nil]".to_string());
    assert_eq!(rules::var(&mut parser), sexp!(
        expression::Expressions { head: exp!(primitives::Nil),
                                  tail: sexp!(expression::Expressions { head: exp!(tables::Indexing(exp!(primitives::Nil))), tail: None }) }));

    parser = Parser::new("(nil).func".to_string());
    assert_eq!(rules::var(&mut parser), sexp!(
        expression::Expressions { head: exp!(primitives::Nil),
                                  tail: sexp!(expression::Expressions { head: exp!(tables::Indexing(exp!(variables::Id("func".to_string())))), tail: None }) }));

    parser = Parser::new("variable[nil]".to_string());
    assert_eq!(rules::var(&mut parser), sexp!(
        expression::Expressions { head: exp!(variables::Id("variable".to_string())),
                                  tail: sexp!(expression::Expressions { head: exp!(tables::Indexing(exp!(primitives::Nil))), tail: None }) }));

    parser = Parser::new("variable.func".to_string());
    assert_eq!(rules::var(&mut parser), sexp!(
        expression::Expressions { head: exp!(variables::Id("variable".to_string())),
                                  tail: sexp!(expression::Expressions { head: exp!(tables::Indexing(exp!(variables::Id("func".to_string())))), tail: None }) }));
}

#[test]
fn test_invalid_var() {
    let mut parser = Parser::new("(nil)".to_string());
    assert_eq!(rules::var(&mut parser), None);
}

#[test]
fn test_var_recursive() {
    let mut parser = Parser::new("variable[nil].func".to_string());
    assert_eq!(rules::var(&mut parser),
        sexp!(expression::Expressions { head: exp!(variables::Id("variable".to_string())),
                                        tail: sexp!(expression::Expressions { head: exp!(tables::Indexing(exp!(primitives::Nil))),
                                                                              tail: sexp!(expression::Expressions { head: exp!(tables::Indexing(exp!(variables::Id("func".to_string())))),
                                                                                                                    tail: None }) }) }));

    parser = Parser::new("variable.func[nil]".to_string());
    assert_eq!(rules::var(&mut parser), sexp!(
        expression::Expressions { head: exp!(variables::Id("variable".to_string())),
                                  tail: sexp!(expression::Expressions { head: exp!(tables::Indexing(exp!(variables::Id("func".to_string())))),
                                                                        tail: sexp!(expression::Expressions { head: exp!(tables::Indexing(exp!(primitives::Nil))),
                                                                                                              tail: None }) }) }));

}

#[test]
fn test_varlist() {
    let mut parser = Parser::new("var1, var2, var3[nil].func".to_string());
    assert_eq!(rules::varlist(&mut parser),
        sexp!(variables::Varlist {
            head: exp!(expression::Expressions { head: exp!(variables::Id("var1".to_string())), tail: None }),
            tail: sexp!(variables::Varlist {
                head: exp!(expression::Expressions { head: exp!(variables::Id("var2".to_string())), tail: None }),
                tail: sexp!(variables::Varlist {
                    head: exp!(expression::Expressions {
                        head: exp!(variables::Id("var3".to_string())),
                        tail: sexp!(expression::Expressions {
                            head: exp!(tables::Indexing(exp!(primitives::Nil))),
                            tail: sexp!(expression::Expressions { head: exp!(tables::Indexing(exp!(variables::Id("func".to_string())))), tail: None }) }) }),
                    tail: None }) }) }));
}
