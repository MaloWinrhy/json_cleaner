use serde_json::Value;
use crate::domain::json_cleaner::clean_json;

pub struct CleanJsonUsecase;

impl CleanJsonUsecase {
    pub fn execute(input: Value) -> Value {
        clean_json(input).unwrap_or(Value::Null)
    }
}
