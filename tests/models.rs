#![allow(dead_code)] // This code is used in the tests
use utoipa::{ToResponse, ToSchema};
use utoipa_auto_macro::utoipa_ignore;

#[derive(ToSchema)]
pub struct ModelSchema;
#[derive(ToResponse)]
pub struct ModelResponse;

#[utoipa_ignore]
#[derive(ToSchema)]
pub struct IgnoredModelSchema;
