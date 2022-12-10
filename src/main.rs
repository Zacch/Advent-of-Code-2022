extern crate core;

use std::time::Instant;

mod day_10;
mod traits;
//mod tree_node;

fn main() {
    let now = Instant::now();

    day_10::day_10();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
