use std::fs;

use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 9");
    let contents = fs::read_to_string("input/day_09.txt")
        .expect("Couldn't read the file");
    let lines:Vec<&str> = Vec::from_iter(contents.lines());

    let mut parts: Vec<(i32, i32)> = vec![(0, 0), (0, 0)];
    let mut part2_parts: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut positions: Vec<(i32, i32)> = vec![(0, 0)];
    let mut part2_positions: Vec<(i32, i32)> = vec![(0, 0)];

    for line in lines {
        let tokens = line.tokens();
        let distance = tokens[1].parse().expect(&*format!("{} is not a number!", line));
        match tokens[0] {
            "R" => move_head((0, 1), distance, &mut parts, &mut positions),
            "L" => move_head((0, -1), distance, &mut parts, &mut positions),
            "U" => move_head((1, 0), distance, &mut parts, &mut positions),
            "D" => move_head((-1, 0), distance, &mut parts, &mut positions),
            _ => panic!()
        }
        match tokens[0] {
            "R" => move_head((0, 1), distance, &mut part2_parts, &mut part2_positions),
            "L" => move_head((0, -1), distance, &mut part2_parts, &mut part2_positions),
            "U" => move_head((1, 0), distance, &mut part2_parts, &mut part2_positions),
            "D" => move_head((-1, 0), distance, &mut part2_parts, &mut part2_positions),
            _ => ()
        }
    }
    println!("Part 1: {}", positions.len());
    println!("Part 2: {}", part2_positions.len());
}

fn move_head(direction: (i32, i32), distance: i32, parts: &mut Vec<(i32, i32)>, positions: &mut Vec<(i32, i32)>) {
    for _ in 0..distance {
        parts[0].0 += direction.0;
        parts[0].1 += direction.1;

        for i in 1..parts.len() {
            parts[i] = follow(parts[i - 1], parts[i]);
        }


        let tail = parts[parts.len() - 1];
       // println!("H ({}, {}), T  ({}, {})", head.0, head.1, tail.0, tail.1);
        if !positions.iter().any(|p| p.0 == tail.0 && p.1 == tail.1) {
            positions.push(tail.clone());
        }
    }
}

fn follow(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let mut result = tail;
    if result.0 < head.0 - 1 {
        result.0 += 1;
        if head.1 > result.1 {result.1 += 1;}
        if head.1 < result.1 {result.1 -= 1;}
    }

    if result.0 > head.0 + 1 {
        result.0 -= 1;
        if head.1 > result.1 {result.1 += 1;}
        if head.1 < result.1 {result.1 -= 1;}
    }

    if result.1 < head.1 - 1 {
        result.1 += 1;
        if head.0 > result.0 {result.0 += 1;}
        if head.0 < result.0 {result.0 -= 1;}
    }

    if result.1 > head.1 + 1 {
        result.1 -= 1;
        if head.0 > result.0 {result.0 += 1;}
        if head.0 < result.0 {result.0 -= 1;}
    }
    return result;
}
