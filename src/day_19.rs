use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

use crate::traits::StringExtensions;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Recipe {
    id: i32,
    stop_time: i32,
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
    max_ore_cost: i32,
}

impl Recipe {
    pub fn from_string(line: &str) -> Recipe {
        let ints =line.to_int_vector();
        let ore_costs = vec![ints[1], ints[2], ints[3], ints[5]];
        Recipe {
            id: ints[0],
            stop_time: 24,
            ore_robot_cost: ints[1],
            clay_robot_cost: ints[2],
            obsidian_robot_ore_cost: ints[3],
            obsidian_robot_clay_cost: ints[4],
            geode_robot_ore_cost: ints[5],
            geode_robot_obsidian_cost: ints[6],
            max_ore_cost: *ore_costs.iter().max().unwrap(),
        }
    }
}

// ------------------------
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    time: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
    ore_bots: i32,
    clay_bots: i32,
    obsidian_bots: i32,
    geode_bots: i32,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{},{},{},{}-{},{},{},{}]",
               self.time, self.ore, self.clay, self.obsidian, self.geodes,
               self.ore_bots, self.clay_bots, self.obsidian_bots, self.geodes)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ordering = self.geodes.cmp(&other.geodes);
        if ordering == Ordering::Equal { ordering = self.geode_bots.cmp(&other.geode_bots)}
        if ordering == Ordering::Equal { ordering = self.obsidian.cmp(&other.obsidian)}
        if ordering == Ordering::Equal { ordering = self.obsidian_bots.cmp(&other.obsidian_bots)}
        if ordering == Ordering::Equal { ordering = self.clay.cmp(&other.clay)}
        if ordering == Ordering::Equal { ordering = self.clay_bots.cmp(&other.clay_bots)}
        if ordering == Ordering::Equal { ordering = self.ore.cmp(&other.ore)}
        if ordering == Ordering::Equal { ordering = self.ore_bots.cmp(&other.ore_bots)}
        if ordering == Ordering::Equal { ordering = other.time.cmp(&self.time)}
        Some(ordering)
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering { self.partial_cmp(other).unwrap() }
}

// ------------------------
pub fn run() {
    println!("Day 19");
    let contents = fs::read_to_string("input/day_19.txt").expect("File missing");

    let mut recipes: Vec<Recipe> = vec![];
    for line in contents.lines() {
        recipes.push(Recipe::from_string(line));
    }

    let mut part1 = 0;
    for recipe in recipes.iter() {
        part1 += recipe.id * max_geode_production(recipe)
    }
    println!("Part 1: {}", part1);
/*
    let mut part2 = 1;
    for i in 0..2 {
        let mut new_recipe = recipes[i];
        new_recipe.stop_time = 32;
        part2 *= max_geode_production(new_recipe);
    }
    println!("Part 2: {}", part2);
*/
}

fn max_geode_production(recipe: &Recipe) -> i32 {
    let start = State {
        time: 0, ore: 0, clay: 0, obsidian: 0, geodes: 0,
        ore_bots: 1, clay_bots: 0, obsidian_bots: 0, geode_bots: 0, };

    let mut memos: HashMap<State, State> = HashMap::new();
    let mut best_results_so_far = vec![0; (recipe.stop_time + 1) as usize];
    let best_end_result: State = dfs(recipe, &start, &mut memos, &mut best_results_so_far);

    println!("Best end result for recipe {:?}", recipe.id);
    println!("{:?}", best_end_result);
    best_end_result.geodes
}

fn dfs(recipe: &Recipe, state: &State, memos: &mut HashMap<State, State>, best_so_far: &mut Vec<i32>) -> State {
    if memos.contains_key(state) { return memos[state]; }
    if state.time == recipe.stop_time { memos.insert(*state, *state); return *state; }

    // Give up this branch if the best solution seen is better than we can possibly achieve
    let time_left = recipe.stop_time - state.time;
    let theoretical_max = time_left * (state.geode_bots + time_left - 1) + state.geodes;
    if theoretical_max < best_so_far[state.time as usize] { return *state; }

    let children: Vec<State> = children(recipe, state);
    let mut results: Vec<State> = vec![];
    for next in children.iter() {
        results.push(dfs(recipe, next, memos, best_so_far));
    }
    let best = *results.iter().max().unwrap();
    if best_so_far[state.time as usize] < best.geodes {
        best_so_far[state.time as usize] = best.geodes;
    }
    memos.insert(*state, best);
    best
}

fn children(recipe: &Recipe, state: &State) -> Vec<State> {
    let no_action = State {
        time: state.time + 1,
        ore: state.ore + state.ore_bots,
        clay: state.clay + state.clay_bots,
        obsidian: state.obsidian + state.obsidian_bots,
        geodes: state.geodes + state.geode_bots,
        ore_bots: state.ore_bots,
        clay_bots: state.clay_bots,
        obsidian_bots: state.obsidian_bots,
        geode_bots: state.geode_bots,
    };
    let mut result: Vec<State> = vec![];

    // Always build a geode cracking robot if possible
    if recipe.geode_robot_obsidian_cost <= state.obsidian && recipe.geode_robot_ore_cost <= state.ore {
        return vec![State {
            time: no_action.time,
            ore: no_action.ore - recipe.geode_robot_ore_cost,
            clay: no_action.clay,
            obsidian: no_action.obsidian - recipe.geode_robot_obsidian_cost,
            geodes: no_action.geodes,
            ore_bots: no_action.ore_bots,
            clay_bots: no_action.clay_bots,
            obsidian_bots: no_action.obsidian_bots,
            geode_bots: no_action.geode_bots + 1,
        }];
    }

    if recipe.obsidian_robot_clay_cost <= state.clay &&
        recipe.obsidian_robot_ore_cost <= state.ore &&
        state.obsidian_bots < recipe.geode_robot_obsidian_cost
    {
        result.push(State {
            time: no_action.time,
            ore: no_action.ore - recipe.obsidian_robot_ore_cost,
            clay: no_action.clay - recipe.obsidian_robot_clay_cost,
            obsidian: no_action.obsidian,
            geodes: no_action.geodes,
            ore_bots: no_action.ore_bots,
            clay_bots: no_action.clay_bots,
            obsidian_bots: no_action.obsidian_bots + 1,
            geode_bots: no_action.geode_bots,
        })
    }

    if recipe.clay_robot_cost <= state.ore  &&
        state.clay_bots < recipe.obsidian_robot_clay_cost
    {
        result.push(State {
            time: no_action.time,
            ore: no_action.ore - recipe.clay_robot_cost,
            clay: no_action.clay,
            obsidian: no_action.obsidian,
            geodes: no_action.geodes,
            ore_bots: no_action.ore_bots,
            clay_bots: no_action.clay_bots + 1,
            obsidian_bots: no_action.obsidian_bots,
            geode_bots: no_action.geode_bots,
        })
    }

    if recipe.ore_robot_cost <= state.ore &&
        state.ore_bots < recipe.max_ore_cost {
        result.push(State {
            time: no_action.time,
            ore: no_action.ore - recipe.ore_robot_cost,
            clay: no_action.clay,
            obsidian: no_action.obsidian,
            geodes: no_action.geodes,
            ore_bots: no_action.ore_bots + 1,
            clay_bots: no_action.clay_bots,
            obsidian_bots: no_action.obsidian_bots,
            geode_bots: no_action.geode_bots,
        })
    }

    result.push(no_action);

    result
}
