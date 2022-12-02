use std::time::Instant;

mod day_01;

fn main() {
    let now = Instant::now();

    day_01::day_01();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
