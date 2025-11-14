// Native Rust implementation of array_sum benchmark

fn main() {
    let mut arr = Vec::new();
    for i in 0..10000 {
        arr.push(i);
    }

    let mut sum = 0;
    for _ in 0..100 {
        for &num in &arr {
            sum += num;
        }
    }

    println!("{}", sum);
}
