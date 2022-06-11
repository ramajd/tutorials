use std::fs::File;
use std::io::Read;

use crate::board::Board;

mod board;

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    let mut input = String::new();

    file.read_to_string(&mut input)
        .expect("Failed to read input");

    let mut board = Board::new();

    board.add_worker("santa");
    board.add_worker("robot");

    board.perform_transitions(input);
    println!("Visit count: {}", board.visit_count());
}
