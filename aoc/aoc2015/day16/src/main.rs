use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut interests: HashMap<String, u32> = HashMap::new();
    interests.insert("children".to_string(), 3);
    interests.insert("cats".to_string(), 7);
    interests.insert("samoyeds".to_string(), 2);
    interests.insert("pomeranians".to_string(), 3);
    interests.insert("akitas".to_string(), 0);
    interests.insert("vizslas".to_string(), 0);
    interests.insert("goldfish".to_string(), 5);
    interests.insert("trees".to_string(), 3);
    interests.insert("cars".to_string(), 2);
    interests.insert("perfumes".to_string(), 1);

    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut aunt_db: HashMap<usize, HashMap<String, u32>> = HashMap::new();

    for (idx, line) in buffer.lines().enumerate() {
        let line = line.expect("Failed to read line");
        let aunt_interests = parse_aunt(&line);
        aunt_db.insert(idx + 1, aunt_interests);
    }

    let selected_aunt = find_aunt(&interests, &aunt_db);
    println!("Selected aunt = {}", selected_aunt);

    let selected_aunt = find_aunt_with_adjusted_rules(&interests, &aunt_db);
    println!("Selected aunt with new rules = {}", selected_aunt);
}

fn find_aunt(
    interests: &HashMap<String, u32>,
    aunt_db: &HashMap<usize, HashMap<String, u32>>,
) -> usize {
    for (aunt_no, aunt_interests) in aunt_db {
        let mut rejected = false;
        for (interest, amount) in aunt_interests {
            if !interests.contains_key(interest) || interests[interest] != *amount {
                rejected = true;
            }
        }
        if !rejected {
            return *aunt_no;
        }
    }
    0
}

fn find_aunt_with_adjusted_rules(
    interests: &HashMap<String, u32>,
    aunt_db: &HashMap<usize, HashMap<String, u32>>,
) -> usize {
    for (aunt_no, aunt_interests) in aunt_db {
        let mut rejected = false;
        for (interest, amount) in aunt_interests {
            if !interests.contains_key(interest) {
                rejected = true;
            } else {
                match interest.as_str() {
                    "cats" | "trees" => {
                        if interests[interest] >= *amount {
                            rejected = true;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if interests[interest] <= *amount {
                            rejected = true;
                        }
                    }
                    _ => {
                        if interests[interest] != *amount {
                            rejected = true;
                        }
                    }
                }
            }
        }
        if !rejected {
            return *aunt_no;
        }
    }
    0
}

fn parse_aunt(line: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    let split = line.split(',');
    for (idx, part) in split.enumerate() {
        let mut parts = part.split(":");
        if idx == 0 {
            parts.next();
        }
        let key = parts.next().unwrap().trim();
        let val = parts.next().unwrap().trim().parse::<u32>().unwrap();
        result.insert(key.to_string(), val);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "Sue 21: vizslas: 3, cars: 7, akitas: 3";

        let mut want: HashMap<String, u32> = HashMap::new();
        want.insert("vizslas".to_owned(), 3);
        want.insert("cars".to_owned(), 7);
        want.insert("akitas".to_owned(), 3);

        assert_eq!(parse_aunt(&input), want);
    }
}
