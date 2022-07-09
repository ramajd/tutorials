use std::fs::File;
use std::io::prelude::*;

trait Keyboard {
    fn generate_next(&self, instruction: &str, current: u32) -> Result<u32, &'static str>;

    fn find_code(&self, input: &str) -> Result<String, &'static str> {
        let mut result = String::new();
        let mut current = 5;
        for line in input.lines() {
            current = self.generate_next(line.trim(), current)?;
            result += format!("{:X}", current).as_str();
        }
        Ok(result)
    }
}

#[derive(Debug)]
struct SimpleKeyboard;

impl Keyboard for SimpleKeyboard {
    fn generate_next(&self, instruction: &str, current: u32) -> Result<u32, &'static str> {
        let mut current = current;
        for ch in instruction.trim().chars() {
            current = match (ch, current) {
                ('L', 1 | 4 | 7) => current,
                ('L', _) => current - 1,
                ('R', 3 | 6 | 9) => current,
                ('R', _) => current + 1,
                ('U', 1 | 2 | 3) => current,
                ('U', _) => current - 3,
                ('D', 7 | 8 | 9) => current,
                ('D', _) => current + 3,
                (_, _) => return Err("Invalid character provided"),
            };
        }
        Ok(current)
    }
}

#[derive(Debug)]
struct ExtendedKeyboard;

impl Keyboard for ExtendedKeyboard {
    fn generate_next(&self, instruction: &str, current: u32) -> Result<u32, &'static str> {
        let mut current = current;
        for ch in instruction.trim().chars() {
            current = match (ch, current) {
                ('L', 0x1 | 0x2 | 0x5 | 0xA | 0xD) => current,
                ('L', _) => current - 1,
                ('R', 0x1 | 0x4 | 0x9 | 0xC | 0xD) => current,
                ('R', _) => current + 1,
                ('U', 0x1 | 0x2 | 0x4 | 0x5 | 0x9) => current,
                ('U', 0x3 | 0xD) => current - 2,
                ('U', _) => current - 4,
                ('D', 0x5 | 0x9 | 0xA | 0xC | 0xD) => current,
                ('D', 0x1 | 0xB) => current + 2,
                ('D', _) => current + 4,
                (_, _) => return Err("Invalid character provided"),
            };
        }
        Ok(current)
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.txt");

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read operations");

    let keyboard = SimpleKeyboard {};
    println!("bathroom code = {:?}", keyboard.find_code(&data));

    let keyboard = ExtendedKeyboard {};
    println!(
        "bathroom code with extended keyboard = {:?}",
        keyboard.find_code(&data)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_key() {
        let keyboard = SimpleKeyboard {};
        assert_eq!(keyboard.generate_next("ULL", 5), Ok(1));
        assert_eq!(keyboard.generate_next("RRDDD", 1), Ok(9));
        assert_eq!(keyboard.generate_next("LURDL", 9), Ok(8));
        assert_eq!(keyboard.generate_next("UUUUD", 8), Ok(5));
    }

    #[test]
    fn test_simple_keyboard() {
        let input = String::from(
            r#"ULL
        RRDDD
        LURDL
        UUUUD"#,
        );
        let keyboard = SimpleKeyboard {};
        assert_eq!(keyboard.find_code(&input), Ok(String::from("1985")));
    }

    #[test]
    fn test_extended() {
        let keyboard = ExtendedKeyboard {};
        assert_eq!(keyboard.generate_next("ULL", 5), Ok(5));
        assert_eq!(keyboard.generate_next("RRDDD", 5), Ok(0xd));
        assert_eq!(keyboard.generate_next("LURDL", 0xd), Ok(0xb));
        assert_eq!(keyboard.generate_next("UUUUD", 0xb), Ok(3));
    }

    #[test]
    fn test_extended_keyboard() {
        assert_eq!(format!("{:x}", 42), "2a");
        assert_eq!(format!("{:X}", 42), "2A");
        let input = String::from(
            r#"ULL
            RRDDD
            LURDL
            UUUUD"#,
        );
        let keyboard = ExtendedKeyboard {};
        assert_eq!(keyboard.find_code(&input), Ok(String::from("5DB3")));
    }
}
