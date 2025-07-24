use clap::Parser;
use serde_json::{Value, Map};
use std::fs;


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input JSON file path
    #[arg(short, long)]
    input: String,

    /// Output file path (optional)
    #[arg(short, long)]
    output: Option<String>,
}

pub fn clean_json(value: Value) -> Option<Value> {
    match value {
        Value::Null => None,
        Value::Bool(_) | Value::Number(_) | Value::String(_) => Some(value),
        Value::Array(arr) => {
            let cleaned: Vec<Value> = arr.into_iter()
                .filter_map(clean_json)
                .collect();
            if cleaned.is_empty() {
                None
            } else {
                Some(Value::Array(cleaned))
            }
        }
        Value::Object(map) => {
            let cleaned: Map<_, _> = map.into_iter()
                .filter_map(|(k, v)| clean_json(v).map(|val| (k, val)))
                .collect();
            if cleaned.is_empty() {
                None
            } else {
                Some(Value::Object(cleaned))
            }
        }
    }
}
fn main() {
    let args = Args::parse();
    let input_data = fs::read_to_string(&args.input)
        .expect("Failed to read input file");

    let json: Value = serde_json::from_str(&input_data)
        .expect("Invalid JSON");

    let cleaned = clean_json(json).unwrap_or(Value::Null);

    let output = serde_json::to_string_pretty(&cleaned).unwrap();

    if let Some(output_path) = args.output {
        fs::write(output_path, output).expect("Failed to write output");
    } else {
        println!("{}", output);
    }
}

