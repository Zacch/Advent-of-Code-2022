use std::fs;

use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 5");
    let contents = fs::read_to_string("input/day_05.txt")
        .expect("Couldn't read the file");
    let lines:Vec<&str> = Vec::from_iter(contents.lines());

    let mut separator_line = 0;
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            separator_line = i;
            break;
        }
    }

    let stack_count = (lines[separator_line - 1].len() + 2) / 4;
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut stacks2: Vec<Vec<char>> = vec![];
    for _ in 0..stack_count {
        stacks.push(vec![]);
        stacks2.push(vec![]);
    }

    for i in (0..separator_line - 1).rev() {
        let chars: Vec<char> = lines[i].chars().collect();
        for stack in 0..stack_count {
            let content_position = stack * 4 + 1;
            if content_position > chars.len() {
                break;
            }
            if chars[content_position] != ' ' {
                stacks[stack].push(chars[content_position]);
                stacks2[stack].push(chars[content_position]);
            }
        }
    }

    for i in separator_line + 1..lines.len() {
        let ints = lines[i].to_int_vector();
        let (count, from, to) =
            (ints[0] as usize, ints[1] as usize - 1, ints[2] as usize - 1);
        let stack2from = stacks2[from].len() - count;
        for _ in 0..count {
            let crate1 = stacks[from].pop().unwrap();
            stacks[to].push(crate1);
            let crate2 = stacks2[from].remove(stack2from);
            stacks2[to].push(crate2);
        }
    }

    let part1 = String::from_iter(stacks.into_iter().map(|mut s|s.pop().unwrap()));
    println!("Part 1: {}", part1);

    let part2 = String::from_iter(stacks2.into_iter().map(|mut s|s.pop().unwrap()));
    println!("Part 2: {}", part2);
}
