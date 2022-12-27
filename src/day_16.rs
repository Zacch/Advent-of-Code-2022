use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 16");
    let contents = fs::read_to_string("input/day_16.txt").expect("Couldn't read the file");

    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in contents.lines() {
        let tokens = line.tokens();
        let rate = line.to_int_vector()[0] as usize;
        let neighbours: Vec<String> = tokens[9..].iter().map(|t|t.replace(',', "")).collect();
        let valve = Valve { name: tokens[1].to_string(), rate, neighbours };
        valves.insert(tokens[1].to_string(), valve);
    }
    println!("Part 1: {}", a_star(&valves, 1, 30));
    println!("Part 2: {}", a_star(&valves, 2, 26));
}

fn a_star(valves: &HashMap<String, Valve>, number_of_players: usize, end_time: usize) -> usize {

    let valves_with_flow: Vec<&Valve> =
        valves.iter().map(|t| t.1).filter(|v|v.rate > 0 ).collect();
    let valve_with_flow_count = valves_with_flow.len();

    let elephant_pos = if number_of_players == 2 { "AA".to_string() } else { "".to_string() };
    let start = State {
        player: "AA".to_string(),
        elephant: elephant_pos,
        time_left: end_time,
        opened_valves: vec![],
        pressure_so_far: 0,
    };

    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    frontier.push(start);

    let mut visited: HashSet<State> = HashSet::new();

    let mut best_so_far = 0;
    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if current.opened_valves.len() == valve_with_flow_count ||
            current.time_left == 0 {
            best_so_far = best_so_far.max(current.pressure_so_far);
            continue;
        }
        if visited.contains(&current) { continue; }

        for neighbour in neighbours(&current, valves) {
            if visited.contains(&neighbour) { continue; }
            if max_flow(&neighbour, &valves_with_flow) >= best_so_far {
                frontier.push(neighbour);
            }
        }
        visited.insert(current);
    }
    best_so_far
}


fn neighbours(state: &State, valves: &HashMap<String, Valve>) -> Vec<State> {
    let player_neighbours = player_neighbours(state, &valves);
    if state.elephant.is_empty() { return player_neighbours; }

    let mut neighbours: Vec<State> = vec![];
    let valve: &Valve = &valves[&state.elephant];
    if valve.rate > 0 && !state.opened_valves.contains(&valve.name)  && state.player != state.elephant {
        for player_neighbour in &player_neighbours {
            let mut new_valves = player_neighbour.opened_valves.clone();
            new_valves.push(valve.name.clone());
            let new_pressure = player_neighbour.pressure_so_far + valve.rate * (player_neighbour.time_left);
            neighbours.push(State {
                player: player_neighbour.player.clone(),
                elephant: player_neighbour.elephant.clone(),
                opened_valves: new_valves,
                pressure_so_far: new_pressure,
                ..*player_neighbour
            })
        }
    }
    for next_valve_name in valve.neighbours.iter() {
        for player_neighbour in &player_neighbours {
            neighbours.push(State {
                player: player_neighbour.player.clone(),
                elephant: next_valve_name.clone(),
                opened_valves: player_neighbour.opened_valves.clone(),
                ..*player_neighbour
            })
        }
    }

    neighbours
}

fn player_neighbours(state: &State, valves: &&HashMap<String, Valve>) -> Vec<State> {
    let mut neighbours: Vec<State> = vec![];
    let valve: &Valve = &valves[&state.player];
    if valve.rate > 0 && !state.opened_valves.contains(&valve.name) {
        let mut new_valves = state.opened_valves.clone();
        new_valves.push(valve.name.clone());
        let new_pressure = state.pressure_so_far + valve.rate * (state.time_left - 1);
        neighbours.push(State {
            player: state.player.clone(),
            elephant: state.elephant.clone(),
            time_left: state.time_left - 1,
            opened_valves: new_valves,
            pressure_so_far: new_pressure,
        })
    }
    for next_valve_name in valve.neighbours.iter() {
        neighbours.push(State {
            player: next_valve_name.to_string(),
            elephant: state.elephant.clone(),
            time_left: state.time_left - 1,
            opened_valves: state.opened_valves.clone(),
            ..*state
        })
    }
    neighbours
}

fn max_flow(state: &State, valves_with_flow: &[&Valve]) -> usize {
    state.pressure_so_far +
    valves_with_flow.iter()
        .filter(|v| !state.opened_valves.contains(&v.name))
        .map(|v| v.rate * (1.max(state.time_left) - 1))
        .sum::<usize>()
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Valve {
    name: String,
    rate: usize,
    neighbours: Vec<String>
}

#[derive(Clone, Eq, Debug)]
struct State {
    player: String,
    elephant: String,
    time_left: usize,
    opened_valves: Vec<String>,
    pressure_so_far: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player.hash(state);
        self.elephant.hash(state);
        self.time_left.hash(state);
        self.pressure_so_far.hash(state);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pressure_so_far.cmp(&other.pressure_so_far)
            .then_with(|| self.time_left.cmp(&other.time_left))
            .then_with(|| self.player.cmp(&other.player))
            .then_with(|| self.elephant.cmp(&other.elephant))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

