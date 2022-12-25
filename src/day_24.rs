use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

use crate::points::Point;
use crate::utilities::Rect;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    pos: Point,
    cost_so_far: usize,
    est_total_cost: usize,
}

impl State {
    fn new(pos: Point, cost_so_far: usize, goal: &Point) -> State {
        State {
            pos,
            cost_so_far,
            est_total_cost: cost_so_far + pos.manhattan_distance(goal) as usize,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.est_total_cost.cmp(&self.est_total_cost)
            .then_with(|| other.pos.cmp(&self.pos))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



pub fn run() {
    println!("Day 24");
    let contents = fs::read_to_string("input/day_24.txt").expect("Couldn't read the file");
    let lines:Vec<&str> = contents.lines().collect();

    let bounds = Rect {
        left: 1,
        top: lines.len() as i32 - 2,
        bottom: 1,
        right: lines[0].len() as i32 - 2,
    };

    let mut blizzards: HashSet<(Point, char)> = HashSet::new();
    let mut y = bounds.top;
    for line in &lines {
        if line.contains("##") { continue; }
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' | '.' => continue,
                '<' | '>' | '^' | 'v' => {blizzards.insert((Point { x: x as i32, y }, char));},
                _ => panic!()
            }
        }
        y -= 1;
    }
    let mut blizzards_at_time: Vec<_> = vec![blizzards];
    let goal =  Point { x: bounds.right, y: 0 };
    let start = State::new(start_of(&bounds), 0, &goal);

    let part1 = a_star(start, goal, &bounds, &mut blizzards_at_time);
    println!("Part 1: {}", part1.cost_so_far);
    let part1_point_5 =  a_star(part1, start.pos, &bounds, &mut blizzards_at_time);
    let part2 =  a_star(part1_point_5, goal, &bounds, &mut blizzards_at_time);
    println!("Part 2: {:?}", part2.cost_so_far);
}

fn a_star(start: State, goal: Point, bounds: &Rect, blizzards_at_time: &mut Vec<HashSet<(Point, char)>>) -> State {
    let mut visited: HashSet<State> = HashSet::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(start);

    let mut part1: State = start;

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if visited.contains(&current) { continue; }

        if current.pos == goal {
            part1 = current;
            break;
        }
        for neighbor in neighbours(&current, blizzards_at_time, bounds, goal) {
            if visited.contains(&neighbor) { continue; }
            frontier.push(neighbor);
        }
        visited.insert(current);
    }
    part1
}

fn start_of(bounds: &Rect) -> Point {
    Point { x: 1, y: bounds.top + 1 }
}

fn neighbours(state: &State, blizzards_at_time: &mut Vec<HashSet<(Point, char)>>, bounds: &Rect, goal: Point) -> Vec<State> {
    let mut neighbours: Vec<State> = vec![];
    let start = start_of(bounds);
    let minute = state.cost_so_far + 1;
    while blizzards_at_time.len() <= minute {
        let old = blizzards_at_time.last().unwrap();
        let new_blizzards = move_blizzards(bounds, old);
        blizzards_at_time.push(new_blizzards);
    }
    let blizzard_positions =
        &blizzards_at_time[minute].iter().map(|(p, _)|*p).collect::<Vec<Point>>();
    let blizzard_set: HashSet<Point> = HashSet::from_iter(blizzard_positions.iter().cloned());

    for neighbour in &[state.pos.s(), state.pos.e(), state.pos.n(), state.pos.w(), state.pos] {
        if blizzard_set.contains(neighbour) { continue; }
        if bounds.contains_inclusive(*neighbour) || *neighbour == start || neighbour == &goal {
            neighbours.push(State::new(*neighbour, minute, &goal));
        }
    }
    neighbours
}

fn move_blizzards(bounds: &Rect, blizzards: &HashSet<(Point, char)>) -> HashSet<(Point, char)> {
    let mut new_blizzards: HashSet<(Point, char)> = HashSet::new();
    for (pos, dir) in blizzards {
        let new_pos = match *dir {
            '^' => { if pos.y < bounds.top { pos.n() } else { Point { x: pos.x, y: bounds.bottom } } },
            'v' => { if pos.y > bounds.bottom { pos.s() } else { Point { x: pos.x, y: bounds.top } } },
            '<' => { if pos.x > bounds.left { pos.w() } else { Point { x: bounds.right, y: pos.y } } },
            '>' => { if pos.x < bounds.right { pos.e() } else { Point { x: bounds.left, y: pos.y } } },
            _ => { panic!() }
        };
        new_blizzards.insert((new_pos, *dir));
    }
    new_blizzards
}

#[allow(dead_code)]
fn print(state: &State, blizzards_at_time: &[HashSet<(Point, char)>], bounds: &Rect) {

    let blizzards: &HashSet<(Point, char)> = &blizzards_at_time[state.cost_so_far];
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; bounds.right as usize + 2]; bounds.top as usize + 2];

    for x in 0..=bounds.right as usize + 1 {
        grid[0][x] = '#';
        grid[bounds.top as usize + 1][x] = '#';
    }
    for y in 0..(bounds.top as usize + 2) {
        grid[y][0] = '#';
        grid[y][bounds.right as usize + 1] = '#';
    }
    grid[bounds.top as usize + 1][1] = '.';
    grid[0][bounds.right as usize] = '.';

    for (p, c) in blizzards {
        grid[p.y as usize][p.x as usize] =match grid[p.y as usize][p.x as usize] {
            '.' => *c,
            '^' | 'v' | '>' | '<' => '2',
            '2' => '3',
            '3' => '4',
            'E' => '🤯',
            '#' => '💥',
            _ => panic!()
        };
       // grid[p.y as usize][p.x as usize] = *c;
    }
    grid[state.pos.y as usize][state.pos.x as usize] = 'E';

    for line in grid.iter().rev() {
        println!("{}", line.iter().collect::<String>());
    }
}

