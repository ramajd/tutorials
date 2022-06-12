use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn vowel_count(word: &str) -> usize {
    let mut count = 0;
    for c in word.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            count += 1;
        }
    }
    count
}

fn has_double_letter(word: &str) -> bool {
    let mut previous_letter = ' ';
    for c in word.chars() {
        if c == previous_letter {
            return true;
        }
        previous_letter = c;
    }
    false
}

fn contains_forbidden_words(word: &str) -> bool {
    let forbidden_words = ["ab", "cd", "pq", "xy"];
    for w in forbidden_words {
        if word.contains(w) {
            return true;
        }
    }
    false
}

fn is_nice(word: &str) -> bool {
    let vowels = vowel_count(word);
    let has_double = has_double_letter(word);
    let is_forbidden = contains_forbidden_words(word);

    !is_forbidden && vowels >= 3 && has_double
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut nice_count = 0;

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        if is_nice(&line) {
            nice_count += 1;
            println!("{}", line);
        }
    }
    println!("nice count: {}", nice_count);
}
