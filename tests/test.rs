mod controllers;
mod models;
use utoipa::OpenApi;
use utoipauto::utoipauto;
// Discover from multiple controllers
#[utoipauto(
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
#[utoipauto(paths = "( crate::controllers::controller1 => ./tests/controllers/controller1.rs)")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct SingleControllerApiDocs {}

#[test]
fn test_ignored_path() {
    assert_eq!(SingleControllerApiDocs::openapi().paths.paths.len(), 1)
}

/// Discover with manual path
#[utoipauto(paths = "./tests/controllers/controller1.rs")]
#[derive(OpenApi)]
#[openapi(
    info(title = "Percentage API", version = "1.0.0"),
    paths(controllers::controller2::route3)
)]
pub struct SingleControllerManualPathApiDocs {}

#[test]
fn test_manual_path() {
    assert_eq!(
        SingleControllerManualPathApiDocs::openapi()
            .paths
            .paths
            .len(),
        2
    )
}
/// Discover from a module root
#[utoipauto(paths = "( crate::controllers => ./tests/controllers)")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct ModuleApiDocs {}
#[test]
fn test_module_import_path() {
    assert_eq!(ModuleApiDocs::openapi().paths.paths.len(), 2)
}

/// Discover from the crate root
#[utoipauto(paths = "./tests")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct CrateApiDocs {}

#[test]
fn test_crate_import_path() {
    assert_eq!(CrateApiDocs::openapi().paths.paths.len(), 2)
}

// Discover from multiple controllers new syntax
#[utoipauto(paths = "./tests/controllers/controller1.rs, ./tests/controllers/controller2.rs")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct MultiControllerNoModuleApiDocs {}

#[test]
fn test_path_import_no_module() {
    assert_eq!(
        MultiControllerNoModuleApiDocs::openapi().paths.paths.len(),
        2
    )
}

// Discover from multiple controllers new syntax
#[utoipauto(paths = "./tests/models.rs")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct ModelsImportApiDocs {}

#[test]
fn test_path_import_schema() {
    assert_eq!(
        ModelsImportApiDocs::openapi()
            .components
            .expect("no components")
            .schemas
            .len(),
        1
    )
}

// Discover from multiple controllers new syntax
#[utoipauto(paths = "./tests/models.rs")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct ResponsesImportApiDocs {}

#[test]
fn test_path_import_responses() {
    assert_eq!(
        ResponsesImportApiDocs::openapi()
            .components
            .expect("no components")
            .responses
            .len(),
        1
    )
}
