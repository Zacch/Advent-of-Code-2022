use std::time::Instant;

mod day_05;
mod traits;

fn main() {
    let now = Instant::now();

    day_05::day_05();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
