use std::collections::HashMap;
use std::fs;

use crate::utilities::Point;

pub fn run() {
    println!("Day 17");
    let contents = fs::read_to_string("input/day_17.txt").expect("File missing");
    let jets: Vec<char> = contents.chars().filter(|c| *c == '<' || *c == '>').collect();
    let shapes: Vec<Vec<Point>> = vec![
        vec![Point::new(0,0), Point::new(1, 0), Point::new(2, 0), Point::new(3, 0)],
        vec![Point::new(0,1), Point::new(1, 0), Point::new(1, 1), Point::new(1, 2), Point::new(2, 1)],
        vec![Point::new(0,0), Point::new(1, 0), Point::new(2, 0), Point::new(2, 1), Point::new(2, 2)],
        vec![Point::new(0,0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)],
        vec![Point::new(0,0), Point::new(1, 0), Point::new(0, 1), Point::new(1, 1)],
    ];
    let mut chamber: Vec<Vec<char>> = vec![vec!['.'; 7]; 100000];
    let mut highest_point: i64 = 0;
    let (mut next_jet, mut next_shape) = (0, 0);

    let mut visited: HashMap<usize, i64> = HashMap::new();
    let mut heights: Vec<i64> = vec![0];
    let mut height_per_cycle: i64 = 0;
    let mut cycle_start: i64 = 0;
    let mut cycle_length: i64 = 0;

    let mut turn = 1;
    let mut stop_turn: i64 = 1000000000000000;
    while turn < stop_turn {
        drop_shape(&jets, &shapes, &mut chamber, &mut highest_point, &mut next_jet, &mut next_shape);

       // if turn < 10 || stop_turn < 1000000 { println!("{:?}", turn); draw_chamber(&chamber, highest_point)}
        heights.push(highest_point);
        let key = next_jet * 100 + next_shape;
        // println!("{}: jet {} shape {}, key {}", turn, next_jet, next_shape, key);
        if visited.contains_key(&key) {
            cycle_start = visited[&key] as i64;
            height_per_cycle = highest_point - heights[cycle_start as usize];
            cycle_length = turn - cycle_start;
            stop_turn = turn + 5;
        }
        visited.insert(key, turn);
        turn += 1;
    }
/*
    println!("cycle_start {:?}", cycle_start);
    println!("height_per_cycle {:?}", height_per_cycle);
    println!("cycle_length {:?}", cycle_length);
    for i in (cycle_start + cycle_length - 5)..turn {
        println!("{}: {:?}", i, heights[i as usize]);
    }
 */
    let mut cycles: i64 = (2022 - cycle_start) / cycle_length;
    let mut rest: i64 = (2022 - cycle_start) % cycle_length;
    println!("Part 1: {:?}", heights[(cycle_start + rest) as usize] + cycles * height_per_cycle);

    cycles = (1000000000000 - cycle_start) / cycle_length;
    rest = (1000000000000 - cycle_start) % cycle_length;
    println!("Part 2: {:?}", heights[(cycle_start + rest) as usize] + cycles * height_per_cycle);

}


fn drop_shape(jets: &Vec<char>, shapes: &Vec<Vec<Point>>,
              chamber: &mut [Vec<char>], highest_point: &mut i64,
              next_jet: &mut usize, next_shape: &mut usize) {

    let mut shape_origin = Point::new(2, (*highest_point + 4) as i32);
    let shape = &shapes[*next_shape];
    *next_shape = (*next_shape + 1) % shapes.len();
    let shape_width = 1 + shape.iter().map(|p| p.x).max().unwrap();
    let mut shape_is_falling = true;

    // println!("{:?} for {:?}", shape_origin, shape);
    while shape_is_falling {
        let jet = jets[*next_jet];
        *next_jet = (*next_jet + 1) % jets.len();

        if jet == '<' && shape_origin.x > 0 && shape.iter().all(|p|
                chamber[(p.y + shape_origin.y) as usize][(p.x + shape_origin.x - 1) as usize] == '.') {
            shape_origin.x -= 1;
        }
        if jet == '>' && shape_origin.x + shape_width < 7 && shape.iter().all(|p|
                chamber[(p.y + shape_origin.y) as usize][(p.x + shape_origin.x + 1) as usize] == '.') {
            shape_origin.x += 1;
        }
        // println!("{} => {:?}", jet, shape_origin);

        if shape.iter().any(|p| p.y + shape_origin.y == 1 ||
                chamber[(p.y + shape_origin.y - 1) as usize][(p.x + shape_origin.x) as usize] == '#') {
            shape_is_falling = false;
            // println!("Stop. origin == {:?}", shape_origin);
            for p in shape {
                let chamber_point = *p + &shape_origin;
                chamber[chamber_point.y as usize][chamber_point.x as usize] = '#';
                *highest_point = (*highest_point).max(chamber_point.y as i64);
            }
        } else {
            shape_origin.y -= 1;
            // println!("v => {:?}", shape_origin);
        }
    }
}

#[allow(dead_code)]
fn draw_chamber(chamber: &[Vec<char>], highest_point: i64) {
    for i in (1.max(highest_point - 10)..highest_point + 4).rev() {
        println!("|{}|", String::from_iter(chamber[i as usize].iter()));
    }
    if highest_point < 12 { println!("+-------+"); } else { println!(); }
}
