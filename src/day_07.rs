use std::cmp::{max, min};
use std::fs;

use substring::*;

use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 7");
    let contents = fs::read_to_string("input/day_07.txt")
        .expect("Couldn't read the file");

    let mut disk:Vec<Directory> = vec![];
    let mut current_path = "/".to_string();
    disk.push(Directory::new(current_path.to_owned()));
    for line in contents.lines() {
        let tokens = line.tokens();

        match tokens[0]  {
            "$" =>
                match tokens[1] {
                    "cd" => {
                        let mut new_path = current_path.to_owned();
                        match tokens[2] {
                            "/" => new_path = "/".to_string(),
                            ".." => {
                                let end = new_path.rfind('/').unwrap();
                                new_path = new_path.substring(0, end).to_string();
                                if new_path == "" { new_path = "/".to_string() }
                            },
                            _ => if new_path == "/" {
                                new_path = format!("/{}", tokens[2])
                            } else {
                                new_path = format!("{}/{}", new_path, tokens[2])
                            }
                        }

                        if !disk.iter().any(|dir| dir.path == new_path) {
                            disk.push(Directory::new(new_path.to_owned()));
                        }

                        current_path = new_path;
                    },
                    "ls" => (),
                    _ => panic!("Unknown command: \"{}\"", line)
                },
            "dir" => {
                let dir_path: String;
                if current_path == "/" {
                    dir_path = format!("/{}", tokens[1])
                } else {
                    dir_path = format!("{}/{}", current_path, tokens[1])
                }
                if !disk.iter().any(|dir| dir.path == dir_path) {
                    disk.push(Directory::new(dir_path.to_owned()));
                }
            },
            _ => {
                let file_size:usize = tokens[0].parse().expect(&*format!("{} is not a number!", tokens[0]));
                let mut current_dir = disk.iter_mut().filter(|dir| dir.path == current_path).nth(0).unwrap();

                loop {
                    current_dir.size += file_size;
                    if current_dir.path == "/" { break; }
                    let end = max(current_dir.path.rfind('/').unwrap(), 1);
                    let new_path = current_dir.path.substring(0, end).to_string();
                    current_dir = disk.iter_mut().filter(|dir| dir.path == new_path).nth(0).unwrap();
                }
            }
        }
    }
    let part1 = disk.iter()
        .filter(|d| d.size <= 100000)
        .map(|d| d.size)
        .fold(0 as usize, |accum, item| accum + item);
    println!("Part 1: {}", part1);

    let root_dir = disk.iter().nth(0).unwrap();
    let size_needed = root_dir.size - 40000000;
    let part2 = disk.iter()
        .filter(|d| d.size >= size_needed)
        .map(|d| d.size)
        .fold(70000000, |smallest, current| min(smallest, current));
    println!("Part 2: {}", part2);
}

#[derive(Debug)]
struct Directory {
    path: String,
    size: usize
}

impl Directory {
    fn new(path: String) -> Directory {
        Directory { path, size: 0 }
    }
}
