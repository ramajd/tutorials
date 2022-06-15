mod kit;

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

    let a = kit.get_signal("a").unwrap();
    println!("Signal value for 'a': {}", a);
}
