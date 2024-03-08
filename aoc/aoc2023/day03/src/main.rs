use std::collections::HashSet;

use utils::files::read_lines;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum EngineItem {
    Symbol(char, isize, isize),
    Number(u32, isize, isize, isize),
}

fn parse_line(line: &str, row_index: usize) -> Vec<EngineItem> {
    let mut items = vec![];
    let mut num = None;

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            num = Some(match num {
                Some((i, l, n)) => (i, l + 1, n * 10 + c.to_digit(10).unwrap()),
                None => (i, 1, c.to_digit(10).unwrap()),
            });
        } else {
            if let Some((idx, len, val)) = num {
                items.push(EngineItem::Number(
                    val,
                    idx as isize,
                    row_index as isize,
                    len,
                ));
                num = None;
            }
            if c != '.' {
                items.push(EngineItem::Symbol(c, i as isize, row_index as isize));
            }
        }
    }

    if let Some((idx, len, val)) = num {
        items.push(EngineItem::Number(
            val,
            idx as isize,
            row_index as isize,
            len,
        ));
    }
    items
}

#[derive(Debug)]
struct Engine {
    rows: Vec<EngineItem>,
}

impl Engine {
    fn make(lines: Vec<String>) -> Self {
        let mut rows = vec![];
        for (row_index, line) in lines.iter().enumerate() {
            let mut current_rows = parse_line(line.as_str(), row_index);
            rows.append(&mut current_rows);
        }
        Self { rows }
    }

    fn get(&self, x: isize, y: isize) -> Option<EngineItem> {
        for item in self.rows.iter() {
            match item {
                EngineItem::Symbol(_, i, j) => {
                    if x == *i && y == *j {
                        return Some(item.clone());
                    }
                }
                EngineItem::Number(_, i, j, l) => {
                    if x >= *i && x < *i + *l && y == *j {
                        return Some(item.clone());
                    }
                }
            }
        }
        None
    }

    fn adjacent_items(&self, item: &EngineItem) -> HashSet<EngineItem> {
        let mut adjaccents = HashSet::new();
        let (xstart, xend);
        let (ystart, yend);
        match item.clone() {
            EngineItem::Symbol(_, x, y) => {
                xstart = x - 1;
                xend = x + 1;
                ystart = y - 1;
                yend = y + 1;
            }
            EngineItem::Number(_, x, y, l) => {
                xstart = x - 1;
                xend = x + l;
                ystart = y - 1;
                yend = y + 1;
            }
        }
        for x in xstart..=xend {
            for y in ystart..=yend {
                if let Some(cur_item) = self.get(x, y) {
                    if cur_item != *item {
                        adjaccents.insert(cur_item);
                    }
                }
            }
        }
        adjaccents
    }

    fn is_valid_number(&self, item: &EngineItem) -> bool {
        match item {
            EngineItem::Symbol(_, _, _) => false,
            EngineItem::Number(_, _, _, _) => {
                let adjaccents = self.adjacent_items(item);
                adjaccents.iter().any(|i| {
                    if let EngineItem::Symbol(_, _, _) = i {
                        true
                    } else {
                        false
                    }
                })
            }
        }
    }

