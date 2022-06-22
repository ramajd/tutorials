use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type HappinessDB = HashMap<String, HashMap<String, i32>>;

fn main() {
    let file = File::open("input.txt").expect("Failed to read input.txt");
    let reader = BufReader::new(file);

    let mut records = HappinessDB::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if let Some((from, to, happiness)) = parse_line(&line) {
            let entry = records.entry(from).or_insert(HashMap::new());
            entry.insert(to, happiness);
        }
    }

    let total_happiness = find_maximum_happiness(&records);
    println!("Total Happiness = {}", total_happiness);

    let mut people: Vec<String> = Vec::new();
    for person in records.keys() {
        if !people.contains(person) {
            people.push(person.clone());
        }
        for person in records[person].keys() {
            if !people.contains(&person) {
                people.push(person.clone());
            }
        }
    }
    for p in people {
        records
            .entry(String::from("ME"))
            .or_insert(HashMap::new())
            .insert(p.clone(), 0);
        records
            .entry(p)
            .or_insert(HashMap::new())
            .insert(String::from("ME"), 0);
    }
    let total_happiness = find_maximum_happiness(&records);
    println!("Total Happiness with me = {}", total_happiness);
}

fn find_maximum_happiness(records: &HappinessDB) -> i32 {
    let mut max_happiness = std::i32::MIN;

    for (p, _) in records {
        let mut placed_list = vec![];
        if let Some(happiness) = place_around_table(p, &mut placed_list, records) {
            if happiness > max_happiness {
                max_happiness = happiness;
            }
        }
    }
    max_happiness
}

fn calculate_happiness(place_order: &Vec<&str>, relations: &HappinessDB) -> i32 {
    if place_order.len() <= 1 {
        return 0;
    }
    let first = &place_order[0];
    let mut prv = first;
    let mut sum = 0;
    for person in place_order.iter().skip(1) {
        sum += relations[*prv][*person];
        sum += relations[*person][*prv];
        prv = person;
    }
    if place_order.len() > 2 {
        sum += relations[*first][*prv];
        sum += relations[*prv][*first]
    }
    sum
}

fn place_around_table<'a>(
    person: &'a str,
    placed_list: &mut Vec<&'a str>,
    relations: &'a HappinessDB,
) -> Option<i32> {
    placed_list.push(person);
    if placed_list.len() == relations.len() {
        let total = calculate_happiness(placed_list, relations);
        return Some(total);
    }

    let mut max_happiness = std::i32::MIN;
    let mut next: Option<&str> = None;
    for p in relations[person].keys() {
        let mut tmp_list = placed_list.clone();
        if !tmp_list.contains(&p.as_str()) {
            if let Some(happiness) = place_around_table(p, &mut tmp_list, relations) {
                if happiness > max_happiness {
                    max_happiness = happiness;
                    next = Some(p);
                }
            }
        }
    }

    if let Some(p) = next {
        return place_around_table(p, placed_list, relations);
    }
    None
}

fn parse_line(line: &str) -> Option<(String, String, i32)> {
    let mut split = line.trim().split(' ');
    let from = split.next()?;
    split.next();
    let status = split.next()?;
    let level = split.next()?;
    let to = split.next_back()?.replace(".", "");

    let level = if status == "gain" {
        level.parse::<i32>().unwrap()
    } else {
        level.parse::<i32>().unwrap() * -1
    };

    Some((from.to_owned(), to.to_owned(), level))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_positive() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.";
        assert_eq!(
            parse_line(input),
            Some(("Alice".to_string(), "Bob".to_string(), 54))
        );
    }

    #[test]
    fn test_parser_negative() {
        let input = "Bob would lose 63 happiness units by sitting next to David.";
        assert_eq!(
            parse_line(input),
            Some(("Bob".to_string(), "David".to_string(), -63))
        );
    }

    fn prepare_test_db() -> HappinessDB {
        let lines = vec![
            "Alice would gain 54 happiness units by sitting next to Bob.",
            "Alice would lose 79 happiness units by sitting next to Carol.",
            "Alice would lose 2 happiness units by sitting next to David.",
            "Bob would gain 83 happiness units by sitting next to Alice.",
            "Bob would lose 7 happiness units by sitting next to Carol.",
            "Bob would lose 63 happiness units by sitting next to David.",
            "Carol would lose 62 happiness units by sitting next to Alice.",
            "Carol would gain 60 happiness units by sitting next to Bob.",
            "Carol would gain 55 happiness units by sitting next to David.",
            "David would gain 46 happiness units by sitting next to Alice.",
            "David would lose 7 happiness units by sitting next to Bob.",
            "David would gain 41 happiness units by sitting next to Carol.",
        ];
        let mut db = HappinessDB::new();
        for line in lines {
            if let Some((from, to, lvl)) = parse_line(line) {
                let rec = db.entry(from).or_insert(HashMap::new());
                rec.insert(to, lvl);
            }
        }
        db
    }

    #[test]
    fn test_calculate_happiness() {
        let relations = prepare_test_db();

        assert!(calculate_happiness(&vec!["Alice", "Bob", "Carol", "David"], &relations) == 330);
        assert!(calculate_happiness(&vec![], &relations) == 0);
        assert!(calculate_happiness(&vec!["Alice"], &relations) == 0);
        assert!(calculate_happiness(&vec!["Alice", "Bob"], &relations) == 54 + 83);
    }

    #[test]
    fn test_place_around_table() {
        //      +41 +46
        // +55   David    -2
        // Carol       Alice
        // +60    Bob    +54
        //      -7  +83
        let relations = prepare_test_db();
        let mut place_order = vec!["Alice"];
        let happiness = place_around_table("Bob", &mut place_order, &relations);
        assert_eq!(happiness, Some(330));
        assert_eq!(place_order, vec!["Alice", "Bob", "Carol", "David"]);
    }

    #[test]
    fn test_maximum_happiness() {
        let records = prepare_test_db();
        assert_eq!(find_maximum_happiness(&records), 330);
    }
}
