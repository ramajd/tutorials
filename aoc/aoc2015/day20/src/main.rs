use std::collections::HashSet;

fn main() {
    let input = 29_000_000;

    let house_no = find_house(input, 10, -1);
    println!(
        "House no. {} is the first that receives {} presents.",
        house_no, input
    );

    let house_no = find_house(input, 11, 50);
    println!("after re-arrange the house no. is: {}", house_no);
}

fn find_house(minimum_presents: usize, present_count: usize, limit: i32) -> usize {
    let mut house_no = 1;
    while calculate_presents(house_no, present_count, limit) < minimum_presents {
        if house_no % 100_000 == 0 {
            println!("{} houses calculated", house_no);
        }
        house_no += 1;
    }
    house_no
}

fn calculate_presents(house_no: usize, present_count: usize, limit: i32) -> usize {
    let elves = find_matching_elves(house_no);
    elves.iter().fold(0, |ac, elf| {
        if limit > 0 && house_no / (limit as usize) > *elf {
            ac
        } else {
            ac + *elf * present_count
        }
    })
}

fn find_matching_elves(house_no: usize) -> HashSet<usize> {
    let mut elves = HashSet::new();
    elves.insert(1);
    elves.insert(house_no);

    let total = (house_no as f64).sqrt() as usize;

    for elf in 2..=total {
        if house_no % elf == 0 {
            elves.insert(elf);
            if elf ^ 2 != house_no {
                elves.insert(house_no / elf);
            }
        }
    }
    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presents() {
        assert_eq!(calculate_presents(1, 10, -1), 10);
        assert_eq!(calculate_presents(2, 10, -1), 30);
        assert_eq!(calculate_presents(3, 10, -1), 40);
        assert_eq!(calculate_presents(4, 10, -1), 70);
        assert_eq!(calculate_presents(5, 10, -1), 60);
        assert_eq!(calculate_presents(6, 10, -1), 120);
        assert_eq!(calculate_presents(7, 10, -1), 80);
        assert_eq!(calculate_presents(8, 10, -1), 150);
        assert_eq!(calculate_presents(9, 10, -1), 130);
    }

    #[test]
    fn test_find_house() {
        assert_eq!(find_house(10, 10, -1), 1);
        assert_eq!(find_house(50, 10, -1), 4);
    }
}
