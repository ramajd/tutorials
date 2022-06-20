use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("input.json").expect("Failed to open input.json");
    let buffer = BufReader::new(file);

    let data: serde_json::Value =
        serde_json::from_reader(buffer).expect("Failed to read JSON file");

    let sum = calculate_sum(&data, &Vec::new());
    println!("Sum of all numbers = {}", sum);

    let ignore_list = vec!["red"];
    let sum = calculate_sum(&data, &ignore_list);
    println!("Sum of the numbers ignoring {:?} = {}", ignore_list, &sum);
}

fn calculate_sum(data: &serde_json::Value, ignore_words: &Vec<&str>) -> i64 {
    let mut result = 0;
    if data.is_array() {
        for v in data.as_array().unwrap() {
            result += calculate_sum(v, ignore_words);
        }
    } else if data.is_object() {
        let mut obj_value = 0;
        for (_, v) in data.as_object().unwrap() {
            if let Some(s) = v.as_str() {
                if ignore_words.contains(&s) {
                    return 0;
                }
            }
            obj_value += calculate_sum(v, ignore_words);
        }
        result += obj_value;
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
        assert!(calculate_sum(&serde_json::json!([1, 2, 3]), &Vec::new()) == 6);
        assert!(calculate_sum(&serde_json::json!({"a":2,"b":4}), &Vec::new()) == 6);
        assert!(calculate_sum(&serde_json::json!([[[3]]]), &Vec::new()) == 3);
        assert!(calculate_sum(&serde_json::json!({"a":{"b":4},"c":-1}), &Vec::new()) == 3);
        assert!(calculate_sum(&serde_json::json!({"a":[-1,1]}), &Vec::new()) == 0);
        assert!(calculate_sum(&serde_json::json!([-1,{"a":1}]), &Vec::new()) == 0);
        assert!(calculate_sum(&serde_json::json!([]), &Vec::new()) == 0);
        assert!(calculate_sum(&serde_json::json!({}), &Vec::new()) == 0);
    }

    #[test]
    fn test_calculate_ignoring_red() {
        let ignore_list = vec!["red"];
        assert!(calculate_sum(&serde_json::json!([1, 2, 3]), &ignore_list) == 6);
        assert!(calculate_sum(&serde_json::json!([1,{"c":"red","b":2},3]), &ignore_list) == 4);
        assert!(
            calculate_sum(
                &serde_json::json!({"d":"red","e":[1,2,3,4],"f":5}),
                &ignore_list
            ) == 0
        );
        assert!(calculate_sum(&serde_json::json!([1, "red", 5]), &ignore_list) == 6);
    }
}
