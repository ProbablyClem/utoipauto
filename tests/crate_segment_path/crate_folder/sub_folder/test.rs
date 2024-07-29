use utoipa::OpenApi;
use utoipauto_macro::utoipauto;

#[utoipauto(paths = "( ./crate_folder/sub_folder/paths.rs from crate::sub_folder )")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct CrateInAnotherPath {}

#[test]
fn test_crate_in_another_path() {
    assert_eq!(CrateInAnotherPath::openapi().paths.paths.len(), 2)
}
