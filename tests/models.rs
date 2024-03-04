#![allow(dead_code)] // This code is used in the tests
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Response, ResponseBuilder, Schema, SchemaType},
    ToResponse, ToSchema,
};
use utoipauto_macro::utoipa_ignore;

#[derive(ToSchema)]
pub struct ModelSchema;
#[derive(ToResponse)]
pub struct ModelResponse;

#[utoipa_ignore]
#[derive(ToSchema)]
pub struct IgnoredModelSchema;

// Manual implementation of ToSchema
pub struct ModelSchemaImpl;

impl<'s> ToSchema<'s> for ModelSchemaImpl {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "ModelSchemaImpl",
            ObjectBuilder::new().schema_type(SchemaType::String).into(),
        )
    }
}

// Manual implementation of utoipa::ToSchema
pub struct ModelSchemaImplFullName;

impl<'s> utoipa::ToSchema<'s> for ModelSchemaImplFullName {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "ModelSchemaImplFullName",
            ObjectBuilder::new().schema_type(SchemaType::String).into(),
        )
    }
}

// Manual implementation of ToSchema
pub struct ModelResponseImpl;

impl<'s> ToResponse<'s> for ModelResponseImpl {
    fn response() -> (&'s str, RefOr<Response>) {
        (
            "ModelResponseImpl",
            ResponseBuilder::new()
                .description("A manual response")
                .into(),
        )
    }
}

// Manual implementation of utoipa::ToResponse
pub struct ModelResponseImplFullName;

impl<'s> utoipa::ToResponse<'s> for ModelResponseImplFullName {
    fn response() -> (&'s str, RefOr<Response>) {
        (
            "ModelResponseImplFullName",
            ResponseBuilder::new()
                .description("A manual response")
                .into(),
        )
    }
}
