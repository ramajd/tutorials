fn main() {
    let input = "wtnhxymk";

    let password = first_door_password(input);
    println!("The password for 1st door = {}", password);

    let second_password = second_door_password(input);
    println!("The password for 2nd door = {}", second_password);
}

fn first_door_password(door_id: &str) -> String {
    let mut password: [u8; 8] = [b'-'; 8];
    println!(
        "password = '{}'",
        password.iter().map(|b| *b as char).collect::<String>()
    );

    let mut offset = 0;

    for i in 0..8 {
        let (hash, new_offset) = find_hash(door_id, offset);
        offset = new_offset + 1;
        let char = hash.as_bytes()[5];
        password[i] = char;
        println!(
            "password = '{}'",
            password.iter().map(|b| *b as char).collect::<String>()
        );
    }
    password.iter().map(|b| *b as char).collect()
}

fn second_door_password(door_id: &str) -> String {
    let mut password: [u8; 8] = [b'-'; 8];
    println!(
        "password = '{}'",
        password.iter().map(|b| *b as char).collect::<String>()
    );
    let mut offset = 0;
    for _ in 0..8 {
        loop {
            let (hash, new_offset) = find_hash(door_id, offset);
            offset = new_offset + 1;
            let position = (hash.as_bytes()[5] as char).to_digit(16).unwrap() as usize;
            let char = hash.as_bytes()[6];
            if position < 8 && password[position] == b'-' {
                password[position] = char;
                println!(
                    "password = '{}'",
                    password.iter().map(|b| *b as char).collect::<String>()
                );
                break;
            }
        }
    }

    password.iter().map(|b| *b as char).collect()
}

fn find_hash(door_id: &str, offset: usize) -> (String, usize) {
    let mut current = offset;

    loop {
        let str = format!("{}{}", door_id, current);
        let digest = md5::compute(str.as_bytes());
        let hash = format!("{:x}", digest);

        if hash.starts_with("00000") {
            return (hash, current);
        }
        current += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_hash() {
        let door_id = "abc";

        let (hash, offset) = find_hash(door_id, 0);
        assert!(hash.starts_with("000001"));

        let (hash, offset) = find_hash(door_id, offset + 1);
        assert!(hash.starts_with("000008"));

        let (hash, _offset) = find_hash(door_id, offset + 1);
        assert!(hash.starts_with("00000f"));
    }

    #[test]
    fn test_first_door_password() {
        let door_id = "abc";
        assert_eq!(first_door_password(door_id), String::from("18f47a30"));
    }

    #[test]
    fn test_second_door_password() {
        let door_id = "abc";
        assert_eq!(second_door_password(door_id), String::from("05ace8e3"));
    }
}
