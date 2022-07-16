use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct IPv7(String);

impl IPv7 {
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }

    fn hypernets<'a>(&'a self) -> Vec<&'a str> {
        let mut result = Vec::new();
        let mut part = self.0.as_str();

        while let Some(start) = part.find('[') {
            if let Some(end) = part.find(']') {
                result.push(part.get(start + 1..end).unwrap());
                part = part.get(end + 1..).unwrap();
            }
        }
        result
    }

    fn supernets<'a>(&'a self) -> Vec<&'a str> {
        let mut result = Vec::new();
        let mut part = self.0.as_str();
        while let Some(start) = part.find('[') {
            let item = part.get(..start).unwrap();
            if item.len() > 0 {
                result.push(part.get(..start).unwrap());
            }
            let end = part.find(']').unwrap();
            part = part.get(end + 1..).unwrap();
        }
        if part.len() > 0 {
            result.push(part);
        }
        result
    }

    fn find_abba(s: &str) -> Option<usize> {
        let mut p = ' ';
        let mut pp = ' ';
        let mut ppp = ' ';
        for (idx, c) in s.chars().enumerate() {
            if ppp != pp && ppp == c && pp == p {
                return Some(idx - 3);
            }
            ppp = pp;
            pp = p;
            p = c;
        }
        None
    }

    fn find_aba(s: &str) -> Option<usize> {
        let mut p = ' ';
        let mut pp = ' ';
        for (idx, c) in s.chars().enumerate() {
            if c != p && c == pp {
                return Some(idx - 2);
            }
            pp = p;
            p = c;
        }
        None
    }

    fn is_tls(&self) -> bool {
        let hypernets = self.hypernets();
        for h in hypernets {
            if let Some(_) = IPv7::find_abba(h) {
                return false;
            }
        }
        let supernets = self.supernets();
        for s in supernets {
            if let Some(_) = IPv7::find_abba(s) {
                return true;
            }
        }
        false
    }

    fn is_ssl(&self) -> bool {
        let supernets = self.supernets();
        for supernet in supernets {
            let mut part = supernet;
            while let Some(idx) = IPv7::find_aba(part) {
                let aba = part.get(idx..idx + 3).unwrap();
                if self.has_marching_bab(aba) {
                    return true;
                }
                part = part.get(idx + 1..).unwrap();
            }
        }
        false
    }

    fn has_marching_bab(&self, aba: &str) -> bool {
        if aba.len() > 2 {
            let a = aba.get(0..1).unwrap();
            let b = aba.get(1..2).unwrap();
            let bab = format!("{}{}{}", b, a, b);
            for h in self.hypernets() {
                if h.contains(&bab) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buffer = BufReader::new(file);

    let mut tls_count = 0;
    let mut ssl_count = 0;

    for line in buffer.lines() {
        let ip = IPv7::new(&line.unwrap());
        if ip.is_tls() {
            tls_count += 1;
        }
        if ip.is_ssl() {
            ssl_count += 1;
        }
    }

    println!("tls count = {}", tls_count);
    println!("ssl count = {}", ssl_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypernets() {
        assert_eq!(IPv7::new("a[b]c").hypernets(), vec!["b"]);
        assert_eq!(IPv7::new("[b]c").hypernets(), vec!["b"]);
        assert_eq!(IPv7::new("b[c]").hypernets(), vec!["c"]);
        assert_eq!(IPv7::new("[a]b[c]").hypernets(), vec!["a", "c"]);
        assert_eq!(IPv7::new("[a][c]").hypernets(), vec!["a", "c"]);
    }

    #[test]
    fn test_supernets() {
        assert_eq!(IPv7::new("a[b]c").supernets(), vec!["a", "c"]);
        assert_eq!(IPv7::new("[b]c").supernets(), vec!["c"]);
        assert_eq!(IPv7::new("b[c]").supernets(), vec!["b"]);
        assert_eq!(IPv7::new("[a]b[c]").supernets(), vec!["b"]);
        assert_eq!(IPv7::new("a").supernets(), vec!["a"]);
        assert_eq!(IPv7::new("[a]").supernets(), Vec::<&str>::new());
    }

    #[test]
    fn test_tls() {
        assert!(IPv7::new("abba[mnop]qrst").is_tls()); // supports TLS (abba outside square brackets).
        assert!(!IPv7::new("abcd[bddb]xyyx").is_tls()); // does not support TLS (bddb is within square brackets, even though xyyx is outside square brackets).
        assert!(!IPv7::new("aaaa[qwer]tyui").is_tls()); // does not support TLS (aaaa is invalid; the interior characters must be different).
        assert!(IPv7::new("ioxxoj[asdfgh]zxcvbn").is_tls()); // supports TLS (oxxo is outside square brackets, even though it's within a larger string).
    }

    #[test]
    fn test_ssl() {
        assert!(IPv7::new("aba[bab]xyz").is_ssl()); // supports SSL (aba outside square brackets with corresponding bab within square brackets).
        assert!(!IPv7::new("xyx[xyx]xyx").is_ssl()); // does not support SSL (xyx, but no corresponding yxy).
        assert!(IPv7::new("aaa[kek]eke").is_ssl()); // supports SSL (eke in supernet with corresponding kek in hypernet; the aaa sequence is not related, because the interior character must be different).
        assert!(IPv7::new("zazbz[bzb]cdb").is_ssl()); // supports SSL (zaz has no corresponding aza, but zbz has a corresponding bzb, even though zaz and zbz overlap).
    }
}
