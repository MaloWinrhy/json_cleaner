# json_cleaner

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**json_cleaner** is a simple and efficient command-line tool written in Rust that removes `null`, empty strings (`""`), empty arrays (`[]`), and empty objects (`{}`) from JSON files.

This tool is built for developers who want to clean up noisy JSON data without writing custom scripts.

---

## Features

- Recursively removes:
  - `null` values
  - Empty strings
  - Empty arrays
  - Empty objects
- Reads JSON from a file
- Optionally writes output to a file or overwrites the input
- Outputs formatted (pretty-printed) JSON
- Lightweight and fast

---

## Installation

Install directly from crates.io:

```bash
cargo install json_cleaner
```

---

## Usage

Basic usage:

```bash
json_cleaner --input input.json
```

With output written to another file:

```bash
json_cleaner --input input.json --output cleaned.json
```

Overwrite the original file in place:

```bash
json_cleaner --input input.json --in-place
```

Display help:

```bash
json_cleaner --help
```

---

## Example

### Input (`example.json`):

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

### Output:

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

## License

This project is licensed under the [MIT License](LICENSE).
