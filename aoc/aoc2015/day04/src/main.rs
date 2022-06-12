use std::{ops::Range, thread};

use md5;

fn calculate_hash(secret: &str, number: usize) -> String {
    let hash = md5::compute(format!("{}{}", secret, number).as_bytes());
    let mut hash_str = String::new();
    for byte in hash.as_slice() {
        hash_str.push_str(&format!("{:02x}", byte));
    }
    hash_str
}

fn process_range(secret: &str, predict: &str, range: Range<usize>) -> Option<(usize, String)> {
    for i in range {
        let hash = calculate_hash(secret, i);
        if hash.starts_with(predict) {
            return Some((i, hash));
        }        
    }
    None
}

fn mine_block(secret: &'static str, predict: &'static str) -> (usize, String) {
    let cpu_count = num_cpus::get();
    let step = 100_000;

    let mut counter = 0;

    loop {
        let mut handles = Vec::new();
        for _ in 0..cpu_count {
            let range = counter..counter + step;

            counter += step;
            let handle = thread::spawn(move || process_range(secret, predict, range));
            handles.push(handle);
        }
        for handle in handles {
            if let Some((number, hash)) = handle.join().unwrap() {
                return (number, hash);
            }
        }
    }
}

fn main() {
    let secret = "iwrupvqb";
    let part1 = mine_block(secret, "00000");
    println!("Part1: {} - {}", part1.0, part1.1);

    let part2 = mine_block(secret, "000000");
    println!("Part2: {} - {}", part2.0, part2.1);
}
