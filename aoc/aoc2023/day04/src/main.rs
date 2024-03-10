use std::collections::HashSet;

use utils::files::read_lines;

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    result: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let mut score = 0;

        for number in &self.numbers {
            if let Some(_) = self.result.iter().find(|&n| *n == *number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        score
    }
}

impl TryFrom<&str> for Card {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(":").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(());
        }
        let id = parts[0]
            .replace("Card", "")
            .trim()
            .parse::<u32>()
            .map_err(|_| ())?;
        let parts = parts[1].split("|").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(());
        }
        let result: Vec<u32> = parts[0]
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let numbers: Vec<u32> = parts[1]
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        Ok(Self {
            id,
            result,
            numbers,
        })
    }
}

fn main() {
    let lines = read_lines("day04/input.txt").expect("Failed to read input");
    let mut part1 = 0;
    for line in lines.flatten() {
        let card =
            Card::try_from(line.as_str()).expect(format!("invalid card string: {line}").as_str());
        // println!("{:?}", card);
        part1 += card.score();
    }
    println!("Part 1 = {part1}");
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod day04_tests {

    use std::vec;

    use super::*;

    lazy_static! {
        static ref TEST_CASES: Vec<&'static str> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
    }

    #[test]
    fn test_parse_card() {
        let results = vec![
            Card {
                id: 1,
                result: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                id: 2,
                result: vec![13, 32, 20, 16, 61],
                numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Card {
                id: 3,
                result: vec![1, 21, 53, 59, 44],
                numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
            Card {
                id: 4,
                result: vec![41, 92, 73, 84, 69],
                numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            },
            Card {
                id: 5,
                result: vec![87, 83, 26, 28, 32],
                numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            },
            Card {
                id: 6,
                result: vec![31, 18, 13, 56, 72],
                numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ];
        for (idx, case) in TEST_CASES.iter().enumerate() {
            println!("{case}");
            let card = Card::try_from(*case).unwrap();
            assert_eq!(card, results[idx]);
        }
    }

    #[test]
    fn test_score() {
        let scores = vec![8, 2, 2, 1, 0, 0];
        for (idx, case) in TEST_CASES.iter().enumerate() {
            println!("{case}");
            let card = Card::try_from(*case).unwrap();
            assert_eq!(card.score(), scores[idx]);
        }
    }
}
