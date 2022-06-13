mod lights;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::lights::{extract_command, Grid};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut grid = Grid::new(1000);

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        let command = extract_command(&line).expect("Failed to parse command");
        grid.apply_command(command)
            .expect("Failed to apply command");
    }

    println!("Number of lit lights: {}", grid.lit_count());
    println!("Total brightness: {}", grid.total_brightness());
}
