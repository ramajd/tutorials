use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Variant = HashSet<usize>;

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut containers = Vec::new();

    for line in buffer.lines() {
        let container = line
            .expect("Failed to read line")
            .parse::<u32>()
            .expect("Failed to parse container");
        containers.push(container);
    }

    let variants = generate_variants(150, &containers, Variant::new());
    println!("Variant count = {}", variants.len());

    let mut min_len = std::usize::MAX;
    let mut min_count = 0;

    for variant in variants {
        let len = variant.len();
        if len < min_len {
            min_len = len;
            min_count = 1;
        } else if len == min_len {
            min_count += 1;
        }
    }
    // Variant count = 654
    // minimum required container count is 4 with 57 variants
    println!(
        "minimum required container count is {} with {} variants",
        min_len, min_count
    );
}

fn generate_variants(total: u32, containers: &[u32], filled: Variant) -> Vec<Variant> {
    let mut result = Vec::new();
    if total == 0 {
        // println!("{:?} - {}", filled, start_idx);
        return vec![filled];
    }

    for (idx, c) in containers.iter().enumerate() {
        if !filled.contains(&idx) && total >= *c {
            let mut filled = filled.clone();
            filled.insert(idx);
            let created = generate_variants(total - *c, containers, filled);
            for v in created {
                if !result.contains(&v) {
                    result.push(v);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_count() {
        let containers = [20, 15, 10, 5, 5];
        let total = 25;
        let variants = generate_variants(total, &containers, Variant::new());
        assert_eq!(variants.len(), 4);
    }
}
