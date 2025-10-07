use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut sum: i64 = 0;
    for i in 0..100_000_000 {
        sum += i;
    }

    let duration = start.elapsed();
    println!("Rust sum = {}", sum);
    println!("Rust time: {:.4} sec", duration.as_secs_f64());
}
