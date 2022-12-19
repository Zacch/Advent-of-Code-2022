use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

use crate::traits::StringExtensions;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Recipe {
    id: i32,
    stop_time: i32,
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
}

impl Recipe {

    pub fn from_string(line: &str) -> Recipe {
        let ints =line.to_int_vector();
        Recipe {
            id: ints[0],
            stop_time: 24,
            ore_robot_cost: ints[1],
            clay_robot_cost: ints[2],
            obsidian_robot_ore_cost: ints[3],
            obsidian_robot_clay_cost: ints[4],
            geode_robot_ore_cost: ints[5],
            geode_robot_obsidian_cost: ints[6],
        }
    }
}

pub fn run() {
    println!("Day 19");
    let contents = fs::read_to_string("input/day_19.txt").expect("File missing");

    let mut recipes: Vec<Recipe> = vec![];
    for line in contents.lines() {
        recipes.push(Recipe::from_string(line));
    }

    let mut part1 = 0;
    for recipe in recipes.iter() {
        part1 += recipe.id * max_geode_production(*recipe)
    }
    println!("Part 1: {}", part1);

    let mut part2 = 1;
    for i in 0..2 {
        let mut new_recipe = recipes[i];
        new_recipe.stop_time = 32;
        part2 *= max_geode_production(new_recipe);
    }
    println!("Part 2: {}", part2);

}

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

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{},{},{},{}-{},{},{},{}]",
               self.time, self.ore, self.clay, self.obsidian, self.geodes,
                self.ore_bots, self.clay_bots, self.obsidian_bots, self.geodes)
    }
}

fn max_geode_production(recipe: Recipe) -> i32 {
    let start = State {
        time: 0, ore: 0, clay: 0, obsidian: 0, geodes: 0,
        ore_bots: 1, clay_bots: 0, obsidian_bots: 0, geode_bots: 0, };

    let mut memos: HashMap<String, State> = HashMap::new();
    let best_end_result: State = dfs(recipe, start, &mut memos);

    println!("Best end result for recipe {:?}", recipe.id);
    println!("{:?}", best_end_result);
    best_end_result.geodes
}

fn dfs(recipe: Recipe, state: State, memos: &mut HashMap<String, State>) -> State {
    let state_key = state.to_string();
    if memos.contains_key(&state_key) { return memos[&state_key]; }
    if state.time == recipe.stop_time { memos.insert(state_key, state); return state; }

    let descendants: Vec<State> = descendants(recipe, state);
    let mut results: Vec<State> = vec![];
    for next in descendants {
        results.push(dfs(recipe, next, memos));
    }
    let best = *results.iter().max().unwrap();
    memos.insert(state_key, best);
    best
}

fn descendants(recipe: Recipe, state: State) -> Vec<State> {
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

    if recipe.geode_robot_obsidian_cost <= state.obsidian && recipe.geode_robot_ore_cost <= state.ore {
        result.push(State {
            time: no_action.time,
            ore: no_action.ore - recipe.geode_robot_ore_cost,
            clay: no_action.clay,
            obsidian: no_action.obsidian - recipe.geode_robot_obsidian_cost,
            geodes: no_action.geodes,
            ore_bots: no_action.ore_bots,
            clay_bots: no_action.clay_bots,
            obsidian_bots: no_action.obsidian_bots,
            geode_bots: no_action.geode_bots + 1,
        })
    }

    if recipe.obsidian_robot_clay_cost <= state.clay && recipe.obsidian_robot_ore_cost <= state.ore {
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

    if recipe.clay_robot_cost <= state.ore {
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

    if recipe.ore_robot_cost <= state.ore {
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
