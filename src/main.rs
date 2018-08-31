#![feature(trace_macros)]
#![feature(fn_traits)]
#![feature(box_patterns)]

pub mod utils;
#[macro_use]
pub mod ast;
pub mod error;

#[cfg(test)]
mod test;

use std::time::Instant;

// To avoid warnings in tests
#[allow(dead_code, unused_variables)]
fn main() {
    let start = Instant::now();

    let ast = ast::AST::new(
        "
      function a.b:fib(n)
        N=N+1
        if n<2 then
          return n
        else
          return a.b.fib(n-1) + a.b.fib(n-2)
        end
      end"
            .to_owned(),
    );

    print!("AST {:?}", ast);

    let elapsed = Instant::now() - start;
    println!(
        "Parsed input in {}.{:09} seconds",
        elapsed.as_secs(),
        elapsed.subsec_nanos()
    );
}
