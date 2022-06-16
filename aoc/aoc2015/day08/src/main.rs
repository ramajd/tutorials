use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to read input.txt");
    let buffer = BufReader::new(file);

    let mut total_difference = 0;
    let mut total_new_representation_difference = 0;

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        let code_count = line.len();
        let memory_count = process_memory_chars(&line);
        total_difference += code_count - (memory_count) as usize;

        let new_representation = convert_to_new_representation(&line);
        total_new_representation_difference += new_representation.len() - code_count;
    }
    println!("Total difference: {}", total_difference);
    println!(
        "Total difference with new representation: {}",
        total_new_representation_difference
    );
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

fn convert_to_new_representation(line: &str) -> String {
    let mut result = String::new();

    result.push('"');
    for c in line.chars() {
        match c {
            '"' => {
                result.push('\\');
                result.push('"');
            }
            '\\' => {
                result.push('\\');
                result.push('\\');
            }
            _ => {
                result.push(c);
            }
        };
    }
    result.push('"');

    result
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

    #[test]
    fn test_new_representation() {
        assert_eq!(
            convert_to_new_representation(r#""""#),
            String::from(r#""\"\"""#)
        ); // an increase from 2 characters to 6.
        assert_eq!(
            convert_to_new_representation(r#""abc""#),
            String::from(r#""\"abc\"""#)
        ); // an increase from 5 characters to 9.
        assert_eq!(
            convert_to_new_representation(r#""aaa\"aaa""#),
            String::from(r#""\"aaa\\\"aaa\"""#)
        ); // an increase from 10 characters to 16.
        assert_eq!(
            convert_to_new_representation(r#""\x27""#),
            String::from(r#""\"\\x27\"""#)
        ); // an increase from 6 characters to 11.
    }
}
