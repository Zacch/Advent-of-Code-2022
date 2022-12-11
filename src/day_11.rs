use std::collections::HashMap;
use std::fs;

use crate::traits::StringExtensions;

pub fn run() {
    println!("Day 11");
    let contents = fs::read_to_string("input/day_11.txt")
        .expect("Couldn't read the file");

    let lines: Vec<&str> = Vec::from_iter(contents.lines());

    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkeys2: Vec<Monkey> = vec![];

    let mut line_no = 0;
    while line_no + 5 < lines.len() {
        let items = lines[line_no + 1].to_int_vector();
        let items2 = lines[line_no + 1].to_int_vector();
        let operation_tokens = lines[line_no + 2].tokens();
        let operation: Op = match operation_tokens[4] {
            "*" => if operation_tokens[5] == "old" { Op::Square } else { Op::Multiply },
            "+" => Op::Add,
            _ => panic!()
        };
        let op2 = operation.clone();
        let argument = if operation_tokens[5] == "old" { 0 } else { operation_tokens[5].parse().unwrap() };
        let test_divisor = lines[line_no + 3].to_int_vector()[0];
        let receiver_if_true = lines[line_no + 4].to_int_vector()[0] as usize;
        let receiver_if_false = lines[line_no + 5].to_int_vector()[0] as usize;
        let monkey = Monkey { items, operation, argument, test_divisor, receiver_if_true, receiver_if_false, items_inspected: 0 };
        monkeys.push(monkey);
        let monkey2 = Monkey { items: items2, operation: op2, argument, test_divisor, receiver_if_true, receiver_if_false, items_inspected: 0 };
        monkeys2.push(monkey2);
        line_no += 7;
    }

    // part 1
    {
        for _round in 1..21 {
            for i in 0..monkeys.len() {
                let mut targets: HashMap<usize, Vec<i32>> = HashMap::new();
                let items_to_inspect: usize;
                {
                    let monkey = &monkeys[i];
                    items_to_inspect = monkey.items.len();
                    for item in monkey.items.iter() {
                        let inspected = match monkey.operation {
                            Op::Multiply => { item * monkey.argument }
                            Op::Add => { item + monkey.argument }
                            Op::Square => { item * item }
                        };
                        let worry_level = inspected / 3;
                        let target =
                            if worry_level % monkey.test_divisor == 0
                            { monkey.receiver_if_true } else { monkey.receiver_if_false };
                        match targets.get_mut(&target) {
                            None => {
                                targets.insert(target, vec![worry_level]);
                            },
                            Some(v) => {
                                v.push(worry_level);
                            }
                        }
                    }
                }
                for (target_index, mut items) in targets {
                    let target = monkeys.iter_mut().nth(target_index).unwrap();
                    target.items.append(&mut items);
                }
                let mutable_monkey = monkeys.iter_mut().nth(i).unwrap();
                mutable_monkey.items.clear();
                mutable_monkey.items_inspected += items_to_inspect;
            }
        }
        let mut inspections: Vec<usize> = monkeys.iter()
            .map(|m| m.items_inspected)
            .collect();
        inspections.sort();
        inspections.reverse();
        println!("Part 1: {}", inspections[0] * inspections[1]);
    }

    // part 2
    {
        let modulo = monkeys2.iter()
            .fold(1, |p, m| p * m.test_divisor) as i64;

        for _round in 0..10000 {
            for i in 0..monkeys2.len() {
                let mut targets: HashMap<usize, Vec<i32>> = HashMap::new();
                let items_to_inspect: usize;
                {
                    let monkey = &monkeys2[i];
                    items_to_inspect = monkey.items.len();
                    for item in monkey.items.iter().map(|i| *i as i64) {
                        let inspected: i64 = match monkey.operation {
                            Op::Multiply => { item * (monkey.argument as i64) }
                            Op::Add => { item + (monkey.argument as i64) }
                            Op::Square => { item * item }
                        };
                        let worry_level = inspected % modulo;
                        let target =
                            if worry_level % (monkey.test_divisor as i64) == 0
                            { monkey.receiver_if_true } else { monkey.receiver_if_false };
                        match targets.get_mut(&target) {
                            None => {
                                targets.insert(target, vec![worry_level as i32]);
                            },
                            Some(v) => {
                                v.push(worry_level as i32);
                            }
                        }
                    }
                }
                for (target_index, mut items) in targets {
                    let target = monkeys2.iter_mut().nth(target_index).unwrap();
                    target.items.append(&mut items);
                }
                let mutable_monkey = monkeys2.iter_mut().nth(i).unwrap();
                mutable_monkey.items.clear();
                mutable_monkey.items_inspected += items_to_inspect;
            }
        }
        let mut inspections: Vec<usize> = monkeys2.iter()
            .map(|m| m.items_inspected)
            .collect();
        inspections.sort();
        inspections.reverse();
        println!("Part 2: {}", inspections[0] * inspections[1]);
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
enum Op { Multiply, Add, Square }

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Op,
    argument: i32,
    test_divisor: i32,
    receiver_if_true: usize,
    receiver_if_false: usize,
    items_inspected: usize,
}
