use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use crate::points::{CubeFace, Point3};
use crate::points::Direction::*;
use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 18");
    let contents = fs::read_to_string("input/day_18.txt").expect("File missing");

    let mut cubes: HashMap<String, Point3> = HashMap::new();
    for line in contents.lines() {
        let cube = Point3::from_vector(line.to_int_vector());
        cubes.insert(cube.to_string(), cube);
    }

    let mut part1 = 0;
    for cube in cubes.values() {
        if !cubes.contains_key(&cube.left().to_string()) { part1 += 1 }
        if !cubes.contains_key(&cube.right().to_string()) { part1 += 1 }
        if !cubes.contains_key(&cube.up().to_string()) { part1 += 1 }
        if !cubes.contains_key(&cube.down().to_string()) { part1 += 1 }
        if !cubes.contains_key(&cube.forward().to_string()) { part1 += 1 }
        if !cubes.contains_key(&cube.back().to_string()) { part1 += 1 }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2(&cubes));
}

fn part2(cubes: &HashMap<String, Point3>) -> usize {
    let max_z = cubes.values().map(|p| p.z).max().unwrap();
    let first_cube = cubes.values().next().unwrap();
    let mut current_cube = Point3::new(first_cube.x, first_cube.y, max_z);
    while !cubes.contains_key(&current_cube.to_string()) {
        current_cube = current_cube.back();
    }
    let start_face = CubeFace::new(current_cube, Forward);

    let mut frontier: VecDeque<CubeFace> = VecDeque::new();
    frontier.push_back(start_face);
    let mut visited: HashSet<CubeFace> = HashSet::new();

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        if visited.contains(&current) { continue; }
        let neighbors = neighbors_of(current, cubes);
        for next in neighbors {
            if visited.contains(&next) { continue; }
            frontier.push_back(next);
        }
        visited.insert(current);
    }

    visited.len()
}

