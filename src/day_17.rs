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
    let mut chamber: Vec<Vec<char>> = vec![vec!['.'; 7]; 1000000];
    let mut column_heights: Vec<i64> = vec![0; 7];
    let mut highest_point: i64;
    let (mut next_jet, mut next_shape) = (0, 0);

    let mut highest_column_history: Vec<i64> = vec![0];
    let mut history: HashMap<String, i64> = HashMap::new();

    let height_per_cycle: i64;
    let cycle_start: i64;
    let cycle_length: i64;

    let mut turn = 1;
    loop {
        drop_shape(&jets, &shapes, &mut chamber, &mut column_heights, &mut next_jet, &mut next_shape);
        highest_point = *column_heights.iter().max().unwrap();

        let mut this_key = "".to_string();
        for i in 0..7 {
            this_key += &(50 + column_heights[i] - column_heights[0]).to_string();
        }

        let full_key = next_shape.to_string() + &next_jet.to_string() + &this_key.to_owned();
        if history.contains_key(&full_key) {
            cycle_start = history[&full_key];
            height_per_cycle = highest_point - highest_column_history[cycle_start as usize];
            cycle_length = turn - cycle_start;
            break;
        }
        history.insert(full_key, turn);
        highest_column_history.push(highest_point);

        turn += 1;
    }

    let mut cycles: i64 = (2022 - cycle_start) / cycle_length;
    let mut rest: i64 = (2022 - cycle_start) % cycle_length;
    println!("Part 1: {:?}", highest_column_history[(cycle_start + rest) as usize] + cycles * height_per_cycle);

    cycles = (1000000000000 - cycle_start) / cycle_length;
    rest = (1000000000000 - cycle_start) % cycle_length;
    println!("Part 2: {:?}", highest_column_history[(cycle_start + rest) as usize] + cycles * height_per_cycle);
}


fn drop_shape(jets: &Vec<char>, shapes: &Vec<Vec<Point>>,
              chamber: &mut [Vec<char>], column_heights: &mut [i64],
              next_jet: &mut usize, next_shape: &mut usize) {

    let highest_point = column_heights.iter().max().unwrap();
    let mut shape_origin = Point::new(2, (*highest_point + 4) as i32);
    let shape = &shapes[*next_shape];
    *next_shape = (*next_shape + 1) % shapes.len();
    let shape_width = 1 + shape.iter().map(|p| p.x).max().unwrap();
    let mut shape_is_falling = true;

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

        if shape.iter().any(|p| p.y + shape_origin.y == 1 ||
                chamber[(p.y + shape_origin.y - 1) as usize][(p.x + shape_origin.x) as usize] == '#') {
            shape_is_falling = false;
            for p in shape {
                let chamber_point = *p + &shape_origin;
                chamber[chamber_point.y as usize][chamber_point.x as usize] = '#';
                column_heights[chamber_point.x as usize] =
                    column_heights[chamber_point.x as usize].max(chamber_point.y as i64);
            }
        } else {
            shape_origin.y -= 1;
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
