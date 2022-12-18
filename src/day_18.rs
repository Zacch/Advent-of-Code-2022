use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use crate::points::{CubeFace, Direction, Point3};
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
    let face = CubeFace::new(current_cube, Forward);

    println!("{:?}", current_cube);
    println!("{:?}", face);

    let mut faces: Vec<CubeFace> = vec![face];

    let mut frontier: VecDeque<CubeFace> = VecDeque::new();
    frontier.push_back(face);
    let mut visited: HashSet<CubeFace> = HashSet::new();

    while !frontier.is_empty() {
     //   println!("-------------------------");
        let current = frontier.pop_front().unwrap();
       // println!("current   {:?}", current);
        let neighbors = neighbors_of(current);
        for next in neighbors {
            if visited.contains(&next) { continue; }
            if !cubes.contains_key(&next.cube.to_string()) { continue; }
            if cubes.contains_key(&next.cube.go(face.direction).to_string()) { continue; }

       //     println!("Adding {:?}", next);
            faces.push(next);
            frontier.push_back(next);
        }
        println!("Inserting {:?}", current);
        visited.insert(current);
    }

    visited.len()
}

//noinspection DuplicatedCode
fn neighbors_of(face: CubeFace) -> Vec<CubeFace> {

    let facing_cube = face.cube.go(face.direction);
    match face.direction {
        Up => {
            vec![
                CubeFace::new(facing_cube, Down),
                CubeFace::new(face.cube, Left),
                CubeFace::new(facing_cube, Left),
                CubeFace::new(face.cube.left(), Up),
                CubeFace::new(face.cube, Right),
                CubeFace::new(facing_cube, Right),
                CubeFace::new(face.cube.right(), Up),
                CubeFace::new(face.cube, Forward),
                CubeFace::new(facing_cube, Forward),
                CubeFace::new(face.cube.forward(), Up),
                CubeFace::new(face.cube, Back),
                CubeFace::new(facing_cube, Back),
                CubeFace::new(face.cube.back(), Up),
        ]},
        Down => {
            vec![
                CubeFace::new(facing_cube, Up),
                CubeFace::new(face.cube, Left),
                CubeFace::new(facing_cube, Left),
                CubeFace::new(face.cube.left(), Down),
                CubeFace::new(face.cube, Right),
                CubeFace::new(facing_cube, Right),
                CubeFace::new(face.cube.right(), Down),
                CubeFace::new(face.cube, Forward),
                CubeFace::new(facing_cube, Forward),
                CubeFace::new(face.cube.forward(), Down),
                CubeFace::new(face.cube, Back),
                CubeFace::new(facing_cube, Back),
                CubeFace::new(face.cube.back(), Down),
            ]},
        Left => {
            vec![
                CubeFace::new(facing_cube, Right),
                CubeFace::new(face.cube, Up),
                CubeFace::new(facing_cube, Up),
                CubeFace::new(face.cube.up(), Left),
                CubeFace::new(face.cube, Down),
                CubeFace::new(facing_cube, Down),
                CubeFace::new(face.cube.down(), Left),
                CubeFace::new(face.cube, Forward),
                CubeFace::new(facing_cube, Forward),
                CubeFace::new(face.cube.forward(), Left),
                CubeFace::new(face.cube, Back),
                CubeFace::new(facing_cube, Back),
                CubeFace::new(face.cube.back(), Left),
            ]},
        Right => {
            vec![
                CubeFace::new(facing_cube, Left),
                CubeFace::new(face.cube, Up),
                CubeFace::new(facing_cube, Up),
                CubeFace::new(face.cube.up(), Right),
                CubeFace::new(face.cube, Down),
                CubeFace::new(facing_cube, Down),
                CubeFace::new(face.cube.down(), Right),
                CubeFace::new(face.cube, Forward),
                CubeFace::new(facing_cube, Forward),
                CubeFace::new(face.cube.forward(), Right),
                CubeFace::new(face.cube, Back),
                CubeFace::new(facing_cube, Back),
                CubeFace::new(face.cube.back(), Right),
            ]},
        Forward => {
            vec![
                CubeFace::new(facing_cube, Back),
                CubeFace::new(face.cube, Left),
                CubeFace::new(facing_cube, Left),
                CubeFace::new(face.cube.left(), Forward),
                CubeFace::new(face.cube, Right),
                CubeFace::new(facing_cube, Right),
                CubeFace::new(face.cube.right(), Forward),
                CubeFace::new(face.cube, Up),
                CubeFace::new(facing_cube, Up),
                CubeFace::new(face.cube.up(), Forward),
                CubeFace::new(face.cube, Down),
                CubeFace::new(facing_cube, Down),
                CubeFace::new(face.cube.down(), Forward),
            ]},
        Back => {
            vec![
                CubeFace::new(facing_cube, Forward),
                CubeFace::new(face.cube, Left),
                CubeFace::new(facing_cube, Left),
                CubeFace::new(face.cube.left(), Back),
                CubeFace::new(face.cube, Right),
                CubeFace::new(facing_cube, Right),
                CubeFace::new(face.cube.right(), Back),
                CubeFace::new(face.cube, Up),
                CubeFace::new(facing_cube, Up),
                CubeFace::new(face.cube.up(), Back),
                CubeFace::new(face.cube, Down),
                CubeFace::new(facing_cube, Down),
                CubeFace::new(face.cube.down(), Back),
            ]},
    }
}
