use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Room {
    id: u32,
    checksum: String,
    encrypted: String,
}

impl FromStr for Room {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx_checksum = s.rfind("[").ok_or("invalid input")?;
        let checksum = s[idx_checksum + 1..s.len() - 1].to_owned();

        let idx_id = s.rfind("-").ok_or("invalid input")?;
        let id = s[idx_id + 1..idx_checksum]
            .parse()
            .map_err(|_| "invalid input")?;

        let encrypted = s[..idx_id].to_owned();

        Ok(Self {
            id,
            checksum,
            encrypted,
        })
    }
}

impl Room {
    fn is_valid(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    fn calculate_checksum(&self) -> String {
        let mut repetitions = HashMap::new();

        for c in self.encrypted.chars().filter(|c| *c != '-') {
            *repetitions.entry(c).or_insert(0) += 1;
        }

        let mut keys: Vec<char> = repetitions.keys().map(|c| *c).collect();
        keys.sort_by(|a, b| {
            let va = repetitions.get(a).unwrap();
            let vb = repetitions.get(b).unwrap();
            if va == vb {
                a.cmp(b)
            } else {
                vb.cmp(va)
            }
        });
        keys[..5].iter().collect::<String>()
    }

    fn decrypt(&self) -> String {
        let offset = (self.id % 26) as u8;
        self.encrypted
            .bytes()
            .map(|c| match c {
                b'a'..=b'z' => ((c - b'a' + offset) % 26 + b'a') as char,
                b'-' => ' ',
                _ => panic!("invalid encryption"),
            })
            .collect()
    }
}

fn main() {
    let file = File::open("input.txt").expect("failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut rooms: Vec<Room> = Vec::new();
    let mut sector_sum = 0;
    for line in buffer.lines() {
        let line = line.expect("Failed to read lint");
        if let Ok(room) = line.parse::<Room>() {
            if room.is_valid() {
                sector_sum += room.id;
                rooms.push(room);
            }
        }
    }
    println!("Sum of sectorID's = {}", sector_sum);

    let northpole_room = rooms
        .iter()
        .find(|room| room.decrypt().contains("northpole"))
        .unwrap();
    println!(
        "SectorID of 'northpole object storage': {}",
        northpole_room.id
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            "aaaaa-bbb-z-y-x-123[abxyz]".parse(),
            Ok(Room {
                id: 123,
                checksum: "abxyz".to_owned(),
                encrypted: "aaaaa-bbb-z-y-x".to_owned()
            })
        );
        assert_eq!(
            "a-b-c-d-e-f-g-h-987[abcde]".parse(),
            Ok(Room {
                id: 987,
                checksum: "abcde".to_owned(),
                encrypted: "a-b-c-d-e-f-g-h".to_owned(),
            })
        );
        assert_eq!(
            "not-a-real-room-404[oarel]".parse(),
            Ok(Room {
                id: 404,
                checksum: "oarel".to_owned(),
                encrypted: "not-a-real-room".to_string(),
            })
        );
        assert_eq!(
            "totally-real-room-200[decoy]".parse::<Room>(),
            Ok(Room {
                id: 200,
                checksum: "decoy".to_owned(),
                encrypted: "totally-real-room".to_owned()
            })
        );
    }

    #[test]
    fn test_validation() {
        let room: Room = "aaaaa-bbb-z-y-x-123[abxyz]".parse().unwrap();
        assert!(room.is_valid());

        let room: Room = "a-b-c-d-e-f-g-h-987[abcde]".parse().unwrap();
        assert!(room.is_valid());

        let room: Room = "not-a-real-room-404[oarel]".parse().unwrap();
        assert!(room.is_valid());

        let room: Room = "totally-real-room-200[decoy]".parse().unwrap();
        assert!(!room.is_valid());
    }

    #[test]
    fn test_decryption() {
        let room = Room {
            id: 343,
            encrypted: "qzmt-zixmtkozy-ivhz".to_owned(),
            checksum: "".to_owned(),
        };
        assert_eq!(room.decrypt(), "very encrypted name");
    }
}
