#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use utils::files::read_lines;

fn process_line_part1(line: &str) -> u32 {
    let mut val = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            val = c.to_digit(10).unwrap();
            break;
        }
    }
    for c in line.chars().rev().into_iter() {
        if c.is_digit(10) {
            val = val * 10 + c.to_digit(10).unwrap();
            break;
        }
    }
    return val;
}

lazy_static! {
    static ref DIGITS_MAP: HashMap<&'static str, u32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
}

fn process_line_part2(line: &str) -> u32 {
    let mut idx_s = None;
    let mut first = 0;
    let mut idx_e = None;
    let mut second = 0;
    for (k, v) in DIGITS_MAP.iter() {
        if let Some(idx) = line.find(k) {
            if idx_s.is_none() || idx < idx_s.unwrap() {
                idx_s = Some(idx);
                first = *v;
            }
        }
        if let Some(idx) = line.find(&format!("{v}")) {
            if idx_s.is_none() || idx < idx_s.unwrap() {
                idx_s = Some(idx);
                first = *v;
            }
        }
        if let Some(idx) = line.rfind(k) {
            if idx_e.is_none() || idx > idx_e.unwrap() {
                idx_e = Some(idx);
                second = *v;
            }
        }
        if let Some(idx) = line.rfind(&format!("{v}")) {
            if idx_e.is_none() || idx > idx_e.unwrap() {
                idx_e = Some(idx);
                second = *v;
            }
        }
    }

    return first * 10 + second;
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    if let Ok(lines) = read_lines("day01/input.txt") {
        for line in lines.flatten() {
            part1 += process_line_part1(&line);
            part2 += process_line_part2(&line);
        }
        println!("Part 1 = {part1}");
        println!("Part 2 = {part2}");
    } else {
        println!("input not found!");
    }
}

#[cfg(test)]
mod day01_tests {

    #[test]
    fn test_part1() {
        use super::*;

        let test_cases: HashMap<&str, u32> = HashMap::from([
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ]);

        for (k, v) in test_cases.iter() {
            let result = process_line_part1(k);
            assert_eq!(result, *v);
        }
    }

    #[test]
    fn test_part2() {
        use super::*;
        let test_cases = HashMap::from([
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ]);

        for (k, v) in test_cases.iter() {
            let result = process_line_part2(k);
            assert_eq!(result, *v);
        }
    }
}
