use std::collections::VecDeque;
use std::fs;

use crate::points::Point;
use crate::utilities::Rect;

pub fn run() {
    println!("Day 12");
    let lowest_elevation = 'a'.to_digit(36).unwrap();
    let highest_elevation = 'z'.to_digit(36).unwrap();

    let contents = fs::read_to_string("input/day_12.txt").expect("Couldn't read the file");

    let lines: Vec<&str> = Vec::from_iter(contents.lines());

    let mut grid: Vec<Vec<u32>> = vec![];
    let mut start = Point::origin();
    let mut end = Point::origin();

    let (mut x, mut y) = (0, 0);
    for line in lines.iter() {
        let mut row: Vec<u32> = vec![];
        for c in line.chars() {
            match c {
                'S' => {
                    start = Point { x, y };
                    row.push(lowest_elevation);
                },
                'E' => {
                    end = Point { x, y };
                    row.push(highest_elevation);
                },
                _ => row.push(c.to_digit(36).unwrap()),
            }
            x += 1;
        }
        grid.push(row);
        y += 1;
        x = 0;
    }
    let bounds = Rect { left: 0, top: lines.len() as i32, bottom: 0, right: lines[0].len() as i32 };

    let mut visited: Vec<(Point, i32)> = vec![];
    let mut frontier: VecDeque<(Point, i32)> = VecDeque::new();
    frontier.push_back((start, 0));

    'part1: loop {
        let (current, distance) = frontier.pop_front().unwrap();
        if visited.iter().any(|(p, d)| *p == current && *d <= distance) { continue; }

        let neighbours =
            vec![current.left(), current.right(), current.up(), current.down()];
        for next in neighbours {
            if !bounds.contains(next) { continue; }
            if visited.iter().any(|(p, d)| *p == next && *d <= distance + 1) { continue; }
            if height(&grid, next) > height(&grid, current) + 1 { continue; }
            if next == end {
                println!("Part 1: {}", distance + 1);
                break 'part1;
            }

            frontier.push_back((next, distance + 1));
        }
        visited.push((current, distance));
    }
    visited = vec![];
    frontier = VecDeque::new();
    frontier.push_back((end, 0));

    'part2: loop {
        let (current, distance) = frontier.pop_front().unwrap();
        if visited.iter().any(|(p, d)| *p == current && *d <= distance) { continue; }

        let neighbours =
            vec![current.left(), current.right(), current.up(), current.down()];
        for next in neighbours {
            if !bounds.contains(next) { continue; }
            if visited.iter().any(|(p, d)| *p == next && *d <= distance + 1) { continue; }
            if height(&grid, next) < height(&grid, current) - 1 { continue; }
            if height(&grid, next) == lowest_elevation {
                println!("Part 2: {}", distance + 1);
                break 'part2;
            }

            frontier.push_back((next, distance + 1));
        }
        visited.push((current, distance));
    }
}

fn height(grid: &[Vec<u32>], p: Point) -> u32 {
    grid[p.y as usize][p.x as usize]
}
