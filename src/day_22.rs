use std::collections::HashSet;
use std::fs;

use crate::points::Point;

pub fn run() {
    println!("Day 22");
    let contents = fs::read_to_string("input/day_22.txt").expect("Couldn't read the file");

    let mut rows: Vec<Point> = vec![Point::origin()]; // x = start, y = end, 1 based closed intervals
    let mut cols: Vec<Point> = vec![Point::origin()];
    let mut walls: HashSet<Point> = HashSet::new();

    let lines:Vec<&str> = contents.lines().collect();
    for (r, line) in lines.iter().enumerate() {
        if line.is_empty() { break; }
        let mut row = Point::origin();
        for (column_index, c) in line.chars().enumerate() {
            if c == ' ' { continue; }
            if row == Point::origin() { row.x = column_index as i32 + 1; }
            row.y = column_index as i32 + 1;
            if c == '#' {
                let wall = Point { x: column_index as i32 + 1, y: r as i32 + 1 };
                walls.insert(wall);
            }
        }
        rows.push(row);
    }
    let path_line = *lines.last().unwrap();
    let mut path: Vec<(u32, char)> = vec![];

    let mut number = 0;
    for char in path_line.chars() {
        if char.is_ascii_digit() { number = number * 10 + char.to_digit(10).unwrap(); }
        else {
            path.push((number, char));
            number = 0;
        }
    }
    path.push((number, 'X'));

    let width = rows.iter().map(|p| p.y).max().unwrap();

    for c in 1..=width {
        let mut column = Point::origin();
        for (r, row) in rows.iter().enumerate() {
            if row.x > c || row.y < c { continue; }
            if column == Point::origin() { column.x = r as i32; }
            column.y = r as i32;
        }
        cols.push(column);
    }

    let mut position = Point::new(rows[1].x, 1);
    let mut direction = Point::new(1, 0);

    for (count, turn) in &path {
        for _ in 0..*count {
            let mut next_pos = position + direction;

            if direction.y == 0 {
                let next_row = rows[next_pos.y as usize];
                if next_pos.x > next_row.y { next_pos.x = next_row.x; }
                if next_pos.x < next_row.x { next_pos.x = next_row.y; }
            } else {
                let next_col = cols[next_pos.x as usize];
                if next_pos.y > next_col.y { next_pos.y = next_col.x; }
                if next_pos.y < next_col.x { next_pos.y = next_col.y; }
            }

            if !walls.contains(&next_pos) {
                 position = next_pos;
            }
        }
        match turn {
            'R' => direction = direction.turn_cw(),
            'L' => direction = direction.turn_ccw(),
            'X' => break,
            _ => panic!()
        }
    }

    println!("Part 1: {}", final_score(&position, &direction));
    println!("Part 2: {}", part2(&rows, &cols, &walls, &path));  // 198093 is too high
}

fn final_score(position: &Point, direction: &Point) -> i32 {
    1000 * position.y + 4 * position.x + face_score( direction)
}

fn face_score(p: &Point) -> i32 {
    match p.x {
        1 => 0,
        -1 => 2,
        0 => match p.y {
            1 => 1,
            -1 => 3,
            _ => panic!()
        }
        _ => panic!()
    }
}

fn part2(rows: &[Point], cols: &[Point], walls: &HashSet<Point>, path: &Vec<(u32, char)>) -> i32{
    let mut position = Point::new(rows[1].x, 1);
    let mut direction = Point::new(1, 0);

    for (count, turn) in path {
        for _ in 0..*count {
            let mut next_pos = position + direction;
            let mut next_dir = direction;

            if direction.y == 0 {
                let next_row = rows[next_pos.y as usize];
                if next_pos.x > next_row.y || next_pos.x < next_row.x {
                    adjust_for_folding(&position, &mut next_pos, &mut next_dir);
                }
            } else {
                let next_col = cols[next_pos.x as usize];
                if next_pos.y > next_col.y || next_pos.y < next_col.x {
                    adjust_for_folding(&position, &mut next_pos, &mut next_dir);
                }
            }

            if !walls.contains(&next_pos) {
                position = next_pos;
                direction = next_dir;
            }
        }
        match turn {
            'R' => direction = direction.turn_cw(),
            'L' => direction = direction.turn_ccw(),
            'X' => break,
            _ => panic!()
        }
    }

    final_score(&position, &direction)
}

