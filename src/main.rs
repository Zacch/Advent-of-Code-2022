use std::time::Instant;

mod day_06;
mod traits;

fn main() {
    let now = Instant::now();

    day_06::day_06();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
