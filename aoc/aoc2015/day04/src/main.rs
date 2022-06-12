use md5;

fn calculate_hash(secret: &str, number: usize) -> String {
    let hash = md5::compute(format!("{}{}", secret, number).as_bytes());
    let mut hash_str = String::new();
    for byte in hash.as_slice() {
        hash_str.push_str(&format!("{:02x}", byte));
    }
    hash_str
}

fn mine_block(secret: &str, predict: &str) -> (usize, String) {
    let mut number = 0;
    let mut hash = calculate_hash(secret, number);
    while !hash.starts_with(predict) {
        number += 1;
        hash = calculate_hash(secret, number);
    }
    (number, hash)
}

fn main() {
    let secret = "iwrupvqb";
    let result = mine_block(secret, "00000");
    println!("{} - {}", result.0, result.1);
}