/// What a pain...
fn adjust_for_folding(position: &Point, next_pos: &mut Point, next_dir: &mut Point) {
    let quadrant = quadrant_of(position);

    match quadrant {
        2 => {
            match face_score(next_dir) {
                2 => {
                    // Q7 upside down
                    *next_dir = Point { x: 1, y: 0 };
                    *next_pos = Point {x: 1, y: 151 - next_pos.y };
                },
                3 => {
                    // Q10 from the side
                    *next_dir = Point { x: 1, y: 0 };
                    *next_pos = Point {x: 1, y: next_pos.x + 100 };
                },
                _ => panic!()
            }
        },
        3 => {
            match face_score(next_dir) {
                0 => {
                    // Q8 upside down
                    *next_dir = Point { x: -1, y: 0 };
                    *next_pos = Point { x: 100, y: 151 - next_pos.y };
                },
                1 => {
                    // Q5 from right
                    *next_dir = Point { x: -1, y: 0 };
                    *next_pos = Point { x: 100, y: next_pos.x - 50 };
                },
                3 => {
                    // Q10 from the bottom
                    *next_pos = Point {x: next_pos.x - 100, y: 200 };
                },
                _ => panic!()
            }
        },
        5 => {
            match face_score(next_dir) {
                0 => {
                    // Q3 from bottom
                    *next_dir = Point { x: 0, y: -1 };
                    *next_pos = Point { x: next_pos.y + 50, y: 50 };
                },
                2 => {
                    // Q7 from the top
                    *next_dir = Point { x: 0, y: 1 };
                    *next_pos = Point { x: next_pos.y - 50, y: 101 };
                },
                _ => panic!()
            }
        },
        7 => {
            match face_score(next_dir) {
                2 => {
                    // Q2 upside down
                    *next_dir = Point { x: 1, y: 0 };
                    *next_pos = Point {x: 51, y: 151 - next_pos.y };
                },
                3 => {
                    // Q5 from the side
                    *next_dir = Point { x: 1, y: 0 };
                    *next_pos = Point {x: 51, y: next_pos.x + 50 };
                },
                _ => panic!()
            }
        },
        8 => {
            match face_score(next_dir) {
                0 => {
                    // Q3 upside down from the right
                    *next_dir = Point { x: -1, y: 0 };
                    *next_pos = Point { x: 150, y: 151 - next_pos.y };
                },
                1 => {
                    // Q10 from right
                    *next_dir = Point { x: -1, y: 0 };
                    *next_pos = Point { x: 50, y: next_pos.x + 100 };
                },
                _ => panic!()
            }
        },
        10 => {
            match face_score(next_dir) {
                0 => {
                    // Q8 upwards
                    *next_dir = Point { x: 0, y: -1 };
                    *next_pos = Point {x: next_pos.y - 100, y: 150 };
                },
                1 => {
                    // Q3 from the top
                    *next_pos = Point {x:next_pos.x + 100, y: 1 };
                },
                2 => {
                    // Q2 downwards *
                    *next_dir = Point { x: 0, y: 1 };
                    *next_pos = Point {x:next_pos.y - 100, y: 1 };
                },
                _ => panic!()
            }
        },
        _ => panic!(),
    }
}

/*
Quadrants:

      1  2  3
      4  5  6
      7  8  9
     10 11 12
 */
fn quadrant_of(p: &Point) -> i32 {
    ((p.y - 1) / 50) * 3 + ((p.x - 1) / 50) + 1
}

