use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let end_time = 2503;

    let mut records: HashMap<String, (u32, u32, u32)> = HashMap::new();

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        if let Some((deer, speed, duration, rest)) = parse_input(&line) {
            records.insert(deer, (speed, duration, rest));
        }
    }

    let (winner, distance) = perform_race(&records, end_time);
    println!("winner is [{}] with distance = {}", winner, distance);

    let (winner, points) = perform_point_race(&records, end_time);
    println!("winner is [{}] with points = {}", winner, points);
}

fn perform_race(records: &HashMap<String, (u32, u32, u32)>, end_time: u32) -> (String, u32) {
    let mut winner = String::new();
    let mut max_distance = 0;
    for (deer, (speed, duration, rest)) in records {
        let distance = calculate_distance(*speed, *duration, *rest, end_time);
        if distance > max_distance {
            max_distance = distance;
            winner = deer.clone();
        }
    }
    (winner, max_distance)
}

fn perform_point_race(records: &HashMap<String, (u32, u32, u32)>, end_time: u32) -> (String, u32) {
    let mut scores: HashMap<String, u32> = HashMap::new();

    for i in 1..=end_time {
        let mut winners: Vec<String> = Vec::new();
        let mut max_distance = 0;
        for (deer, (speed, duration, rest)) in records {
            let d = calculate_distance(*speed, *duration, *rest, i);
            if d > max_distance {
                max_distance = d;
                winners.clear();
                winners.push(deer.clone());
            } else if d == max_distance {
                winners.push(deer.clone());
            }
        }
        for winner in winners {
            scores.entry(winner).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    let res = scores
        .iter()
        .reduce(|ac, v| if v.1 > ac.1 { v } else { ac })
        .unwrap();

    (res.0.to_string(), *res.1)
}

fn calculate_distance(speed: u32, duration: u32, rest: u32, end_time: u32) -> u32 {
    let num_runs = end_time / (duration + rest);
    let remaining = end_time % (duration + rest);
    let total_duration = (num_runs * duration)
        + if remaining >= duration {
            duration
        } else {
            remaining
        };
    speed * total_duration
}

fn parse_input(line: &str) -> Option<(String, u32, u32, u32)> {
    let mut split = line.trim().split(' ');
    let deer = split.next()?;
    split.next();
    split.next();
    let speed = split.next()?.parse::<u32>().unwrap();
    split.next();
    split.next();
    let duration = split.next()?.parse::<u32>().unwrap();
    split.next_back();
    let end_time = split.next_back()?.parse::<u32>().unwrap();

    Some((deer.to_owned(), speed, duration, end_time))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        assert_eq!(parse_input(input), Some(("Comet".to_owned(), 14, 10, 127)));
        let input = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(parse_input(input), Some(("Dancer".to_owned(), 16, 11, 162)));
    }

    #[test]
    fn test_distance() {
        assert_eq!(calculate_distance(14, 10, 127, 1000), 1120);
        assert_eq!(calculate_distance(16, 11, 162, 1000), 1056);
    }

    #[test]
    fn test_race() {
        let mut records: HashMap<String, (u32, u32, u32)> = HashMap::new();
        records.insert("Comet".to_owned(), (14, 10, 127));
        records.insert("Dancer".to_owned(), (16, 11, 162));

        assert_eq!(perform_race(&records, 1000), ("Comet".to_string(), 1120));
    }

    #[test]
    fn test_race_points() {
        let mut records: HashMap<String, (u32, u32, u32)> = HashMap::new();
        records.insert("Comet".to_owned(), (14, 10, 127));
        records.insert("Dancer".to_owned(), (16, 11, 162));

        assert_eq!(
            perform_point_race(&records, 1000),
            ("Dancer".to_string(), 689)
        );
    }
}
