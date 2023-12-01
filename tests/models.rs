use utoipa::{OpenApi, ToResponse, ToSchema};
use utoipa_auto_macro::utoipa_auto_discovery;

#[derive(ToSchema)]
pub struct ModelSchema;
#[derive(ToResponse)]
pub struct ModelResponse;
