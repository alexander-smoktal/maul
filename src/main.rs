// #![feature(trace_macros)]

pub mod utils;
#[macro_use]
pub mod ast;
#[macro_use]
pub mod interpreter;
pub mod error;

#[cfg(test)]
mod test;

use std::time::Instant;

// To avoid warnings in tests
#[allow(dead_code, unused_variables)]
fn main() {
    let start = Instant::now();

    // let ast = ast::AST::new(
    //     "
    //   function a.b:fib(n)
    //     N=N+1
    //     if n<2 then
    //       return n
    //     else
    //       return a.b.fib(n-1) + a.b.fib(n-2)
    //     end
    //   end".to_owned(),
    // );

    let ast = ast::AST::new("y = 3 for i = 0, 10000000 do y = y + i end".to_string());

    print!("AST {:?}", ast);

    let mut elapsed = Instant::now() - start;
    println!(
        "Parsed input in {}.{:09} seconds",
        elapsed.as_secs(),
        elapsed.subsec_nanos()
    );

    ast.eval();

    elapsed = Instant::now() - start;
    println!(
        "Evaluated in {}.{:09} seconds",
        elapsed.as_secs(),
        elapsed.subsec_nanos()
    );
}
