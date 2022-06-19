fn main() {
    let input = "1113122113";
    let result = iterate(input, 40);
    println!("Sequence length after 40 iterations = {}", result.len());

    let result = iterate(input, 50);
    println!("Sequence length after 50 iterations = {}", result.len());
}

fn iterate(input: &str, count: u32) -> String {
    let mut result = input.to_string();
    for _ in 0..count {
        result = process_sequence(&result);
    }
    result
}

fn process_sequence(input: &str) -> String {
    let mut result = String::new();
    let mut prv_char = ' ';
    let mut count = 0;
    for c in input.chars() {
        if c != prv_char && count > 0 {
            let part = format!("{}{}", count, prv_char);
            // print!("[{}] ", part);
            result.push_str(&part);
            count = 0;
        }
        prv_char = c;
        count += 1;
    }
    if count > 0 {
        let part = format!("{}{}", count, prv_char);
        // println!("[{}]", part);

        result.push_str(&part);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_sequence() {
        assert_eq!(process_sequence("1"), String::from("11")); // (1 copy of digit 1).
        assert_eq!(process_sequence("11"), String::from("21")); // (2 copies of digit 1).
        assert_eq!(process_sequence("21"), String::from("1211")); // (one 2 followed by one 1).
        assert_eq!(process_sequence("1211"), String::from("111221")); // (one 1, one 2, and two 1s).
        assert_eq!(process_sequence("111221"), String::from("312211")); // (three 1s, two 2s, and one 1).
    }
}
