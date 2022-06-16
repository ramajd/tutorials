use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to read input.txt");
    let buffer = BufReader::new(file);
    let mut total_difference = 0;
    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        let code_count = line.len();
        let memory_count = process_memory_chars(&line);
        total_difference += code_count - (memory_count) as usize;
    }
    println!("Total difference: {}", total_difference);
}

fn process_memory_chars(line: &str) -> u16 {
    let mut count = 0;
    let mut ignore_count = 0;

    for c in line.chars() {
        match c {
            '\\' => {
                if ignore_count > 0 {
                    count += 1;
                    ignore_count -= 1;
                } else {
                    ignore_count += 1;
                }
            }
            'x' => {
                count += 1;
                if ignore_count > 0 {
                    ignore_count += 1;
                }
            }
            '"' => {
                if ignore_count > 0 {
                    count += 1;
                    ignore_count -= 1;
                }
            }
            _ => {
                if ignore_count > 0 {
                    ignore_count -= 1;
                } else {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_quote() {
        assert_eq!(process_memory_chars(r#""""#), 0);
        assert_eq!(process_memory_chars(r#"\""#), 1);
        assert_eq!(process_memory_chars(r#""abc""#), 3);
    }

    #[test]
    fn test_backslash() {
        assert_eq!(process_memory_chars(r#""aaa\"aaa""#), 7);
    }

    #[test]
    fn test_hex_escape() {
        assert_eq!(process_memory_chars(r#""\x27"#), 1);
    }
}
