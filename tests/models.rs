#![allow(dead_code)] // This code is used in the tests
use utoipa::{ToResponse, ToSchema};

#[derive(ToSchema)]
pub struct ModelSchema;
#[derive(ToResponse)]
pub struct ModelResponse;
