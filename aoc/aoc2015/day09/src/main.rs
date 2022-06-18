use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type CityMap = HashMap<String, HashMap<String, u32>>;

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut cities: CityMap = HashMap::new();

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        let (from, to, distance) = parse_line(&line);

        cities
            .entry(from.to_owned())
            .or_insert(HashMap::new())
            .insert(to.to_owned(), distance);
        cities
            .entry(to.to_owned())
            .or_insert(HashMap::new())
            .insert(from.to_owned(), distance);
    }

    let mut min_distance = std::u32::MAX;
    let mut min_path: Vec<String> = Vec::new();
    let mut max_distance = 0;
    let mut max_path: Vec<String> = Vec::new();
    for (city, _) in &cities {
        min_path.clear();
        let d = traverse_cities(city, &cities, &mut min_path, |a, b| a < b);
        if d < min_distance {
            min_distance = d;
        }
        max_path.clear();
        let d = traverse_cities(city, &cities, &mut max_path, |a, b| a > b);
        if d > max_distance {
            max_distance = d;
        }
    }
    println!("Minimum distance: {} for {:?}", min_distance, min_path);
    println!("Maximum distance: {} for {:?}", max_distance, max_path);
}

fn parse_line(line: &str) -> (String, String, u32) {
    let mut parts = line.split(" = ");
    let routes = parts.next().unwrap().trim();
    let distance = parts.next().unwrap().trim().parse::<u32>().unwrap();

    let mut parts = routes.split(" to ");
    let from = parts.next().unwrap().trim().to_string();
    let to = parts.next().unwrap().trim().to_string();
    (from, to, distance)
}

fn traverse_cities(
    start: &str,
    cities: &CityMap,
    visited: &mut Vec<String>,
    predict: fn(u32, u32) -> bool,
) -> u32 {
    visited.push(start.to_owned());

    let mut distance: Option<u32> = None;
    let mut next_city = String::new();
    for (city, d) in &cities[start] {
        if !visited.contains(city) && (distance.is_none() || predict(*d, distance.unwrap())) {
            distance = Some(*d);
            next_city = city.to_owned();
        }
    }
    if distance.is_none() {
        0
    } else {
        distance.unwrap() + traverse_cities(&next_city, cities, visited, predict)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse_with_min_distance() {
        let lines = vec![
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];
        let mut cities: HashMap<String, HashMap<String, u32>> = HashMap::new();
        for line in lines {
            let (from, to, distance) = parse_line(&line);
            cities
                .entry(from.to_owned())
                .or_insert(HashMap::new())
                .insert(to.to_owned(), distance);
            cities
                .entry(to.to_owned())
                .or_insert(HashMap::new())
                .insert(from.to_owned(), distance);
        }

        let mut path = vec![];
        let distance = traverse_cities("London", &cities, &mut path, |a, b| a < b);
        assert_eq!(distance, 605);

        let mut path = vec![];
        let distance = traverse_cities("Dublin", &cities, &mut path, |a, b| a < b);
        assert_eq!(distance, 659);

        let mut path = vec![];
        let distance = traverse_cities("Belfast", &cities, &mut path, |a, b| a < b);
        assert_eq!(distance, 605);
    }

    #[test]
    fn test_traverse_with_max_distance() {
        let lines = vec![
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];
        let mut cities: HashMap<String, HashMap<String, u32>> = HashMap::new();
        for line in lines {
            let (from, to, distance) = parse_line(&line);
            cities
                .entry(from.to_owned())
                .or_insert(HashMap::new())
                .insert(to.to_owned(), distance);
            cities
                .entry(to.to_owned())
                .or_insert(HashMap::new())
                .insert(from.to_owned(), distance);
        }

        let mut path = vec![];
        let distance = traverse_cities("London", &cities, &mut path, |a, b| a > b);
        assert_eq!(distance, 659);

        let mut path = vec![];
        let distance = traverse_cities("Dublin", &cities, &mut path, |a, b| a > b);
        assert_eq!(distance, 982);

        let mut path = vec![];
        let distance = traverse_cities("Belfast", &cities, &mut path, |a, b| a > b);
        assert_eq!(distance, 982);
    }
}
