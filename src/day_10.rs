use std::fs;

use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 10");
    let contents = fs::read_to_string("input/day_10.txt")
        .expect("Couldn't read the file");
    let lines:Vec<&str> = Vec::from_iter(contents.lines());
    let mut x: i32 = 1;
    let mut cycle = 1;
    let mut part1 = 0;
    let mut part2 = "".to_string();

    for line in lines {
        let tokens = line.tokens();
        match tokens[0] {
            "noop" => { tick(x, &mut cycle, &mut part1, &mut part2)},
            "addx" => {
                tick(x, &mut cycle, &mut part1, &mut part2);
                tick(x, &mut cycle, &mut part1, &mut part2);
                x += tokens[1].parse::<i32>().expect(&*format!("{} is not a number!", line));
            },
            _ => println!("Unknown token {}", tokens[0])
        }
    }
    tick(x, &mut cycle, &mut part1, &mut part2);
    println!("Part 1: {}", part1);
    let mut i = 0;
    println!("Part 2:");
    while i + 40 <= part2.len() {
        println!("{}", &part2[i..(i + 40)]);
        i += 40;
    }
}

fn tick(x: i32, cycle: &mut i32, part1: &mut i32, part2: &mut String) {

    let mut pixel = " ";
    if ((*cycle - 1) % 40 - x).abs() < 2 {
        pixel = "#";
    }
    *part2 = part2.to_owned() + pixel;

    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => {
            let signal_strength = x * *cycle;
            *part1 += signal_strength;
        }
        _ => ()
    }
    *cycle += 1;
}
