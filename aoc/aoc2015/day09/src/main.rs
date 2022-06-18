use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut cities: HashMap<String, HashMap<String, u32>> = HashMap::new();

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
    let mut path: Vec<String> = Vec::new();
    for (city, _) in &cities {
        path.clear();
        let distance = traverse_cities(city, &cities, &mut path);
        if distance < min_distance {
            min_distance = distance;
        }
    }
    println!("Minimum distance: {} for {:?}", min_distance, path);
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
    city: &str,
    cities: &HashMap<String, HashMap<String, u32>>,
    visit_path: &mut Vec<String>,
) -> u32 {
    visit_path.push(city.to_owned());

    let mut min_distance = std::u32::MAX;
    let mut next_city = String::new();
    for (c, d) in &cities[city] {
        if !visit_path.contains(c) && (*d < min_distance) {
            min_distance = *d;
            next_city = c.to_owned();
        }
    }

    if next_city.is_empty() {
        0
    } else {
        traverse_cities(&next_city, cities, visit_path) + min_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse_cities() {
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

        {
            let mut path = vec![];
            let distance = traverse_cities("London", &cities, &mut path);
            assert_eq!(distance, 605);
        };
        {
            let mut path = vec![];
            let distance = traverse_cities("Dublin", &cities, &mut path);
            assert_eq!(distance, 659);
        };
        {
            let mut path = vec![];
            let distance = traverse_cities("Belfast", &cities, &mut path);
            assert_eq!(distance, 605);
        };
    }
}