//noinspection DuplicatedCode
fn neighbors_of(face: CubeFace, cubes: &HashMap<String, Point3>) -> Vec<CubeFace> {
    let cube = face.cube;
    let mut neighbours: Vec<CubeFace> = vec![];

    match face.direction {
        Forward => {
            if cubes.contains_key(&cube.right().forward().to_string()) {
                neighbours.push(CubeFace::new(cube.right().forward(), Left))
            } else if cubes.contains_key(&cube.right().to_string()) {
                neighbours.push(CubeFace::new(cube.right(), Forward))
            } else {
                neighbours.push(CubeFace::new(cube, Right))
            }

            if cubes.contains_key(&cube.left().forward().to_string()) {
                neighbours.push(CubeFace::new(cube.left().forward(), Right))
            } else if cubes.contains_key(&cube.left().to_string()) {
                neighbours.push(CubeFace::new(cube.left(), Forward))
            } else {
                neighbours.push(CubeFace::new(cube, Left))
            }

            if cubes.contains_key(&cube.up().forward().to_string()) {
                neighbours.push(CubeFace::new(cube.up().forward(), Down))
            } else if cubes.contains_key(&cube.up().to_string()) {
                neighbours.push(CubeFace::new(cube.up(), Forward))
            } else {
                neighbours.push(CubeFace::new(cube, Up))
            }

            if cubes.contains_key(&cube.down().forward().to_string()) {
                neighbours.push(CubeFace::new(cube.down().forward(), Up))
            } else if cubes.contains_key(&cube.down().to_string()) {
                neighbours.push(CubeFace::new(cube.down(), Forward))
            } else {
                neighbours.push(CubeFace::new(cube, Down))
            }
        },
        Back => {
            if cubes.contains_key(&cube.right().back().to_string()) {
                neighbours.push(CubeFace::new(cube.right().back(), Left))
            } else if cubes.contains_key(&cube.right().to_string()) {
                neighbours.push(CubeFace::new(cube.right(), Back))
            } else {
                neighbours.push(CubeFace::new(cube, Right))
            }

            if cubes.contains_key(&cube.left().back().to_string()) {
                neighbours.push(CubeFace::new(cube.left().back(), Right))
            } else if cubes.contains_key(&cube.left().to_string()) {
                neighbours.push(CubeFace::new(cube.left(), Back))
            } else {
                neighbours.push(CubeFace::new(cube, Left))
            }

            if cubes.contains_key(&cube.up().back().to_string()) {
                neighbours.push(CubeFace::new(cube.up().back(), Down))
            } else if cubes.contains_key(&cube.up().to_string()) {
                neighbours.push(CubeFace::new(cube.up(), Back))
            } else {
                neighbours.push(CubeFace::new(cube, Up))
            }

            if cubes.contains_key(&cube.down().back().to_string()) {
                neighbours.push(CubeFace::new(cube.down().back(), Up))
            } else if cubes.contains_key(&cube.down().to_string()) {
                neighbours.push(CubeFace::new(cube.down(), Back))
            } else {
                neighbours.push(CubeFace::new(cube, Down))
            }
        }
        Left => {
            if cubes.contains_key(&cube.forward().left().to_string()) {
                neighbours.push(CubeFace::new(cube.forward().left(), Back))
            } else if cubes.contains_key(&cube.forward().to_string()) {
                neighbours.push(CubeFace::new(cube.forward(), Left))
            } else {
                neighbours.push(CubeFace::new(cube, Forward))
            }

            if cubes.contains_key(&cube.back().left().to_string()) {
                neighbours.push(CubeFace::new(cube.back().left(), Forward))
            } else if cubes.contains_key(&cube.back().to_string()) {
                neighbours.push(CubeFace::new(cube.back(), Left))
            } else {
                neighbours.push(CubeFace::new(cube, Back))
            }

            if cubes.contains_key(&cube.up().left().to_string()) {
                neighbours.push(CubeFace::new(cube.up().left(), Down))
            } else if cubes.contains_key(&cube.up().to_string()) {
                neighbours.push(CubeFace::new(cube.up(), Left))
            } else {
                neighbours.push(CubeFace::new(cube, Up))
            }

            if cubes.contains_key(&cube.down().left().to_string()) {
                neighbours.push(CubeFace::new(cube.down().left(), Up))
            } else if cubes.contains_key(&cube.down().to_string()) {
                neighbours.push(CubeFace::new(cube.down(), Left))
            } else {
                neighbours.push(CubeFace::new(cube, Down))
            }
        }
        Right => {
            if cubes.contains_key(&cube.forward().right().to_string()) {
                neighbours.push(CubeFace::new(cube.forward().right(), Back))
            } else if cubes.contains_key(&cube.forward().to_string()) {
                neighbours.push(CubeFace::new(cube.forward(), Right))
            } else {
                neighbours.push(CubeFace::new(cube, Forward))
            }

            if cubes.contains_key(&cube.back().right().to_string()) {
                neighbours.push(CubeFace::new(cube.back().right(), Forward))
            } else if cubes.contains_key(&cube.back().to_string()) {
                neighbours.push(CubeFace::new(cube.back(), Right))
            } else {
                neighbours.push(CubeFace::new(cube, Back))
            }

            if cubes.contains_key(&cube.up().right().to_string()) {
                neighbours.push(CubeFace::new(cube.up().right(), Down))
            } else if cubes.contains_key(&cube.up().to_string()) {
                neighbours.push(CubeFace::new(cube.up(), Right))
            } else {
                neighbours.push(CubeFace::new(cube, Up))
            }

            if cubes.contains_key(&cube.down().right().to_string()) {
                neighbours.push(CubeFace::new(cube.down().right(), Up))
            } else if cubes.contains_key(&cube.down().to_string()) {
                neighbours.push(CubeFace::new(cube.down(), Right))
            } else {
                neighbours.push(CubeFace::new(cube, Down))
            }
        }
        Up => {
            if cubes.contains_key(&cube.forward().up().to_string()) {
                neighbours.push(CubeFace::new(cube.forward().up(), Back))
            } else if cubes.contains_key(&cube.forward().to_string()) {
                neighbours.push(CubeFace::new(cube.forward(), Up))
            } else {
                neighbours.push(CubeFace::new(cube, Forward))
            }

            if cubes.contains_key(&cube.back().up().to_string()) {
                neighbours.push(CubeFace::new(cube.back().up(), Forward))
            } else if cubes.contains_key(&cube.back().to_string()) {
                neighbours.push(CubeFace::new(cube.back(), Up))
            } else {
                neighbours.push(CubeFace::new(cube, Back))
            }

            if cubes.contains_key(&cube.right().up().to_string()) {
                neighbours.push(CubeFace::new(cube.right().up(), Left))
            } else if cubes.contains_key(&cube.right().to_string()) {
                neighbours.push(CubeFace::new(cube.right(), Up))
            } else {
                neighbours.push(CubeFace::new(cube, Right))
            }

            if cubes.contains_key(&cube.left().up().to_string()) {
                neighbours.push(CubeFace::new(cube.left().up(), Right))
            } else if cubes.contains_key(&cube.left().to_string()) {
                neighbours.push(CubeFace::new(cube.left(), Up))
            } else {
                neighbours.push(CubeFace::new(cube, Left))
            }
        }
        Down => {
            if cubes.contains_key(&cube.forward().down().to_string()) {
                neighbours.push(CubeFace::new(cube.forward().down(), Back))
            } else if cubes.contains_key(&cube.forward().to_string()) {
                neighbours.push(CubeFace::new(cube.forward(), Down))
            } else {
                neighbours.push(CubeFace::new(cube, Forward))
            }

            if cubes.contains_key(&cube.back().down().to_string()) {
                neighbours.push(CubeFace::new(cube.back().down(), Forward))
            } else if cubes.contains_key(&cube.back().to_string()) {
                neighbours.push(CubeFace::new(cube.back(), Down))
            } else {
                neighbours.push(CubeFace::new(cube, Back))
            }

            if cubes.contains_key(&cube.right().down().to_string()) {
                neighbours.push(CubeFace::new(cube.right().down(), Left))
            } else if cubes.contains_key(&cube.right().to_string()) {
                neighbours.push(CubeFace::new(cube.right(), Down))
            } else {
                neighbours.push(CubeFace::new(cube, Right))
            }

            if cubes.contains_key(&cube.left().down().to_string()) {
                neighbours.push(CubeFace::new(cube.left().down(), Right))
            } else if cubes.contains_key(&cube.left().to_string()) {
                neighbours.push(CubeFace::new(cube.left(), Down))
            } else {
                neighbours.push(CubeFace::new(cube, Left))
            }
        }
    }
    neighbours
}
