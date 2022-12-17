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
    let mut columns: Vec<i32> = vec![0; 7];
    let (mut next_jet, mut next_shape) = (0, 0);

    for _turn in 1..=2022 {
        let mut shape_origin = Point::new(2, 4 + *columns.iter().max().unwrap());
        let shape = &shapes[next_shape];
        next_shape = (next_shape + 1) % shapes.len();
        let shape_width = 1 + shape.iter().map(|p| p.x).max().unwrap();
        let mut shape_is_falling = true;

        println!("{:?} for {:?}", shape_origin, shape);
        while shape_is_falling {
            let jet = jets[next_jet];
            next_jet = (next_jet + 1) % jets.len();

            if jet == '<' && shape_origin.x > 0 &&
                shape.iter().all(|p| columns[(p.x + shape_origin.x - 1) as usize] < p.y + shape_origin.y) {
                shape_origin.x -= 1;
            }
            if jet == '>' && shape_origin.x + shape_width < 7 &&
                shape.iter().all(|p| columns[(p.x + shape_origin.x + 1) as usize] < p.y + shape_origin.y) {
                shape_origin.x += 1;
            }
            println!("{} => {:?}", jet, shape_origin);

            if shape.iter().any(|p| columns[(p.x + shape_origin.x) as usize] >= p.y + shape_origin.y - 1) {
                shape_is_falling = false;
                println!("{:?}, origin == {:?}", columns, shape_origin);
                for p in shape {
                    let column = (p.x + shape_origin.x) as usize;
                    columns[column] = columns[column].max(p.y + shape_origin.y);
                }
                println!("{:?} afterr turn {}", columns, _turn);
                println!("{:?}", "");
            } else {
                shape_origin.y -= 1;
                println!("v => {:?}", shape_origin);

            }
        }
    }
    println!("Part 1: {:?}", columns.iter().max().unwrap());
}
