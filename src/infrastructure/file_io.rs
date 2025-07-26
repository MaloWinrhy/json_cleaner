use std::fs;
use serde_json::Value;

pub fn read_json_file(path: &str) -> Value {
    let input_data = fs::read_to_string(path).expect("Failed to read input file");
    serde_json::from_str(&input_data).expect("Invalid JSON")
}
