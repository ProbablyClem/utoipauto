use utoipa::OpenApi;

use utoipauto_macro::utoipauto;

#[utoipauto(paths = "./utoipauto/tests/generic_full_path")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct CrateApiDocs;
