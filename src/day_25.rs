use std::fs;

pub fn run() {
    println!("Day 25");
    let contents = fs::read_to_string("input/day_25.txt")
        .expect("Couldn't read the file");

    let mut sum = 0;
    for line in contents.lines() {

        let mut factor = 1;
        let mut number = 0;
        for c in line.chars().rev() {
            let int = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!()
            };
            number += int * factor;
            factor *= 5;
        }
        sum += number
    }

    println!("Part 1: {}", encode(sum));
}

fn encode(number: i64) -> String {
    let mut remaining = number;
    let mut decoded: Vec<char> = vec![];
    while remaining > 0 {
        let mut remainder = remaining % 5;
        if remainder > 2 { remainder -= 5}

        let c = match remainder {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => panic!()
        };
        decoded.push(c);

        remaining -= remainder;
        remaining /= 5;
    }
    decoded.into_iter().rev().collect()
}
