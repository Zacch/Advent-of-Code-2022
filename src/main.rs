use std::time::Instant;

mod day_02;

fn main() {
    let now = Instant::now();

    day_02::day_02();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
