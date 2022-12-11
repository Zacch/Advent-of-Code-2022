use std::fs;

pub fn run() {
    println!("Day 1");
    let contents = fs::read_to_string("input/day_01.txt")
        .expect("Couldn't read the file");

    let mut elf_calories: Vec<i32> = Vec::new();
    let mut calories = 0;
    for line in contents.lines() {
        if line.len() > 0 {
            let number: i32 = line.parse().expect(&*format!("{} is not a number!", line));
            calories += number;
        } else {
            elf_calories.push(calories);
            calories = 0;
        }
    }
    if calories > 0 {
        elf_calories.push(calories);
    }
    elf_calories.sort();
    elf_calories.reverse();
    println!("Part 1: {}", elf_calories[0]);
    println!("Part 2: {}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
}
