fn main() {
    // password requirements:
    // 1. exactly 8 lowercase letters
    // 2. increment until it's valid
    // 3. straight of 1 increasing of 3 letters like 'acb' or 'xyz'
    // 4. should not include 'i', 'o', 'l'
    // 5. at least 2 overlapping pairs, like 'aa' or 'bb'

    let input = "vzbxkghb";
    let next = find_next(input);
    println!("Next password = {}", next);
    let next = find_next(&next);
    println!("Next password = {}", next);
}

fn find_next(input: &str) -> String {
    let mut next = increment(input);
    while !qualified(&next) {
        next = increment(&next);
    }
    next
}

fn increment(input: &str) -> String {
    let next_char = |c: char, incr: bool| match (c, incr) {
        ('z', true) => ('a', true),
        (c, true) => (((c as u8) + 1) as char, false),
        _ => (c, false),
    };

    let mut result = String::new();

    if input.len() == 0 {
        return result;
    }

    let mut incr = true;
    for c in input.chars().rev() {
        let (next, overflow) = next_char(c, incr);
        incr = overflow;
        result.insert(0, next);
    }
    result
}

fn qualified(input: &str) -> bool {
    let len_matched = input.len() == 8;
    let mut straight_found = false;
    let mut has_letters = false;
    let mut pair_count = 0;

    let mut p = 0;
    let mut pp = 0;
    for c in input.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            has_letters = true;
        }
        let c = c as u8;
        if (c == p + 1) && (c == pp + 2) {
            straight_found = true;
        }
        if (p == c) && (pp != c) {
            pair_count += 1;
        }
        pp = p;
        p = c;
    }

    len_matched && straight_found && !has_letters && (pair_count >= 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        assert_eq!(increment("abc"), "abd");
        assert_eq!(increment("xyz"), "xza");
        assert_eq!(increment("zzz"), "aaa");
        assert_eq!(increment(""), "");
    }

    #[test]
    fn test_qualified() {
        assert_eq!(qualified("abc"), false); // length < 8
        assert_eq!(qualified("hijklmmn"), false); // no i, o, l
        assert_eq!(qualified("abbceffg"), false); // no straight
        assert_eq!(qualified("abbcdgjk"), false); // need 2 overlapping pairs
        assert_eq!(qualified("abcdffaa"), true);
        assert_eq!(qualified("ghjaabcc"), true);
        assert_eq!(qualified("abcdffaa"), true);
        assert_eq!(qualified("ghjaabcc"), true);
    }

    #[test]
    fn test_find_next() {
        assert_eq!(find_next("abcdefgh"), String::from("abcdffaa"));
        assert_eq!(find_next("ghijklmn"), String::from("ghjaabcc"));
    }
}
