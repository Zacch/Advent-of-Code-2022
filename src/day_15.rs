use std::fs;

use crate::traits::StringExtensions;
use crate::utilities::{IntRange, Point};

pub fn run() {
    println!("Day 15");
    let contents = fs::read_to_string("input/day_15.txt").expect("Couldn't read the file");

    let mut beacons: Vec<Point> = vec![];
    let mut sensors: Vec<(Point, i32)> = vec![];

    for line in contents.lines() {
        let numbers = line.to_int_vector();
        let sensor = Point::new(numbers[0], numbers[1]);
        let beacon = Point::new(numbers[2], numbers[3]);
        let distance = sensor.manhattan_distance(&beacon);
        sensors.push((sensor, distance));
        if !beacons.contains(&beacon) { beacons.push(beacon); }
    }

    println!("Part1: {}", part1(2000000, &sensors, beacons));

    let free_point: Point = find_uncovered_point(&sensors, 4000000);
    println!("Part 2: {}", free_point.x as i64 * 4000000 + free_point.y as i64);
}

fn part1(line: i32, sensors: &Vec<(Point, i32)>, beacons: Vec<Point>) -> i32 {
    sensor_coverage(line, &sensors).iter().map(|r| r.end + 1 - r.start).sum::<i32>() -
        beacons.iter().filter(|b| b.y == line).count() as i32
}

fn find_uncovered_point(sensors: &Vec<(Point, i32)>, max_coord: i32) -> Point {
    for y in 0..=max_coord {
        let coverage = sensor_coverage(y, &sensors);
        match coverage.len() {
            1 => {
                if coverage[0].start > 0 { return Point::new(0, y); }
                if coverage[0].end < max_coord { return Point::new(coverage[0].end + 1, y); } },
            2 => return Point::new(i32::max(coverage[0].start, coverage[1].start) - 1, y),
            _ => panic!()
        }
    }
    panic!()
}

fn sensor_coverage(line: i32, sensors: &&Vec<(Point, i32)>) -> Vec<IntRange> {
    let mut covered: Vec<IntRange> = vec![];
    for (sensor, range) in sensors.iter() {
        let overlap = range - i32::abs(sensor.y - line);
        if overlap < 0 { continue; }
        let  new_range = IntRange::new(sensor.x - overlap, sensor.x + overlap);

        let mut ranges_to_merge: Vec<IntRange> = vec![];

        for range in covered.iter() {
            if range.touches(&new_range) { ranges_to_merge.push(*range); }
        }

        if ranges_to_merge.is_empty() {
            covered.push(new_range);
            continue
        }
        covered.retain(|r| !ranges_to_merge.contains(r));

        ranges_to_merge.push(new_range);
        let start = ranges_to_merge.iter().map(|r| r.start).min().unwrap();
        let end = ranges_to_merge.iter().map(|r| r.end).max().unwrap();
        covered.push(IntRange::new(start, end))
    }
    covered
}
