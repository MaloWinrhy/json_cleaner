
# 🧹 json_cleaner

**json_cleaner** is a simple and efficient CLI tool written in Rust that removes `null`, empty strings, empty arrays, and empty objects from JSON files.

Built for developers who need clean data to work with, without writing custom logic every time.

---

## 🚀 Features

-  Remove `null`, `""`, `{}`, and `[]` recursively
-  Input/output from file paths
-  Pretty-printed output
-  Fast and minimal dependencies
-  Written entirely in Rust

---

## 📦 Installation

Clone the repository and build it with Cargo:

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

Or specify output:

```bash
cargo run -- --input path/to/input.json --output cleaned.json
```

Alternatively, use the compiled binary:

```bash
./json_cleaner --input data.json --output cleaned.json
```

---

## 🧰 Options

```bash
json_cleaner --help
```

```
USAGE:
    json_cleaner [OPTIONS] <INPUT>

ARGS:
    <INPUT>    Path to the input JSON file

OPTIONS:
    -o, --output <OUTPUT>    Path to output file (optional)
    -h, --help               Print help information
    -V, --version            Print version information
```

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).
