extern crate core;

use std::time::Instant;

mod day_08;
mod traits;
//mod tree_node;

fn main() {
    let now = Instant::now();

    day_08::day_08();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
