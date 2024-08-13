use utoipa::OpenApi;
use utoipauto::utoipauto;

#[utoipauto(paths = "( ./crate_segment_path/crate_folder/sub_folder/paths.rs from crate::sub_folder )")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
#[allow(dead_code)]
pub struct CrateInAnotherPath {}

#[test]
fn test_crate_in_another_path() {
    assert_eq!(CrateInAnotherPath::openapi().paths.paths.len(), 2)
}
