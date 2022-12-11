use std::fs;

pub fn run() {
    println!("Day 2");
    let contents = fs::read_to_string("input/day_02.txt")
        .expect("Couldn't read the file");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in contents.lines() {
        part1 += score(line);
        part2 += score(translate(line));
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
// Opponent's move: A for Rock, B for Paper, and C for Scissors
// Your move: X for Rock, Y for Paper, and Z for Scissors
// Score: 1 for Rock, 2 for Paper, and 3 for Scissors plus
//        0 if you lost, 3 if the round was a draw, and 6 if you won
fn score(line: &str) -> i32 {
    match line {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => 9999999
    }
}

// Opponent's move: A for Rock, B for Paper, and C for Scissors
// Your move: X for Rock, Y for Paper, and Z for Scissors
// X means you need to lose,
// Y means you need to end the round in a draw,
// and Z means you need to win.
fn translate(line: &str) -> &str {
    match line {
        "A X" => "A Z",
        "A Y" => "A X",
        "A Z" => "A Y",
        "B X" => "B X",
        "B Y" => "B Y",
        "B Z" => "B Z",
        "C X" => "C Y",
        "C Y" => "C Z",
        "C Z" => "C X",
        _ => "XXXXX"
    }
}
