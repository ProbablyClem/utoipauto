mod controllers;

use utoipa::OpenApi;
use utoipa_auto_discovery::utoipa_auto_discovery;

// Discover from multiple controllers
#[utoipa_auto_discovery(
    paths = "( crate::controllers::controller1 => ./tests/controllers/controller1.rs) ; ( crate::controllers::controller2 => ./tests/controllers/controller2.rs )"
)]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct MultiControllerApiDocs {}

#[test]
fn test_path_import() {
    assert_eq!(MultiControllerApiDocs::openapi().paths.paths.len(), 2)
}

/// Discover from a single controller
#[utoipa_auto_discovery(
    paths = "( crate::controllers::controller1 => ./tests/controllers/controller1.rs)"
)]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct SingleControllerApiDocs {}

#[test]
fn test_ignored_path() {
    assert_eq!(SingleControllerApiDocs::openapi().paths.paths.len(), 1)
}
