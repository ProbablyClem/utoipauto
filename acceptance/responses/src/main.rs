pub mod response;
mod routes;

use utoipa::OpenApi;
use utoipauto::utoipauto;

#[utoipauto(paths = "./responses/src")]
#[derive(Debug, OpenApi)]
#[openapi(info(title = "Responses Test Api"))]
pub(crate) struct ApiDoc;

fn main() {
    println!(
        "Our OpenApi documentation {}",
        ApiDoc::openapi().to_pretty_json().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use crate::ApiDoc;
    use serde_json::Value;
    use utoipa::OpenApi;

    pub(crate) const EXPECTED_OPEN_API: &str = include_str!("open_api.expected.json");

    fn assert_json_eq(actual: &str, expected: &str) {
        let actual_value: Value = serde_json::from_str(actual).expect("Invalid JSON in actual");
        let expected_value: Value = serde_json::from_str(expected).expect("Invalid JSON in expected");

        println!("Actual: {}", actual);

        assert_eq!(actual_value, expected_value, "JSON objects are not equal");
    }

    #[test]
    fn test_open_api() {
        let open_api = ApiDoc::openapi().to_json().unwrap();
        let expected_value = EXPECTED_OPEN_API;

        assert_json_eq(&open_api, expected_value);
    }
}
