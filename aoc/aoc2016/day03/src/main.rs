use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Triangle(u32, u32, u32);

impl Triangle {
    fn from(s1: u32, s2: u32, s3: u32) -> Result<Self, &'static str> {
        if s1 + s2 <= s3 || s1 + s3 <= s2 || s2 + s3 <= s1 {
            return Err("invalid triangle");
        }
        Ok(Self(s1, s2, s3))
    }
}

impl FromStr for Triangle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2, s3) = parse_line(s)?;
        Self::from(s1, s2, s3)
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read input");

    println!("Number of Triangles (row) = {}", count_horizontal(&data));
    println!("Number of Triangles (column) = {}", count_vertical(&data));
}

fn count_horizontal(data: &str) -> u32 {
    let mut count = 0;
    for line in data.lines() {
        let (s1, s2, s3) = parse_line(line).expect("Failed to parse input");
        if let Ok(_) = Triangle::from(s1, s2, s3) {
            count += 1;
        }
    }
    count
}

fn count_vertical(data: &str) -> u32 {
    let mut count = 0;
    let mut lines = data.lines();
    while let Some(line) = lines.next() {
        let (s1, s2, s3) = parse_line(line).unwrap();
        let mut t1 = vec![s1];
        let mut t2 = vec![s2];
        let mut t3 = vec![s3];

        if let Some(line) = lines.next() {
            let (s1, s2, s3) = parse_line(line).unwrap();
            t1.push(s1);
            t2.push(s2);
            t3.push(s3);
        }
        if let Some(line) = lines.next() {
            let (s1, s2, s3) = parse_line(line).unwrap();
            t1.push(s1);
            t2.push(s2);
            t3.push(s3);
        }
        if t1.len() == 3 {
            if let Ok(_) = Triangle::from(t1[0], t1[1], t1[2]) {
                count += 1;
            }
            if let Ok(_) = Triangle::from(t2[0], t2[1], t2[2]) {
                count += 1;
            }
            if let Ok(_) = Triangle::from(t3[0], t3[1], t3[2]) {
                count += 1;
            }
        }
    }
    count
}

fn parse_line(line: &str) -> Result<(u32, u32, u32), &'static str> {
    let mut parts = line.trim().split_ascii_whitespace();
    let p1 = parts
        .next()
        .ok_or("invalid input")?
        .parse()
        .map_err(|_| "invalid input")?;
    let p2 = parts
        .next()
        .ok_or("invalid input")?
        .parse()
        .map_err(|_| "invalid input")?;
    let p3 = parts
        .next()
        .ok_or("invalid input")?
        .parse()
        .map_err(|_| "invalid input")?;
    Ok((p1, p2, p3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_parser() {
        assert_eq!("5 10 25".parse::<Triangle>(), Err("invalid triangle"));
        assert_eq!(" 3   4   5 ".parse::<Triangle>(), Ok(Triangle(3, 4, 5)));
        assert_eq!(
            "  696  438  832".parse::<Triangle>(),
            Ok(Triangle(696, 438, 832))
        );
    }

    #[test]
    fn test_col_parser() {
        let data = String::from(
            r#"101 301 501
        102 302 502
        103 303 503
        201 401 601
        202 402 602
        203 403 603"#,
        );
        assert_eq!(count_vertical(&data), 6);
    }
}
