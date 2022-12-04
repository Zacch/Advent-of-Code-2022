use std::time::Instant;

mod day_04;
mod traits;

fn main() {
    let now = Instant::now();

    day_04::day_04();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
