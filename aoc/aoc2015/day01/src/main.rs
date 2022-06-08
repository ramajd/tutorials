use std::fs::File;
use std::io::prelude::*;

fn calc_floor(input: &String) -> i32 {
    let mut floor = 0;
    for ch in input.chars() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }
    floor
}

fn main() {
    let mut file = File::open("input.txt").expect("Failed to read input");
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let result = calc_floor(&input);
    println!("result = {}", result);
}
