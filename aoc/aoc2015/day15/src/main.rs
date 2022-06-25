use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Add;
use std::vec;

#[derive(Debug, PartialEq)]
struct Score {
    cap: i32,
    dur: i32,
    flv: i32,
    txt: i32,
}

impl Score {
    fn new(cap: i32, dur: i32, flv: i32, txt: i32) -> Self {
        Self { cap, dur, flv, txt }
    }

    fn value(&self) -> i32 {
        let cap = if self.cap < 0 { 0 } else { self.cap };
        let dur = if self.dur < 0 { 0 } else { self.dur };
        let txt = if self.txt < 0 { 0 } else { self.txt };
        let flv = if self.flv < 0 { 0 } else { self.flv };

        cap * dur * txt * flv
    }
}

impl Add for Score {
    type Output = Score;

    fn add(self, rhs: Self) -> Self::Output {
        Score {
            cap: self.cap + rhs.cap,
            dur: self.dur + rhs.dur,
            flv: self.flv + rhs.flv,
            txt: self.txt + rhs.txt,
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Ingredient {
    name: String,
    cap: i32,
    dur: i32,
    txt: i32,
    flv: i32,
    cal: i32,
}

impl Ingredient {
    fn new(name: &str, cap: i32, dur: i32, txt: i32, flv: i32, cal: i32) -> Self {
        Self {
            name: name.to_owned(),
            cap,
            dur,
            txt,
            flv,
            cal,
        }
    }

    pub(crate) fn score(&self, amount: i32) -> Score {
        Score {
            cap: self.cap * amount,
            dur: self.dur * amount,
            txt: self.txt * amount,
            flv: self.flv * amount,
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut ingredients = Vec::new();

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        if let Some(ingredient) = parse_input(&line) {
            ingredients.push(ingredient);
        }
    }

    let total_score = make_optimal_cookie(&ingredients, 100);
    println!("Total score for cookie = {}", total_score);

    let total_score = make_optimal_cookie_with_calorie_limit(&ingredients, 100, 500);
    println!(
        "Total score for cookie with {} calories = {}",
        500, total_score
    );
}

fn parse_input(line: &str) -> Option<Ingredient> {
    let extract_value = |prop: &str| {
        prop.trim()
            .split(' ')
            .next_back()
            .unwrap()
            .parse::<i32>()
            .unwrap()
    };

    let mut split = line.split(":");
    let name = split.next().unwrap().trim();
    let properties = split.next().unwrap().trim();
    let mut props = properties.split(',');
    let capacity = extract_value(props.next().unwrap());
    let durability = extract_value(props.next().unwrap());
    let flavor = extract_value(props.next().unwrap());
    let texture = extract_value(props.next().unwrap());
    let calories = extract_value(props.next().unwrap());
    Some(Ingredient::new(
        name, capacity, durability, flavor, texture, calories,
    ))
}

fn make_optimal_cookie(ingredients: &Vec<Ingredient>, total: i32) -> i32 {
    let variants = generate_variants(vec![], ingredients.len(), total);
    let mut max_score = Score::new(0, 0, 0, 0);

    for variant in variants {
        let score = calculate_score(&variant, &ingredients);
        if score > max_score {
            max_score = score;
        }
    }

    max_score.value()
}

fn make_optimal_cookie_with_calorie_limit(
    ingredients: &Vec<Ingredient>,
    total: i32,
    calorie_limit: i32,
) -> i32 {
    let variants = generate_variants(vec![], ingredients.len(), total);
    let mut max_score = Score::new(0, 0, 0, 0);

    for variant in variants {
        let score = calculate_score(&variant, &ingredients);
        let calorie = calculate_calorie(&variant, &ingredients);
        if (calorie == calorie_limit) && score > max_score {
            max_score = score;
        }
    }
    max_score.value()
}

fn generate_variants(prefix: Vec<i32>, len: usize, total: i32) -> Vec<Vec<i32>> {
    if len == 1 {
        let mut result = prefix;
        result.push(total);
        return vec![result];
    }

    let mut result = Vec::new();
    for i in 0..=total {
        let mut prefix = prefix.clone();
        prefix.push(i);
        let mut variants = generate_variants(prefix, len - 1, total - i);
        result.append(&mut variants);
    }
    result
}

fn calculate_score(variant: &Vec<i32>, ingredients: &Vec<Ingredient>) -> Score {
    let mut result = Score::new(0, 0, 0, 0);
    for (idx, amount) in variant.iter().enumerate() {
        result = result + ingredients[idx].score(*amount);
    }
    result
}

fn calculate_calorie(variant: &[i32], ingredients: &[Ingredient]) -> i32 {
    let mut result = 0;
    for (idx, amount) in variant.iter().enumerate() {
        result += ingredients[idx].cal * *amount;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let line = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";
        assert_eq!(
            parse_input(&line),
            Some(Ingredient::new("Butterscotch", -1, -2, 6, 3, 8))
        );

        let line = "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        assert_eq!(
            parse_input(&line),
            Some(Ingredient::new("Cinnamon", 2, 3, -2, -1, 3))
        );
    }

    #[test]
    fn test_variant_generation() {
        let result = generate_variants(vec![], 1, 3);
        assert_eq!(result.len(), 1);
        assert_eq!(result, vec![vec![3]]);

        let result = generate_variants(vec![], 2, 3);
        assert_eq!(result.len(), 4);
        assert_eq!(result, vec![vec![0, 3], vec![1, 2], vec![2, 1], vec![3, 0]]);

        let result = generate_variants(vec![], 3, 3);
        assert_eq!(result.len(), 10);
        assert_eq!(
            result,
            vec![
                vec![0, 0, 3],
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![0, 3, 0],
                vec![1, 0, 2],
                vec![1, 1, 1],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
                vec![3, 0, 0],
            ]
        )
    }

    #[test]
    fn test_make_optimal_cookie() {
        let ingredients = vec![
            Ingredient::new("Butterscotch", -1, -2, 6, 3, 8),
            Ingredient::new("Cinnamon", 2, 3, -2, -1, 3),
        ];

        assert_eq!(make_optimal_cookie(&ingredients, 100), 62842880);
    }

    #[test]
    fn test_make_optimal_cookie_with_calorie_limit() {
        let ingredients = vec![
            Ingredient::new("Butterscotch", -1, -2, 6, 3, 8),
            Ingredient::new("Cinnamon", 2, 3, -2, -1, 3),
        ];
        assert_eq!(
            make_optimal_cookie_with_calorie_limit(&ingredients, 100, 500),
            57600000
        );
    }
}
