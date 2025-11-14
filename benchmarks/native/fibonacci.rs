// Native Rust implementation of fibonacci benchmark
use std::time::Instant;

fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let result = fibonacci(35);
    println!("{}", result);
}
