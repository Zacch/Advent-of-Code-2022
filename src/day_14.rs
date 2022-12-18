use std::fs;

use crate::points::Point;
use crate::traits::StringExtensions;
use crate::utilities::Rect;

pub fn run() {
    println!("Day 14");
    let contents = fs::read_to_string("input/day_14.txt").expect("Couldn't read the file");

    let mut all_points: Vec<Point> = vec![];
    let mut line_points: Vec<Vec<Point>> = vec![];

    for line in contents.lines() {
        let numbers = line.to_int_vector();
        let mut points: Vec<Point> = vec![];
        for i in (0..numbers.len()).step_by(2) {
            points.push(Point::new(numbers[i], numbers[i + 1]));
            all_points.push(Point::new(numbers[i], numbers[i + 1]));
        }
        line_points.push(points);
    }

    let mut bounds = Rect { top: all_points[0].y, left: all_points[0].x, bottom: 0, right: all_points[0].x };

    for point in all_points {
        if point.x < bounds.left { bounds.left = point.x };
        if point.x > bounds.right { bounds.right = point.x };
        if point.y > bounds.top { bounds.top = point.y };
    }

    let mut grid:Vec<Vec<char>> = vec![vec!['.'; (bounds.right + 1 - bounds.left) as usize]; (bounds.top + 1) as usize];
    set_walls(&mut grid, &line_points, bounds);
    println!("Part 1: {}", pour_sand(&mut grid, bounds));
    // show(&grid);

    let bounds2 = Rect { top: bounds.top + 2, left: 500 - (bounds.top + 3), bottom: 0, right: 500 + bounds.top + 3 };
    let mut grid2:Vec<Vec<char>> = vec![vec!['.'; (bounds2.right + 1 - bounds2.left) as usize]; (bounds2.top) as usize];
    grid2.push(vec!['#'; (bounds2.right + 1 - bounds2.left) as usize]);
    set_walls(&mut grid2, &line_points, bounds2);
    set(&mut grid2, Point::new(500, 0), '+', bounds2);
    println!("Part 2: {}", pour_sand(&mut grid2, bounds2));
}

fn set_walls(grid: &mut [Vec<char>], line_points: &Vec<Vec<Point>>, bounds: Rect) {
    for line in line_points {
        let mut current = Point::origin();
        for end_point in line {
            if current == Point::origin() {
                current = *end_point;
                set(grid, current, '#', bounds);
                continue;
            }

            while current != *end_point {
                if current.x < end_point.x { current.x += 1 }
                if current.x > end_point.x { current.x -= 1 }
                if current.y < end_point.y { current.y += 1 }
                if current.y > end_point.y { current.y -= 1 }
                set(grid, current, '#', bounds);
            }
        }
    }
}

fn pour_sand(grid: &mut [Vec<char>], bounds: Rect) -> i32 {
    let mut grains = 0;
    while drop_grain(grid, bounds) { grains += 1; }
    grains
}

/// Returns true if the grain comes to rest
fn drop_grain(grid: &mut [Vec<char>], bounds: Rect) -> bool {

    let mut pos = Point::new(500, 0);
    if get(grid, pos, bounds) == 'o' {
        return false;
    }

    // Yes, the grain falls northwards (there must be a Santa joke here somewhere ;))
    while pos.y < bounds.top {
        if get(grid, pos.n(), bounds) == '.' {
            pos = pos.n();
            continue
        }

        if !bounds.contains(pos.w()) { return false; }
        if get(grid, pos.nw(), bounds) == '.' {
            pos = pos.nw();
            continue
        }

        if !bounds.contains(pos.e()) { return false; }
        if get(grid, pos.ne(), bounds) == '.' {
            pos = pos.ne();
            continue
        }
        set(grid, pos, 'o', bounds);
        return true;
    }
    false
}

fn get(grid: & [Vec<char>], p: Point, bounds: Rect) -> char {
    grid[p.y as usize][(p.x - bounds.left) as usize]
}

fn set(grid: &mut [Vec<char>], p: Point, c: char, bounds: Rect) {
    let col_index = (p.x - bounds.left) as usize;
    let row = &mut grid[p.y as usize];
    row[col_index] = c;
}

#[allow(dead_code)]
fn show(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
