use utoipa::OpenApi;

use utoipauto_macro::utoipauto;

#[utoipauto(paths = "./utoipauto/tests/generic_full_path")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct CrateApiDocs;

/// Discover schema with const generics
#[utoipauto(paths = "./utoipauto/tests/generic_full_path/const_generics.rs")]
#[derive(OpenApi)]
#[openapi(info(title = "Const Generic API", version = "1.0.0"))]
pub struct ConstGenericApiDocs {}

#[test]
fn test_const_generics() {
    assert_eq!(
        ConstGenericApiDocs::openapi()
            .components
            .expect("no components")
            .schemas
            .len(),
        2
    )
}
