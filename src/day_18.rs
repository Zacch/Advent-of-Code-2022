use std::fs;

use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 18");
    let contents = fs::read_to_string("input/day_18.txt").expect("File missing");

    for line in contents.lines() {
        let _coords = line.to_int_vector();
    }
}
