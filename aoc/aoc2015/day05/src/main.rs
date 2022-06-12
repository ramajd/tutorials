use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn vowel_count(word: &str) -> usize {
    let mut count = 0;
    for c in word.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            count += 1;
        }
    }
    count
}

fn has_double_letter(word: &str) -> bool {
    let mut previous_letter = ' ';
    for c in word.chars() {
        if c == previous_letter {
            return true;
        }
        previous_letter = c;
    }
    false
}

fn contains_forbidden_words(word: &str) -> bool {
    let forbidden_words = ["ab", "cd", "pq", "xy"];
    for w in forbidden_words {
        if word.contains(w) {
            return true;
        }
    }
    false
}

fn is_nice(word: &str) -> bool {
    let vowels = vowel_count(word);
    let has_double = has_double_letter(word);
    let is_forbidden = contains_forbidden_words(word);

    !is_forbidden && vowels >= 3 && has_double
}

fn find_repeated_pair(word: &str) -> Option<String> {
    let mut prv = ' ';
    for (idx, c) in word.chars().enumerate() {
        let pair = format!("{}{}", prv, c);
        let res = word.rfind(&pair);

        if let Some(pos) = res {
            if pos > idx {
                return Some(pair);
            }
        }
        prv = c;
    }
    None
}

fn find_repeating_pattern(word: &str) -> Option<String> {
    let mut pp = ' ';
    let mut p = ' ';
    for c in word.chars() {
        if pp == c {
            return Some(format!("{}{}{}", pp, p, c));
        }
        pp = p;
        p = c;
    }
    None
}

fn is_nice_2(word: &str) -> bool {
    find_repeated_pair(word).is_some() && find_repeating_pattern(word).is_some()
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut nice_count_1 = 0;
    let mut nice_count_2 = 0;

    for line in buffer.lines() {
        let line = line.expect("Failed to read line");
        if is_nice(&line) {
            nice_count_1 += 1;
        }
        if is_nice_2(&line) {
            nice_count_2 += 1;
        }
    }
    println!("nice count 1: {}", nice_count_1);
    println!("nice count 2: {}", nice_count_2);
}

#[cfg(test)]
mod tests {
    use crate::{find_repeated_pair, find_repeating_pattern};

    #[test]
    fn repeated_pair() {
        assert_eq!(find_repeated_pair("xyxy"), Some(String::from("xy")));
        assert_eq!(find_repeated_pair("aabcdefgaa"), Some(String::from("aa")));
        assert_eq!(find_repeated_pair("aaa"), None);
        assert_eq!(find_repeated_pair("baaa"), None);
        assert_eq!(find_repeated_pair("baaab"), None);
        assert_eq!(find_repeated_pair("abaaba"), Some(String::from("ab")));
    }

    #[test]
    fn pattern() {
        assert_eq!(find_repeating_pattern("aa"), None);
        assert_eq!(find_repeating_pattern("aaa"), Some(String::from("aaa")));
        assert_eq!(find_repeating_pattern("aba"), Some(String::from("aba")));
    }
}
