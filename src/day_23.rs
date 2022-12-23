use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry::Vacant;
use std::fs;

use crate::points::Point;
use crate::utilities::Rect;

enum Direction { N, S, W, E }

pub fn run() {
    println!("Day 23");
    let contents = fs::read_to_string("input/day_23.txt").expect("Couldn't read the file");
    let lines:Vec<&str> = contents.lines().collect();

    let mut elves: HashSet<Point> = HashSet::new();

    let mut y = (lines.len() - 1) as i32;
    for line in lines {
        for (x, c) in line.chars().enumerate() {
            if c == '#' { elves.insert(Point { x: x as i32, y}); }
        }
        y -= 1
    }

    let mut directions: VecDeque<Direction> = VecDeque::new();
    directions.push_back(Direction::N);
    directions.push_back(Direction::S);
    directions.push_back(Direction::W);
    directions.push_back(Direction::E);

    let mut round = 1;
    let mut elf_moved = true;
    while elf_moved {
        elf_moved = false;
        let mut proposed_moves: HashMap<Point, Vec<Point>> = HashMap::new(); // Destination is key, elf is value

        for elf in &elves {
            if unoccupied(vec![elf.n(), elf.s(), elf.w(), elf.e(), elf.ne(), elf.nw(), elf.se(), elf.sw()], &elves) {
                add_or_insert(&mut proposed_moves, elf, elf);
                continue;
            }
            let mut proposed_move = *elf;

            for direction in &directions {
                match direction {
                    Direction::N => {
                        if unoccupied(vec![elf.n(), elf.ne(), elf.nw()], &elves) {
                            proposed_move = elf.n();
                            break;
                        }
                    },
                    Direction::S => {
                        if unoccupied(vec![elf.s(), elf.se(), elf.sw()], &elves) {
                            proposed_move = elf.s();
                            break;
                        }
                    },
                    Direction::W => {
                        if unoccupied(vec![elf.w(), elf.nw(), elf.sw()], &elves) {
                            proposed_move = elf.w();
                            break;
                        }
                    },
                    Direction::E => {
                        if unoccupied(vec![elf.e(), elf.ne(), elf.se()], &elves) {
                            proposed_move = elf.e();
                            break;
                        }
                    },
                };
            }
            add_or_insert(&mut proposed_moves, elf, &proposed_move)
        }

        elves.drain();

        for (destination, proposers) in proposed_moves {
            if proposers.len() == 1 {
                elves.insert(destination);
                if destination != proposers[0] { elf_moved = true; }
            } else {
                for proposer in proposers {
                    elves.insert(proposer);
                }
            }
        }

        if round == 10 {
            println!("Part 1: {}", empty_tiles(&elves));
        }

        let first_direction = directions.pop_front().unwrap();
        directions.push_back(first_direction);
        round += 1;
    }

    println!("Part 2: {}", round - 1);
}

#[allow(dead_code)]
fn draw(elves: &HashSet<Point>) {
    let bounds: Rect = Rect {
        left: elves.iter().map(|e| e.x).min().unwrap(),
        top: elves.iter().map(|e| e.y).max().unwrap(),
        bottom: elves.iter().map(|e| e.y).min().unwrap(),
        right: elves.iter().map(|e| e.x).max().unwrap(),
    };

    for y in (bounds.bottom..=bounds.top).rev() {
        let mut line: Vec<char> = vec![' '];
        for x in bounds.left..=bounds.right {
            if elves.contains(&Point{ x, y}) { line.push('#'); }
            else { line.push('.'); }
        }
        println!("{}", line.into_iter().collect::<String>());
    }
    println!("There are {:?} empty ground tiles",
             (bounds.right + 1 - bounds.left) * (bounds.top + 1 - bounds.bottom) - elves.len() as i32);
}

fn empty_tiles(elves: &HashSet<Point>) -> i32 {
    (elves.iter().map(|e| e.x).max().unwrap() + 1 - elves.iter().map(|e| e.x).min().unwrap()) *
        (elves.iter().map(|e| e.y).max().unwrap() + 1 - elves.iter().map(|e| e.y).min().unwrap())
        - elves.len() as i32
}

fn add_or_insert(proposed_moves: &mut HashMap<Point, Vec<Point>>, elf: &Point, proposed_move: &Point) {
    if let Vacant(e) = proposed_moves.entry(*proposed_move) {
        e.insert(vec![*elf]);
    } else {
        proposed_moves.get_mut(&proposed_move).unwrap().push(*elf);
    }
}

fn unoccupied(points: Vec<Point>, elves: &HashSet<Point>) -> bool {
    points.iter().all(|p| !elves.contains(p))
}
