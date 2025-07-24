use std::fs;
use serde_json::Value;

pub fn read_json_file(path: &str) -> Value {
    let input_data = fs::read_to_string(path).expect("Failed to read input file");
    serde_json::from_str(&input_data).expect("Invalid JSON")
}

pub fn write_json_file(path: &str, value: &Value) {
    let output = serde_json::to_string_pretty(value).unwrap();
    fs::write(path, output).expect("Failed to write output");
}
