use serde_json::{Value, Map};

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
