use std::fs;
use assert_cmd::Command;

#[test]
fn test_clean_json_output_to_stdout() {
    let input = r#"{ "a": null, "b": 1, "c": [null, 2], "d": { "e": null, "f": "ok" } }"#;
    let input_path = "test_input.json";
    fs::write(input_path, input).unwrap();

    let mut cmd = Command::cargo_bin("json_cleaner").unwrap();
    cmd.arg("--input").arg(input_path);
    let output = cmd.assert().success().get_output().stdout.clone();
    let output_str = String::from_utf8_lossy(&output);
    assert!(output_str.contains("b"));
    assert!(output_str.contains("f"));
    assert!(!output_str.contains("null"));
    fs::remove_file(input_path).unwrap();
}

#[test]
fn test_clean_json_in_place() {
    let input = r#"{ "a": null, "b": 1 }"#;
    let input_path = "test_in_place.json";
    fs::write(input_path, input).unwrap();

    let mut cmd = Command::cargo_bin("json_cleaner").unwrap();
    cmd.arg("--input").arg(input_path).arg("--in-place");
    cmd.assert().success();
    let result = fs::read_to_string(input_path).unwrap();
    assert!(result.contains("b"));
    assert!(!result.contains("null"));
    fs::remove_file(input_path).unwrap();
}

#[test]
fn test_clean_json_output_to_file() {
    let input = r#"{ "a": null, "b": 1 }"#;
    let input_path = "test_input2.json";
    let output_path = "test_output.json";
    fs::write(input_path, input).unwrap();

    let mut cmd = Command::cargo_bin("json_cleaner").unwrap();
    cmd.arg("--input").arg(input_path).arg("--output").arg(output_path);
    cmd.assert().success();
    let result = fs::read_to_string(output_path).unwrap();
    assert!(result.contains("b"));
    assert!(!result.contains("null"));
    fs::remove_file(input_path).unwrap();
    fs::remove_file(output_path).unwrap();
}
