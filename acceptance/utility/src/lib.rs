use serde_json::Value;

pub fn assert_json_eq(actual: &str, expected: &str) {
    let actual_value: Value = serde_json::from_str(actual).expect("Invalid JSON in actual");
    let expected_value: Value = serde_json::from_str(expected).expect("Invalid JSON in expected");

    assert_eq!(actual_value, expected_value, "JSON objects are not equal");
}
