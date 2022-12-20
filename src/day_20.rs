use std::fs;

const MAX_LENGTH: usize = 5000;

pub fn run() {
    println!("Day 20");
    let contents = fs::read_to_string("input/day_20.txt").expect("Couldn't read the file");
    let mut encrypted:Vec<i64> = vec![];
    for line in contents.lines() {
        if line.is_empty() { continue; }
        encrypted.push(line.parse::<i64>().expect(&(line.to_string() + " is not a number!")));
    }
    let length = encrypted.len() as i64;

    let decrypted = mix2(&encrypted, 1);
    println!("Part 1: {}", add_grove_coordinates(decrypted, length));

    let encrypted2: Vec<i64> = encrypted.iter().map(|n| *n * 811589153).collect();
    let decrypted2 = mix2(&encrypted2, 10);
    println!("Part 2: {}", add_grove_coordinates(decrypted2, length));
}

fn mix2(encrypted: &[i64], shuffles: usize) -> [(usize, i64); MAX_LENGTH] {
    let length = encrypted.len();
    if length as usize > MAX_LENGTH { panic!("Vector is too long!") }

    let mut array: [(usize, i64); MAX_LENGTH] = [(0, 0); MAX_LENGTH];
    for (i, x) in encrypted.iter().enumerate() {
        array[i] = (i, *x);
    }

    for _shuffle in 0..shuffles {
        for i in 0..length {
            let before = array.iter().position(|&n| n.0 == i).unwrap();
            let (_i, number) = array[before];
            let number_of_shifts = number % (length as i64 - 1);
            let mut current_index = before;

            if number_of_shifts > 0 {
                for _ in 0..number_of_shifts {
                    if current_index == length - 1 {
                        let last_element = array[current_index];
                        for j in (1..length).rev() {
                            array[j] = array[j - 1];
                        }
                        current_index = 0;
                        array[current_index] = last_element;
                    }
                    array[current_index] = array[current_index + 1];
                    current_index += 1;
                }
                array[current_index] = (i, number);
            }
            if number_of_shifts < 0 {
                for _ in 0..-number_of_shifts {
                    if current_index == 0 {
                        let first_element = array[current_index];
                        for j in 0..length - 1 { array[j] = array[j + 1]; }
                        current_index = length - 1;
                        array[current_index] = first_element;
                    }
                    array[current_index] = array[current_index - 1];
                    current_index -= 1;
                }
                array[current_index] = (i, number);
            }
        }
    }
    array
}

fn add_grove_coordinates(decrypted: [(usize, i64); MAX_LENGTH], length: i64) -> i64 {
    let position = decrypted.iter().position(|n| n.1 == 0).unwrap();
    let mut result = 0;
    for i in 1..=3 {
        let new_position = (position + 1000 * i) as i64 % length;
        result += decrypted[new_position as usize].1;
    }
    result
}
