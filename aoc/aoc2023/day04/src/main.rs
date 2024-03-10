use std::collections::HashMap;

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

    fn count_matching(&self) -> u32 {
        let mut count = 0;
        for number in &self.numbers {
            if let Some(_) = self.result.iter().find(|&n| *n == *number) {
                count += 1;
            }
        }
        count
    }

    fn next_cards(&self) -> Vec<u32> {
        let count = self.count_matching();
        let start = self.id + 1;
        let end = start + count;
        (start..end).collect()
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

struct ScoreBoard {
    scores: HashMap<u32, u32>,
}

impl ScoreBoard {
    fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    fn append(&mut self, card: &Card) {
        let count = match self.scores.get(&card.id) {
            None => 1,
            Some(c) => *c + 1,
        };
        self.scores.insert(card.id, count);

        let nexts = card.next_cards();
        for id in nexts {
            if let Some(c) = self.scores.get_mut(&id) {
                *c += count;
            } else {
                self.scores.insert(id, count);
            }
        }
    }

    fn score(&self) -> u32 {
        let mut sum = 0;
        for (_, &v) in self.scores.iter() {
            sum += v;
        }
        sum
    }
}

fn main() {
    let lines = read_lines("day04/input.txt").expect("Failed to read input");
    let mut part1 = 0;
    let mut part2 = ScoreBoard::new();
    for line in lines.flatten() {
        let card = Card::try_from(line.as_str()).unwrap();
        part1 += card.score();
        part2.append(&card);
    }
    println!("Part 1 = {part1}");
    println!("Part 2 = {}", part2.score());
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod day04_tests {

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

    #[test]
    fn test_count() {
        let counts = vec![4, 2, 2, 1, 0, 0];
        for (idx, case) in TEST_CASES.iter().enumerate() {
            println!("{case}");
            let card = Card::try_from(*case).unwrap();
            assert_eq!(card.count_matching(), counts[idx]);
        }
    }

    #[test]
    fn test_nexts() {
        let nexts = vec![
            vec![2, 3, 4, 5],
            vec![3, 4],
            vec![4, 5],
            vec![5],
            vec![],
            vec![],
        ];
        for (idx, case) in TEST_CASES.iter().enumerate() {
            println!("{case}");
            let card = Card::try_from(*case).unwrap();
            assert_eq!(card.next_cards(), nexts[idx]);
        }
    }

    #[test]
    fn test_count_total() {
        let mut board = ScoreBoard::new();

        for &case in TEST_CASES.iter() {
            println!("{case}");
            let card = Card::try_from(case).unwrap();
            board.append(&card);
        }
        assert_eq!(board.score(), 30);
    }
}
