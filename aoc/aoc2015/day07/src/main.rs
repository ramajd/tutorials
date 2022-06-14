use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Wire {
    gate: String,
    inputs: Vec<String>,
    output: String,
}

fn parse_wire(instruction: &str) -> Wire {
    let mut split = instruction.split("->");
    let inputs = split.next().unwrap().trim();
    let output = split.next().unwrap().trim().to_owned();
    let input_parts: Vec<&str> = inputs.split(" ").map(|p| p.trim()).collect();

    let gate = match input_parts.len() {
        1 => "_".to_string(),
        2 => input_parts[0].to_string(),
        3 => input_parts[1].to_string(),
        _ => panic!("Invalid instruction: {}", instruction),
    };
    let inputs = match input_parts.len() {
        1 => vec![input_parts[0].to_string()],
        2 => vec![input_parts[1].to_string()],
        3 => vec![input_parts[0].to_string(), input_parts[2].to_string()],
        _ => panic!("Invalid instruction: {}", instruction),
    };
    Wire {
        gate,
        inputs,
        output,
    }
}

fn calculate_wire_value(wire: &Wire, results: &HashMap<String, u16>) -> Option<u16> {
    let key = wire.output.clone();
    if let Some(value) = results.get(&key) {
        return Some(*value);
    }

    let get_value = |input: &str| {
        if let Some(value) = results.get(input) {
            Some(*value)
        } else {
            input.parse::<u16>().ok()
        }
    };

    match wire.gate.as_str() {
        "_" => {
            let source = wire.inputs[0].clone();
            if let Some(value) = results.get(&source) {
                Some(*value)
            } else if let Ok(value) = source.parse::<u16>() {
                Some(value)
            } else {
                None
            }
        }
        "NOT" => {
            let source = wire.inputs[0].clone();
            if let Some(value) = results.get(&source) {
                Some(!value)
            } else if let Ok(value) = source.parse::<u16>() {
                Some(!value)
            } else {
                None
            }
        }
        "AND" => {
            let source1 = get_value(&wire.inputs[0]);
            let source2 = get_value(&wire.inputs[1]);
            if let (Some(value1), Some(value2)) = (source1, source2) {
                Some(value1 & value2)
            } else {
                None
            }
        }
        "OR" => {
            let source1 = get_value(&wire.inputs[0]);
            let source2 = get_value(&wire.inputs[1]);
            if let (Some(value1), Some(value2)) = (source1, source2) {
                Some(value1 | value2)
            } else {
                None
            }
        }
        "LSHIFT" => {
            let source = get_value(&wire.inputs[0]);
            if let Some(value) = source {
                Some(value << wire.inputs[1].parse::<u16>().unwrap())
            } else {
                None
            }
        }
        "RSHIFT" => {
            let source = get_value(&wire.inputs[0]);
            if let Some(value) = source {
                Some(value >> wire.inputs[1].parse::<u16>().unwrap())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn process_wires(wires: &Vec<Wire>, results: &mut HashMap<String, u16>) {
    while results.len() < wires.len() {
        for wire in wires {
            if let Some(value) = calculate_wire_value(wire, &results) {
                // println!("{} = {}", wire.output, value);
                results.insert(wire.output.clone(), value);
            }
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut wires: Vec<Wire> = Vec::new();

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        wires.push(parse_wire(&line));
    }

    let mut results: HashMap<String, u16> = HashMap::new();
    process_wires(&wires, &mut results);
    let a = results.get("a").unwrap().clone();
    println!("a = {}", a);

    // Part 2: override a with wire b
    let mut results: HashMap<String, u16> = HashMap::new();
    results.insert("b".to_owned(), a);
    process_wires(&wires, &mut results);
    let a = results.get("a").unwrap().clone();
    println!("a = {}", a);
}
