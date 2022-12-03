use std::time::Instant;

mod day_03;

fn main() {
    let now = Instant::now();

    day_03::day_03();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
