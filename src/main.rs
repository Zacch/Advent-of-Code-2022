extern crate core;

use std::time::Instant;

mod day_07;
mod traits;
//mod tree_node;

fn main() {
    let now = Instant::now();

    day_07::day_07();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
