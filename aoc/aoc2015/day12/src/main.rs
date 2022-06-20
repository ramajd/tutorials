use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("input.json").expect("Failed to open input.json");
    let buffer = BufReader::new(file);

    let data: serde_json::Value =
        serde_json::from_reader(buffer).expect("Failed to read JSON file");

    let sum = calculate_sum(&data);
    println!("Sum of all numbers = {}", sum);
}

fn calculate_sum(data: &serde_json::Value) -> i64 {
    let mut result = 0;
    if data.is_array() {
        for v in data.as_array().unwrap() {
            result += calculate_sum(v);
        }
    } else if data.is_object() {
        for (_, v) in data.as_object().unwrap() {
            result += calculate_sum(v);
        }
    } else if data.is_number() {
        result += data.as_i64().unwrap();
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        assert!(calculate_sum(&serde_json::json!([1, 2, 3])) == 6);
        assert!(calculate_sum(&serde_json::json!({"a":2,"b":4})) == 6);
        assert!(calculate_sum(&serde_json::json!([[[3]]])) == 3);
        assert!(calculate_sum(&serde_json::json!({"a":{"b":4},"c":-1})) == 3);
        assert!(calculate_sum(&serde_json::json!({"a":[-1,1]})) == 0);
        assert!(calculate_sum(&serde_json::json!([-1,{"a":1}])) == 0);
        assert!(calculate_sum(&serde_json::json!([])) == 0);
        assert!(calculate_sum(&serde_json::json!({})) == 0);
    }
}