    fn validate_gear(&self, symbol: &EngineItem) -> Option<u32> {
        if let EngineItem::Symbol(c, _, _) = symbol {
            if *c == '*' {
                let adjaccents = self.adjacent_items(symbol);
                let numbers = adjaccents
                    .iter()
                    .filter(|a| {
                        if let EngineItem::Number(_, _, _, _) = *a {
                            true
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<&EngineItem>>();
                if numbers.len() == 2 {
                    return Some(
                        numbers
                            .iter()
                            .map(|n| match n {
                                EngineItem::Symbol(_, _, _) => 1,
                                EngineItem::Number(v, _, _, _) => *v,
                            })
                            .fold(1, |acc, n| acc * n),
                    );
                }
            }
        }
        None
    }
}

fn main() {
    let lines = read_lines("day03/input.txt").expect("Failed to read lines");
    let lines = lines.flatten().collect();
    let engine = Engine::make(lines);
    let mut part1 = 0;
    let mut part2 = 0;
    for item in &engine.rows {
        if let EngineItem::Number(v, _, _, _) = item {
            if engine.is_valid_number(item) {
                part1 += v;
            }
        }
        if let Some(ratio) = engine.validate_gear(item) {
            part2 += ratio;
        }
    }

    println!("Part 1 = {part1}");
    println!("Part 2 = {part2}");
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod day03_tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: Vec<&'static str> = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
    }

    #[test]
    fn test_parse_input() {
        let test_result = vec![
            vec![
                EngineItem::Number(467, 0, 0, 3),
                EngineItem::Number(114, 5, 0, 3),
            ],
            vec![EngineItem::Symbol('*', 3, 1)],
            vec![
                EngineItem::Number(35, 2, 2, 2),
                EngineItem::Number(633, 6, 2, 3),
            ],
            vec![EngineItem::Symbol('#', 6, 3)],
            vec![
                EngineItem::Number(617, 0, 4, 3),
                EngineItem::Symbol('*', 3, 4),
            ],
            vec![
                EngineItem::Symbol('+', 5, 5),
                EngineItem::Number(58, 7, 5, 2),
            ],
            vec![EngineItem::Number(592, 2, 6, 3)],
            vec![EngineItem::Number(755, 6, 7, 3)],
            vec![EngineItem::Symbol('$', 3, 8), EngineItem::Symbol('*', 5, 8)],
            vec![
                EngineItem::Number(664, 1, 9, 3),
                EngineItem::Number(598, 5, 9, 3),
            ],
        ];
        for (i, line) in TEST_INPUT.iter().enumerate() {
            let symbols = parse_line(line, i);
            assert_eq!(symbols, test_result[i]);
        }
    }

    #[test]
    fn test_get_item() {
        let engine = Engine::make(TEST_INPUT.iter().map(|l| l.to_string()).collect());
        println!("{:?}", engine);
        assert_eq!(engine.get(0, 0), Some(EngineItem::Number(467, 0, 0, 3)));
        assert_eq!(engine.get(5, 0), Some(EngineItem::Number(114, 5, 0, 3)));

        assert_eq!(engine.get(3, 1), Some(EngineItem::Symbol('*', 3, 1)));
        assert_eq!(engine.get(4, 1), None);

        assert_eq!(engine.get(2, 2), Some(EngineItem::Number(35, 2, 2, 2)));
        assert_eq!(engine.get(6, 2), Some(EngineItem::Number(633, 6, 2, 3)));

        assert_eq!(engine.get(6, 3), Some(EngineItem::Symbol('#', 6, 3)));

        assert_eq!(engine.get(0, 4), Some(EngineItem::Number(617, 0, 4, 3)));
        assert_eq!(engine.get(3, 4), Some(EngineItem::Symbol('*', 3, 4)));
        assert_eq!(engine.get(4, 4), None);

        assert_eq!(engine.get(5, 5), Some(EngineItem::Symbol('+', 5, 5)));
        assert_eq!(engine.get(7, 5), Some(EngineItem::Number(58, 7, 5, 2)));
        assert_eq!(engine.get(8, 5), Some(EngineItem::Number(58, 7, 5, 2)));

        assert_eq!(engine.get(2, 6), Some(EngineItem::Number(592, 2, 6, 3)));

        assert_eq!(engine.get(6, 7), Some(EngineItem::Number(755, 6, 7, 3)));
        assert_eq!(engine.get(7, 7), Some(EngineItem::Number(755, 6, 7, 3)));
        assert_eq!(engine.get(8, 7), Some(EngineItem::Number(755, 6, 7, 3)));
        assert_eq!(engine.get(9, 7), None);
    }

    #[test]
    fn test_validation() {
        let engine = Engine::make(TEST_INPUT.iter().map(|l| l.to_string()).collect());

        for item in engine.rows.iter() {
            println!("{:?}", item);
            match item {
                EngineItem::Number(114, 5, 0, 3) => {
                    assert_eq!(engine.is_valid_number(item), false);
                }
                EngineItem::Number(58, 7, 5, 2) => {
                    assert_eq!(engine.is_valid_number(item), false);
                }
                EngineItem::Number(_, _, _, _) => {
                    assert_eq!(engine.is_valid_number(item), true);
                }
                EngineItem::Symbol(_, _, _) => {
                    assert_eq!(engine.is_valid_number(item), false);
                }
            }
        }
    }

    #[test]
    fn test_adjaccents() {
        let engine = Engine::make(TEST_INPUT.iter().map(|l| l.to_string()).collect());
        assert_eq!(
            engine.adjacent_items(&EngineItem::Symbol('*', 5, 8)),
            HashSet::from_iter(vec![
                EngineItem::Number(755, 6, 7, 3),
                EngineItem::Number(598, 5, 9, 3),
            ])
        );
        assert_eq!(
            engine.adjacent_items(&EngineItem::Number(114, 5, 0, 3)),
            HashSet::from_iter(vec![])
        );
        assert_eq!(
            engine.adjacent_items(&EngineItem::Symbol('*', 3, 1)),
            HashSet::from_iter(vec![
                EngineItem::Number(467, 0, 0, 3),
                EngineItem::Number(35, 2, 2, 2),
            ])
        );
        assert_eq!(
            engine.adjacent_items(&EngineItem::Number(617, 0, 4, 3)),
            HashSet::from_iter(vec![EngineItem::Symbol('*', 3, 4)])
        );
        assert_eq!(
            engine.adjacent_items(&EngineItem::Number(664, 1, 9, 3)),
            HashSet::from_iter(vec![EngineItem::Symbol('$', 3, 8),])
        );
    }

    #[test]
    fn test_gears() {
        let engine = Engine::make(TEST_INPUT.iter().map(|l| l.to_string()).collect());
        assert_eq!(
            engine.validate_gear(&EngineItem::Symbol('*', 3, 1)),
            Some(16345)
        );
        assert_eq!(engine.validate_gear(&EngineItem::Symbol('*', 3, 4)), None);
        assert_eq!(
            engine.validate_gear(&EngineItem::Symbol('*', 5, 8)),
            Some(451490)
        );
    }
}
