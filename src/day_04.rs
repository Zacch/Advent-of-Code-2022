use std::fs;

use crate::traits::RangeExtensions;
use crate::traits::StringExtensions;

/*
    Just to be clear, this version is very different from my first solution ;).
    I wrote the traits afterwards, both to learn how to do it and to clean up the code.
 */

pub fn day_04() {
    let (mut part1, mut part2) = (0, 0);
    let contents = fs::read_to_string("input/day_04.txt")
        .expect("Couldn't read the file");

    for line in contents.lines() {
        let ints = line.to_int_vector();
        let (r1, r2) = (ints[0] ..= ints[1], ints[2] ..= ints[3]);

        if r1.includes(&r2) || r2.includes(&r1) { part1 += 1; }
        if r1.overlaps(&r2) { part2 += 1; }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
