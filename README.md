# 🧹 json_cleaner

A tiny CLI tool written in Rust to clean JSON files by removing empty or null values.

## ✨ Features

- Load any JSON file
- Recursively remove `null` or empty (`""`, `{}`, `[]`) values
- Output to a cleaned version or stdout
- Easy to use, cross-platform, fast

## 📦 Usage

```bash
cargo run -- path/to/input.json -o path/to/output.json
