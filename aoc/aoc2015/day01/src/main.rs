use std::fs::File;
use std::io::prelude::*;

fn instruction_weight(ch: char) -> i32 {
    match ch {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn calc_floor(input: &String) -> i32 {
    let mut floor = 0;
    for ch in input.chars() {
        floor += instruction_weight(ch);
    }
    floor
}

fn find_basement_position(input: &String) -> Option<usize> {
    let mut position = 0;
    for (idx, ch) in input.chars().enumerate() {
        position += instruction_weight(ch);
        if position == -1 {
            return Some(idx + 1);
        }
    }
    None
}

fn main() {
    let mut file = File::open("input.txt").expect("Failed to read input");
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let result = calc_floor(&input);
    println!("result = {}", result);

    if let Some(basement_position) = find_basement_position(&input) {
        println!("Basement position = {}", basement_position);
    } else {
        println!("Basement not found");
    }
}
