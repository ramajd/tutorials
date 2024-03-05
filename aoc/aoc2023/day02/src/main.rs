use utils::files::read_lines;

#[derive(Debug, PartialEq, Clone, Copy)]
enum BallColor {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl TryFrom<&str> for BallColor {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() == 2 {
            let count = parts[0].parse::<u32>().unwrap_or(0);
            return match parts[1].to_lowercase().as_str() {
                "red" => Ok(BallColor::Red(count)),
                "green" => Ok(BallColor::Green(count)),
                "blue" => Ok(BallColor::Blue(count)),
                _ => Err(()),
            };
        }
        Err(())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameSet {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        // Self(BallColor::Red(r), BallColor::Green(g), BallColor::Blue(b))
        Self { red, green, blue }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl TryFrom<&str> for GameSet {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let balls: Vec<Result<BallColor, ()>> = value.split(',').map(|p| p.try_into()).collect();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for ball in balls {
            match ball {
                Ok(BallColor::Red(v)) => r = v,
                Ok(BallColor::Green(v)) => g = v,
                Ok(BallColor::Blue(v)) => b = v,
                Err(_) => return Err(()),
            }
        }
        Ok(GameSet::new(r, g, b))
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl Game {
    fn is_possible(&self, set: &GameSet) -> bool {
        for s in &self.sets {
            if s.red > set.red || s.green > set.green || s.blue > set.blue {
                return false;
            }
        }
        true
    }

    fn minimum_set(&self) -> GameSet {
        let mut r = None;
        let mut g = None;
        let mut b = None;
        for s in &self.sets {
            if r.is_none() || r.unwrap() < s.red {
                r = Some(s.red);
            }
            if g.is_none() || g.unwrap() < s.green {
                g = Some(s.green);
            }
            if b.is_none() || b.unwrap() < s.blue {
                b = Some(s.blue);
            }
        }
        GameSet::new(r.unwrap(), g.unwrap(), b.unwrap())
    }
}

impl TryFrom<&str> for Game {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        let parts = value.split(":").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(());
        }

        let id = parts[0]
            .replace("Game", "")
            .trim()
            .parse()
            .map_err(|_| ())?;
        let sets = parts[1]
            .trim()
            .split(";")
            .map(|s| GameSet::try_from(s))
            .collect::<Vec<Result<GameSet, ()>>>();
        if sets.iter().any(|s| s.is_err()) {
            return Err(());
        }
        let sets: Vec<GameSet> = sets.iter().map(|s| s.unwrap()).collect();

        Ok(Game { id, sets })
    }
}

fn main() {
    if let Ok(lines) = read_lines("day02/input.txt") {
        let mut part1 = 0;
        let mut part2 = 0;
        let set = GameSet::new(12, 13, 14);
        for line in lines.flatten() {
            if let Ok(game) = Game::try_from(line.as_ref()) {
                if game.is_possible(&set) {
                    part1 += game.id;
                }
                part2 += game.minimum_set().power();
            }
        }
        println!("Part 1 result = {part1}");
        println!("Part 2 result = {part2}");
    } else {
        println!("Error reading input");
    }
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod day02_tests {

    use std::{collections::HashMap, vec};

    use super::*;

    lazy_static! {
        static ref TEST_CASES: Vec<&'static str> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        static ref TEST_CASE_GAMES: Vec<Game> = vec![
            Game {
                id: 1,
                sets: vec![
                    GameSet::new(4, 0, 3),
                    GameSet::new(1, 2, 6),
                    GameSet::new(0, 2, 0),
                ],
            },
            Game {
                id: 2,
                sets: vec![
                    GameSet::new(0, 2, 1),
                    GameSet::new(1, 3, 4),
                    GameSet::new(0, 1, 1),
                ],
            },
            Game {
                id: 3,
                sets: vec![
                    GameSet::new(20, 8, 6),
                    GameSet::new(4, 13, 5),
                    GameSet::new(1, 5, 0),
                ],
            },
            Game {
                id: 4,
                sets: vec![
                    GameSet::new(3, 1, 6),
                    GameSet::new(6, 3, 0),
                    GameSet::new(14, 3, 15),
                ],
            },
            Game {
                id: 5,
                sets: vec![GameSet::new(6, 3, 1), GameSet::new(1, 2, 2)],
            },
        ];
    }

    #[test]
    fn test_parse_ball() {
        let test_cases = HashMap::from([
            ("   3 blue", BallColor::Blue(3)),
            ("10  GrEEn", BallColor::Green(10)),
            ("0 Red  ", BallColor::Red(0)),
        ]);
        for (k, v) in test_cases {
            assert_eq!(BallColor::try_from(k), Ok(v));
        }
    }

    #[test]
    fn test_parse_game_set() {
        let test_cases = HashMap::from([
            ("8 green, 6 blue, 20 red", GameSet::new(20, 8, 6)),
            (" 5 blue, 4 red, 13 green", GameSet::new(4, 13, 5)),
            (" 5 green, 1 red", GameSet::new(1, 5, 0)),
        ]);
        for (k, v) in test_cases.iter() {
            let g = GameSet::try_from(*k);
            assert!(g.is_ok_and(|g| g == *v));
        }
    }

    #[test]
    fn test_parse_game() {
        for (i, s) in TEST_CASES.iter().enumerate() {
            let g = Game::try_from(*s);
            assert!(g.is_ok_and(|g| g == TEST_CASE_GAMES[i]));
        }
    }

    #[test]
    fn test_possibility() {
        let set = GameSet::new(12, 13, 14);

        let game_results = vec![true, true, false, false, true];

        for (i, g) in TEST_CASE_GAMES.iter().enumerate() {
            assert_eq!(g.is_possible(&set), game_results[i]);
        }
    }

    #[test]
    fn test_minimum_possibility() {
        let min_sets = vec![
            GameSet::new(4, 2, 6),
            GameSet::new(1, 3, 4),
            GameSet::new(20, 13, 6),
            GameSet::new(14, 3, 15),
            GameSet::new(6, 3, 2),
        ];
        for (i, g) in TEST_CASE_GAMES.iter().enumerate() {
            assert_eq!(g.minimum_set(), min_sets[i]);
        }
    }
}
