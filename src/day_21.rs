use std::collections::HashMap;
use std::fs;

use crate::traits::StringExtensions;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Monkey {
    name: String,
    operator: char,
    op1: String,
    op2: String
}

pub fn run() {
    println!("Day 21");
    let contents = fs::read_to_string("input/day_21.txt").expect("Couldn't read the file");

    let mut monkey_number: HashMap<String, i64> = HashMap::new();
    let mut undecided_monkeys: Vec<Monkey> = vec![];
    parse_contents(&contents, &mut monkey_number, &mut undecided_monkeys);
    process_monkeys(&mut monkey_number, &mut undecided_monkeys);
    println!("Part 1: {}", monkey_number["root"]);
    println!("Part 2: {}", part2(contents));
}

fn part2(contents: String) -> i64 {
    let mut monkey_number: HashMap<String, i64> = HashMap::new();
    let mut undecided_monkeys: Vec<Monkey> = vec![];
    parse_contents(&contents, &mut monkey_number, &mut undecided_monkeys);

    monkey_number.retain(|s, _| s != "humn");
    process_monkeys(&mut monkey_number, &mut undecided_monkeys);

    let root = undecided_monkeys.iter().find(|m| m.name == "root").unwrap();

    let mut current_number: i64;
    let mut current_monkey: &Monkey;
    if monkey_number.contains_key(&root.op1) {
        current_number = monkey_number[&root.op1];
        current_monkey = undecided_monkeys.iter().find(|m| m.name == root.op2).unwrap();
    } else {
        current_monkey = undecided_monkeys.iter().find(|m| m.name == root.op1).unwrap();
        current_number = monkey_number[&root.op2];
    }

    loop {
        if monkey_number.contains_key(&current_monkey.op1) {
            let other_number = monkey_number[&current_monkey.op1];
            current_number = match current_monkey.operator {
                '+' => current_number - other_number,
                '-' => other_number - current_number,
                '*' => current_number / other_number,
                '/' => other_number / current_number,
                _ => panic!()
            };
            if current_monkey.op2 == "humn" { break; }
            current_monkey = undecided_monkeys.iter().find(|m| m.name == current_monkey.op2).unwrap();
        } else {
            let other_number = monkey_number[&current_monkey.op2];
            current_number = match current_monkey.operator {
                '+' => current_number - other_number,
                '-' => other_number + current_number,
                '*' => current_number / other_number,
                '/' => other_number * current_number,
                _ => panic!()
            };
            if current_monkey.op1 == "humn" { break; }
            current_monkey = undecided_monkeys.iter().find(|m| m.name == current_monkey.op1).unwrap();
        }
    }
    current_number
}

fn process_monkeys(monkey_number: &mut HashMap<String, i64>, undecided_monkeys: &mut Vec<Monkey>) {
    let mut done = false;
    while !done {
        done = true;
        for monkey in undecided_monkeys.iter() {
            if monkey_number.contains_key(&monkey.op1) && monkey_number.contains_key(&monkey.op2) {
                let op1 = monkey_number[&monkey.op1];
                let op2 = monkey_number[&monkey.op2];
                let result = match monkey.operator {
                    '+' => op1 + op2,
                    '-' => op1 - op2,
                    '*' => op1 * op2,
                    '/' => op1 / op2,
                    _ => panic!()
                };
                monkey_number.insert(monkey.name.clone(), result);
                done = false;
            }
        }
        undecided_monkeys.retain(|m| !monkey_number.contains_key(&m.name));
    }
}

fn parse_contents(contents: &str, monkey_number: &mut HashMap<String, i64>, undecided_monkeys: &mut Vec<Monkey>) {
    for line in contents.lines() {
        let name = &line[..(line.find(':').unwrap())];
        let tokens = line.tokens();
        if tokens.len() == 2 {
            monkey_number.insert(name.to_string(), tokens[1].parse().unwrap());
            continue;
        }
        if tokens.len() != 4 { panic!("Bad syntax of {}", line); }
        undecided_monkeys.push(Monkey {
            name: name.to_string(),
            operator: tokens[2].to_string().chars().next().unwrap(),
            op1: tokens[1].to_string(),
            op2: tokens[3].to_string(),
        })
    }
}

#[allow(dead_code)]
fn print_monkeys(monkey_number: &HashMap<String, i64>, undecided_monkeys: &Vec<Monkey>) {
    for monkey in undecided_monkeys {
        print_monkey(monkey_number, monkey);
    }
}

fn print_monkey(monkey_number: &HashMap<String, i64>, monkey: &Monkey) {
    println!("{} == {} {} {}", monkey.name,
         if monkey_number.contains_key(&monkey.op1) { monkey_number[&monkey.op1].to_string() } else { monkey.op1.clone() },
         monkey.operator,
         if monkey_number.contains_key(&monkey.op2) { monkey_number[&monkey.op2].to_string() } else { monkey.op2.clone() });
}
