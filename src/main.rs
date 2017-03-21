pub mod utils;
pub mod ast;
use std::time::Instant;

use ast::parser;

// To avoid warnings in tests
#[allow(dead_code, unused_variables)]
fn main() {
    let start = Instant::now();

    let _ = parser::Parser::new("
      function gen (n)
        return coroutine.wrap(function ()
          for i=2,n do coroutine.yield(i) end
        end)
      end"
            .to_owned())
        .create_AST();

    let elapsed = Instant::now() - start;
    println!("Parsed input in {}.{:09} seconds",
             elapsed.as_secs(),
             elapsed.subsec_nanos());
}
