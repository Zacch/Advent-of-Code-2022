use std::fs;

pub fn run() {
    println!("Day 3");
    let contents = fs::read_to_string("input/day_03.txt")
        .expect("Couldn't read the file");

    let mut part1 = 0;
    let lines:Vec<&str> = Vec::from_iter(contents.lines());
    for line in &lines {
        let (c1, c2) = line.split_at(line.len() / 2);
        for c in c1.chars() {
            if c2.chars().any(|ch| ch == c) {
                part1 += value_of(c);
                break;
            }
        }
    }
    println!("Part 1: {}", part1);

    let mut part2 = 0;
    let mut line_no = 0;
    while line_no < lines.len() {
        for c in lines[line_no].chars() {
            if lines[line_no + 1].chars().any(|ch| ch == c)  && lines[line_no + 2].chars().any(|ch| ch == c) {
                part2 += value_of(c);
                break;
            }
        }
        line_no += 3;
    }
    println!("Part 2: {}", part2);
}

fn value_of(c: char) -> u32 {
    if c as u32 >= 'a' as u32 {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
}
