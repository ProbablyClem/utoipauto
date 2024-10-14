#![allow(dead_code)]

use std::borrow::Cow;
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::{Response, ResponseBuilder, Type};
// This code is used in the tests
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema},
    PartialSchema, ToResponse, ToSchema,
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

impl PartialSchema for ModelSchemaImpl {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new().schema_type(SchemaType::Type(Type::String)).into()
    }
}

impl ToSchema for ModelSchemaImpl {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("ModelSchemaImpl")
    }
}

// Manual implementation of utoipa::ToSchema
pub struct ModelSchemaImplFullName;

impl PartialSchema for ModelSchemaImplFullName {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new().schema_type(SchemaType::Type(Type::String)).into()
    }
}

impl utoipa::ToSchema for ModelSchemaImplFullName {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("ModelSchemaImplFullName")
    }
}

// Manual implementation of ToSchema
pub struct ModelResponseImpl;

impl<'s> ToResponse<'s> for ModelResponseImpl {
    fn response() -> (&'s str, RefOr<Response>) {
        (
            "ModelResponseImpl",
            ResponseBuilder::new().description("A manual response").into(),
        )
    }
}

// Manual implementation of utoipa::ToResponse
pub struct ModelResponseImplFullName;

impl<'s> utoipa::ToResponse<'s> for ModelResponseImplFullName {
    fn response() -> (&'s str, RefOr<Response>) {
        (
            "ModelResponseImplFullName",
            ResponseBuilder::new().description("A manual response").into(),
        )
    }
}
