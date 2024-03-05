use utoipa::OpenApi;
use utoipauto_macro::utoipauto;

mod schemas;
mod more_schemas;

#[utoipauto(paths = "./tests/generic_full_path")]
#[derive(OpenApi)]
#[openapi(info(title = "Percentage API", version = "1.0.0"))]
pub struct CrateApiDocs;