extern crate core;

use std::time::Instant;

use advent_of_code_2022::*;

fn main() {
    let now = Instant::now();
/*
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
    day_11::run();
    day_12::run();
    day_13::run();
    day_14::run();
    day_15::run();

    day_17::run();
    day_18::run();
    day_19::run();
    day_20::run();
    day_21::run();
    day_22::run();
    day_23::run();
    day_24::run();
    day_25::run();
*/
    day_16::run();

    println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
}
