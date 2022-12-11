extern crate core;

use std::time::Instant;

use advent_of_code_2022::*;

fn main() {
    let now = Instant::now();

    day_01::run();
    day_02::run();
    day_03::run();
    day_04::run();
    day_05::run();
    day_06::run();
    day_07::run();
    day_08::run();
    day_09::run();
    day_10::run();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
