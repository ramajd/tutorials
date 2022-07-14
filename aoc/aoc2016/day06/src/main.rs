use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read data");

    let message = recover_most_occurrence(&data);
    println!("recovered with most occurrences = {}", message);
    let message = recover_least_occurrence(&data);
    println!("recovered with least occurrences = {}", message);
}

fn count_occurrences(data: &str) -> Vec<HashMap<char, usize>> {
    let mut counts = Vec::new();
    for line in data.trim().lines().map(|l| l.trim()) {
        for (idx, ch) in line.chars().enumerate() {
            if counts.len() <= idx {
                counts.push(HashMap::new());
            }
            *counts[idx].entry(ch).or_insert(0) += 1;
        }
    }
    counts
}
fn recover_most_occurrence(data: &str) -> String {
    let counts = count_occurrences(data);
    let mut message = String::new();
    for p in counts {
        let ch = p.iter().fold(
            (' ', 0),
            |(rc, rv), (vc, vv)| if rv < *vv { (*vc, *vv) } else { (rc, rv) },
        );
        message += format!("{}", ch.0).as_str();
    }
    message
}

fn recover_least_occurrence(data: &str) -> String {
    let counts = count_occurrences(data);
    let mut message = String::new();
    for p in counts {
        let ch = p.iter().fold((' ', usize::MAX), |(rc, rv), (vc, vv)| {
            if rv > *vv {
                (*vc, *vv)
            } else {
                (rc, rv)
            }
        });
        message += format!("{}", ch.0).as_str();
    }
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_recovery() {
        let data = r#"eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar"#;

        assert_eq!(recover_most_occurrence(data), "easter".to_owned());
        assert_eq!(recover_least_occurrence(data), "advent".to_owned());
    }
}
