use ast::lexer::Lexer;

use ast::expressions::function;


#[test]
#[should_panic]
fn test_empty_function() {
    let mut lexer = Lexer::new("".to_owned());

    function::Function::from_lexer(&mut lexer);
}

//      function f () body end
// translates to

//      f = function () body end
// The statement

//      function t.a.b.c.f () body end
// translates to

//      t.a.b.c.f = function () body end
// The statement

//      local function f () body end
// translates to

//      local f; f = function () body end