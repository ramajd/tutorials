use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum Target {
    Output(usize),
    Bot(usize),
}

enum Instruction {
    Input(usize, usize),
    Logic(usize, Target, Target),
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buffer = BufReader::new(file);

    let mut instructions = HashMap::new();
    let mut inputs = HashMap::new();

    for line in buffer.lines() {
        let line = line.unwrap();

        if let Some(instruction) = parse_line(line.trim()) {
            match instruction {
                Instruction::Input(id, value) => {
                    // inputs.insert(id, value);
                    inputs.entry(id).or_insert(vec![]).push(value);
                }
                Instruction::Logic(id, lower, higher) => {
                    instructions.insert(id, (lower, higher));
                }
            };
        }
    }

    let mut bots: HashMap<usize, RefCell<Vec<usize>>> = HashMap::new();
    let mut outputs: HashMap<usize, RefCell<Vec<usize>>> = HashMap::new();

    for (id, _) in &instructions {
        bots.insert(*id, RefCell::new(vec![]));
    }

    for (id, value) in inputs {
        for v in value {
            bots.get(&id).unwrap().borrow_mut().push(v);
        }
    }

    while bots.iter().any(|(_, values)| values.borrow().len() == 2) {
        for (id, values) in bots.iter().filter(|&(_, v)| v.borrow().len() == 2) {
            let mut values = values.borrow_mut();
            values.sort();

            if *values == [17, 61] {
                println!("Part 1 = {}", id);
            }

            let (lower, higher) = instructions.get(id).unwrap();

            let lower_target = match lower {
                Target::Output(target_id) => {
                    outputs.entry(*target_id).or_insert(RefCell::new(vec![]))
                }
                Target::Bot(target_id) => bots.get(target_id).unwrap(),
            };
            lower_target.borrow_mut().push(values.remove(0));

            let higher_target = match higher {
                Target::Output(target_id) => {
                    outputs.entry(*target_id).or_insert(RefCell::new(vec![]))
                }
                Target::Bot(target_id) => bots.get(target_id).unwrap(),
            };
            higher_target.borrow_mut().push(values.remove(0));
        }
    }

    let o0 = outputs.get(&0).unwrap().borrow()[0];
    let o1 = outputs.get(&1).unwrap().borrow()[0];
    let o2 = outputs.get(&2).unwrap().borrow()[0];
    println!("Part 2 = {}", o0 * o1 * o2);
}

fn parse_line(line: &str) -> Option<Instruction> {
    let mut parts = line.trim().split(' ');
    match parts.next().unwrap() {
        "bot" => {
            // bot 188 gives low to output 0 and high to bot 72
            // bot 79 gives low to bot 204 and high to bot 37
            let id = parts.next().unwrap().parse().unwrap();
            let lower = parts
                .nth(3)
                .and_then(|t| match t {
                    "output" => {
                        let tid = parts.next().unwrap().parse().unwrap();
                        Some(Target::Output(tid))
                    }
                    "bot" => {
                        let tid = parts.next().unwrap().parse().unwrap();
                        Some(Target::Bot(tid))
                    }
                    _ => None,
                })
                .unwrap();
            let higher = parts
                .nth(3)
                .and_then(|t| match t {
                    "output" => {
                        let tid = parts.next().unwrap().parse().unwrap();
                        Some(Target::Output(tid))
                    }
                    "bot" => {
                        let tid = parts.next().unwrap().parse().unwrap();
                        Some(Target::Bot(tid))
                    }
                    _ => None,
                })
                .unwrap();
            Some(Instruction::Logic(id, lower, higher))
        }
        "value" => {
            // value 47 goes to bot 142
            let value = parts.next().unwrap().parse().unwrap();
            let id = parts.nth(3).unwrap().parse().unwrap();
            Some(Instruction::Input(id, value))
        }
        _ => panic!("invalid line: {}", line),
    }
}
