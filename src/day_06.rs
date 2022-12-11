use std::fs;

pub fn run() {
    println!("Day 6");
    let contents = fs::read_to_string("input/day_06.txt")
        .expect("Couldn't read the file");
    let line:Vec<char> = contents.trim().chars().collect();

    println!("Part 1: {}", find_marker(&line, 4));
    println!("Part 2: {}", find_marker(&line, 14));
}

fn find_marker(line: &Vec<char>, window_size: usize) -> usize {
    'outer: for i in (window_size - 1)..line.len() {
        let mut window: Vec<char> = Vec::new();
        for j in 0..window_size {
            if window.contains(&line[i - j]) { continue 'outer; }
            window.push(line[i - j]);
        }
        return i + 1;
    }
    99999999
}
