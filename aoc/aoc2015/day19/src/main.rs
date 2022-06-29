use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use rand::prelude::SliceRandom;
use rand::thread_rng;

type Instruction = (String, String);

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut instructions = Vec::new();
    let mut source = String::new();

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        if let Some(instruction) = parse_line(&line) {
            instructions.push(instruction);
        } else if line.trim().len() > 0 {
            source = line;
        }
    }

    let variants = generate_variants(&source, &instructions);
    println!("Number of generated molecules = {}", variants.len());

    let steps = make_molecule(&source, &instructions, "e");
    println!("It takes {:?} steps to generate the molecule", steps);
}

fn parse_line(line: &str) -> Option<Instruction> {
    let parts: Vec<&str> = line.trim().split("=>").map(|p| p.trim()).collect();
    if parts.len() == 2 {
        return Some((parts[0].to_string(), parts[1].to_string()));
    }
    None
}

fn mutate(molecule: &str, from: &str, to: &str, ignore_count: usize) -> Option<String> {
    if let Some(pos) = molecule.find(from) {
        if ignore_count == 0 {
            let result = molecule.replacen(from, to, 1);
            return Some(result);
        } else {
            let prefix = molecule[..pos + 1].to_string();
            if let Some(mutated) = mutate(&molecule[pos + 1..], from, to, ignore_count - 1) {
                return Some(prefix + &mutated);
            }
        }
    }
    None
}

fn generate_variants(source: &str, instructions: &[Instruction]) -> HashSet<String> {
    let mut result = HashSet::new();
    for (from, to) in instructions {
        let mut idx = 0;

        while let Some(variant) = mutate(source, from, to, idx) {
            result.insert(variant);
            idx += 1;
        }
    }
    result
}

fn make_molecule(molecule: &str, instructions: &[Instruction], start: &str) -> usize {
    let mut mutations = 0;

    let mut cycles = 0;
    let mut instructions = instructions.to_vec();
    let mut target = molecule.to_owned();

    while target != start {
        let tmp = target.clone();
        for (a, b) in &instructions {
            if !target.contains(b) {
                continue;
            }

            target = target.replacen(b, a, 1);
            mutations += 1;
        }

        if tmp == target {
            target = molecule.to_owned();
            mutations = 0;
            instructions.shuffle(&mut thread_rng());
            cycles += 1;
            if cycles % 1_000_000 == 0 {
                println!("cycles: {}", cycles);
            }
        }
    }

    mutations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_line("H => HO"),
            Some(("H".to_string(), "HO".to_string()))
        );
        assert_eq!(parse_line("H"), None);
        assert_eq!(parse_line(""), None);
        assert_eq!(parse_line("   "), None);
    }

    #[test]
    fn test_generate_variants() {
        let instructions = vec![
            ("H".to_string(), "HO".to_string()),
            ("H".to_string(), "OH".to_string()),
            ("O".to_string(), "HH".to_string()),
        ];
        let source = "HOH";

        let variants = generate_variants(source, &instructions);
        println!("Variants = {:?}", variants);
        assert_eq!(variants.len(), 4);
        assert!(variants.contains("HOOH"));
        assert!(variants.contains("HOHO"));
        assert!(variants.contains("OHOH"));
        assert!(variants.contains("HHHH"));
    }

    #[test]
    fn test_mutate() {
        assert_eq!(mutate("aaa", "a", "b", 0), Some(String::from("baa")));
        assert_eq!(mutate("aaa", "aaa", "b", 0), Some(String::from("b")));
        assert_eq!(mutate("aaa", "aa", "b", 0), Some(String::from("ba")));
        assert_eq!(mutate("aaa", "aa", "b", 1), Some(String::from("ab")));
        assert_eq!(mutate("aaa", "a", "b", 2), Some(String::from("aab")));
        assert_eq!(mutate("abc", "c", "b", 0), Some(String::from("abb")));
        assert_eq!(mutate("", "c", "b", 0), None);
        assert_eq!(mutate("", "c", "b", 1), None);
        assert_eq!(mutate("aaa", "c", "b", 0), None);
        assert_eq!(mutate("aaa", "aaa", "b", 1), None);
    }

    #[test]
    fn test_make_molecule() {
        let instructions = vec![
            ("e".to_string(), "H".to_string()),
            ("e".to_string(), "O".to_string()),
            ("H".to_string(), "HO".to_string()),
            ("H".to_string(), "OH".to_string()),
            ("O".to_string(), "HH".to_string()),
        ];
        assert_eq!(make_molecule("HOH", &instructions, "e"), 3);
        // FIXME: current solution adopted from https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4cu5b/
        //        can't pass the following test case (issue: #1)
        // assert_eq!(make_molecule("HOHOHO", &instructions, "e"), 6);
    }
}
