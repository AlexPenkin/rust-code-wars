use rand::Rng;
use sums::sum_pairs;

fn main() {
    use std::time::Instant;
    let mut rng = rand::thread_rng();
    let v: Vec<i8> = (0..2000000).map(|_| rng.gen_range(-33..=33)).collect();
    let now = Instant::now();
    let (first, last) = match sum_pairs(&v, 32) {
        None => (String::from("Not found"), String::from("Not found")),
        Some((first, second)) => (first.to_string(), second.to_string()),
    };
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Result: {} {}", first, last);
}
