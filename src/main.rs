extern crate core;

use std::time::Instant;

mod day_09;
mod traits;
//mod tree_node;

fn main() {
    let now = Instant::now();

    day_09::day_09();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
