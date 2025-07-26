# 🧹 json_cleaner

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**json_cleaner** is a simple and efficient CLI tool written in Rust that removes `null`, empty strings, empty arrays, and empty objects from JSON files.

Built for developers who need clean data to work with, without writing custom logic every time.

---

## 🚀 Features

- ✅ Remove `null`, `""`, `{}`, and `[]` recursively  
- 📂 Input/output from file paths  
- 🎯 Pretty-printed output  
- ⚡ Fast and minimal dependencies  
- 🦀 Written entirely in Rust  

---

## 📦 Installation

```bash
git clone https://github.com/MaloWinrhy/json_cleaner.git
cd json_cleaner
cargo build --release
```

You’ll find the binary in `target/release/json_cleaner`.

---

## 🧪 Example

### Input (`example.json`)

```json
{
  "name": "",
  "age": null,
  "bio": "",
  "address": {
    "city": "Paris",
    "zip": null,
    "extra": {}
  },
  "tags": [],
  "projects": ["EcoWeave"]
}
```

### Output

```json
{
  "address": {
    "city": "Paris"
  },
  "projects": [
    "EcoWeave"
  ]
}
```

---

## 🛠️ Usage

```bash
cargo run -- --input path/to/input.json
```

With output:

```bash
cargo run -- --input input.json --output cleaned.json
```

Or apply changes in place:

```bash
cargo run -- --input input.json --in-place
```

Or with compiled binary:

```bash
./json_cleaner --input input.json --output cleaned.json
```

---

## 📚 Options

```bash
json_cleaner --help
```

```
USAGE:
    json_cleaner [OPTIONS] --input <INPUT>

OPTIONS:
    -i, --input <INPUT>        Path to input file
    -o, --output <OUTPUT>      Path to output file (optional)
    --in-place                 Overwrite input file directly
    -h, --help                 Print help
    -V, --version              Print version
```

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).
