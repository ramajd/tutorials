use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let extracted = extract(&data);
    println!("Extracted length = {}", extracted.len());

    let extracted = extended_count(&data);
    println!("Extended extract length = {}", extracted);
}

fn extract(data: &str) -> String {
    let mut result = String::new();
    let mut input = data.clone().trim();

    while let Some(start) = input.find('(') {
        result += input.get(..start).unwrap();
        if let Some(end) = input.find(')') {
            let marker = input.get(start + 1..end).unwrap();
            let mut parts = marker.split('x');
            let l: usize = parts.next().unwrap().parse().unwrap();
            let c: usize = parts.next().unwrap().parse().unwrap();
            for _ in 0..c {
                result += input.get(end + 1..end + 1 + l).unwrap();
            }

            input = input.get(end + 1 + l..).unwrap();
        }
    }

    if input.len() > 0 {
        result += input;
    }

    result
}

fn extended_count(data: &str) -> usize {
    let mut input = data.trim().to_owned();
    let mut result = 0;
    while let Some(start) = input.find('(') {
        let end = input.find(')').unwrap();
        let marker = input.get(start + 1..end).unwrap();
        let mut parts = marker.split('x');
        let length: usize = parts.next().unwrap().parse().unwrap();
        let count: usize = parts.next().unwrap().parse().unwrap();
        let marker_section = input.get(end + 1..end + 1 + length).unwrap();
        result += start + count * extended_count(marker_section);
        input = input.get(end + 1 + length..).unwrap().to_owned();
    }
    result += input.len();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        assert_eq!(extract("ADVENT"), "ADVENT".to_string());
        assert_eq!(extract("A(1x5)BC"), "ABBBBBC".to_string());
        assert_eq!(extract("(3x3)XYZ"), "XYZXYZXYZ".to_string());
        assert_eq!(extract("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG".to_string());
        assert_eq!(extract("(6x1)(1x3)A"), "(1x3)A".to_string());
        assert_eq!(extract("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY".to_string());
    }

    #[test]
    fn test_extract_extended() {
        assert_eq!(extended_count("(3x3)XYZ"), 9);
        assert_eq!(extended_count("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(extended_count("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(
            extended_count("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
