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
    use utoipa::OpenApi;
    use utility::assert_json_eq;

    pub(crate) const EXPECTED_OPEN_API: &str = include_str!("open_api.expected.json");
    #[test]
    fn test_open_api() {
        let open_api = ApiDoc::openapi().to_json().unwrap();
        let expected_value = EXPECTED_OPEN_API;

        assert_json_eq(&open_api, expected_value);
    }
}
