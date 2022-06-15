mod kit;

use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use kit::Kit;

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut kit = Kit::new();

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        kit.add_instruction(&line);
    }

    kit.process_kit(&HashMap::new());
    let a = kit.get_signal("a").unwrap();
    println!("Signal value for 'a': {}", a);

    let mut second_run = HashMap::new();
    second_run.insert(String::from("b"), a);
    kit.process_kit(&second_run);
    println!("Signal value for 'a': {}", kit.get_signal("a").unwrap());
}
