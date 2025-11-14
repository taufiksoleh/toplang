// Native Rust implementation of primes benchmark

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let mut count = 0;
    for i in 2..10000 {
        if is_prime(i) {
            count += 1;
        }
    }
    println!("{}", count);
}
