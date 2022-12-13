use std::cmp::{min, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs;
use std::str::Chars;

#[derive(Debug, Eq, PartialEq)]
enum Item {
    Int(u32),
    Vec(Vec<Item>),
}

pub fn run() {
    println!("Day 13");

    let contents = fs::read_to_string("input/day_13.txt").expect("Couldn't read the file");
    let lines: Vec<&str> = Vec::from_iter(contents.lines());

    let mut packets: Vec<Item> = vec![];
    let mut part1 = 0;

    for i in 0..(lines.len() + 1) / 3 {
        let left = parse(lines[i * 3]);
        let right = parse(lines[i * 3 + 1]);
        if order(&left, &right) == Less { part1 += i + 1; }
        packets.push(left);
        packets.push(right);
    }
    println!("Part 1: {}", part1);

    let divider_1 = parse("[[2]]");
    let divider_2 = parse("[[6]]");
    let (mut part21, mut part22) = (1, 2);

    for packet in packets {
        if order(&packet, &divider_1) == Less { part21 += 1; }
        if order(&packet, &divider_2) == Less { part22 += 1; }
    }
    println!("Part 2: {}", part21 * part22);
}

fn parse(line: &str) -> Item {
    let mut chars = line.chars();
    assert_eq!(chars.next().unwrap(), '[');
    parse_list(&mut chars)
}

// The first '[' is already parsed
fn parse_list(chars: &mut Chars) -> Item {

    let mut list: Vec<Item> = vec![];
    let mut c = chars.next().unwrap();
    while c != ']' {
        if c == '[' {
            list.push(parse_list(chars));
            c = chars.next().unwrap();
        } else {
            let mut number = c.to_digit(10).unwrap();
            c = chars.next().unwrap();
            while c.is_ascii_digit() {
                number = number * 10 + c.to_digit(10).unwrap();
                c = chars.next().unwrap();
            }
            list.push(Item::Int(number));
        }
        if c == ',' {
            c = chars.next().unwrap();
        }
    }
    Item::Vec(list)
}

fn order(left: &Item, right: &Item) -> Ordering {
    match left {
        Item::Int(l) => {
            match right {
                Item::Int(r) => {
                    if *l < *r { return Less }
                    if *l > *r { return Greater }
                    Equal
                }
                Item::Vec(_) => {
                    let wrapped = Item::Vec(vec![Item::Int(*l)]);
                    order(&wrapped, right)
                }
            }
        }
        Item::Vec(left_vector) => {
            match right {
                Item::Int(r) => {
                    let wrapped = Item::Vec(vec![Item::Int(*r)]);
                    order(left, &wrapped)
                }
                Item::Vec(right_vector) => {
                    for i in 0..min(left_vector.len(), right_vector.len()) {
                        let ordering = order(&left_vector[i], &right_vector[i]);
                        // println!("{:?}", ordering);
                        if ordering != Equal { return ordering }
                    }
                    left_vector.len().cmp(&right_vector.len())
                }
            }
        }
    }
}
