#![allow(dead_code)] // This code is used in the tests
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
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

// // Manual implementation of ToSchema
// pub struct ModelSchemaImpl;

// impl<'s> utoipa::ToSchema<'s> for ModelSchemaImpl {
//     fn schema() -> (&'s str, RefOr<Schema>) {
//         (
//             "string",
//             ObjectBuilder::new().schema_type(SchemaType::String).into(),
//         )
//     }
// }
