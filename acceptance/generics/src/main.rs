pub mod routes;
pub mod schemas;

use routes::*;

use utoipa::OpenApi;
use utoipauto::utoipauto;

#[utoipauto(paths = "./generics/src")]
#[derive(Debug, OpenApi)]
#[openapi(info(title = "Generic Test Api"))]
pub(crate) struct ApiDoc;

fn main() {
    println!(
        "Our OpenApi documentation {}",
        ApiDoc::openapi().to_pretty_json().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    pub(crate) const EXPECTED_OPEN_API: &str = include_str!("open_api.expected.json");

    fn assert_json_eq(actual: &str, expected: &str) {
        let actual_value: Value = serde_json::from_str(actual).expect("Invalid JSON in actual");
        let expected_value: Value = serde_json::from_str(expected).expect("Invalid JSON in expected");

        assert_eq!(actual_value, expected_value, "JSON objects are not equal");
    }

    #[test]
    fn test_openapi() {
        let open_api = ApiDoc::openapi().to_json().unwrap();

        assert_json_eq(&open_api, EXPECTED_OPEN_API);
    }
}
