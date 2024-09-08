use utoipa::{OpenApi, ToSchema};
use utoipauto_macro::utoipauto;

#[derive(ToSchema)]
pub struct LifetimeStructSchema<'a> {
    foo: &'a str,
}

#[derive(ToSchema)]
pub struct DoubleLifetimeStructSchema<'a, 'b> {
    foo: &'a str,
    bar: &'b str,
}

#[derive(ToSchema)]
pub enum LifetimeEnumSchema<'a> {
    Foo(&'a str),
}

#[derive(ToSchema)]
pub enum DoubleLifetimeEnumSchema<'a, 'b> {
    Foo(&'a str, &'b str),
}

/// Discover schema with lifetime generics
#[utoipauto(paths = "./utoipauto/tests/default_features/lifetime_generics.rs")]
#[derive(OpenApi)]
#[openapi(info(title = "Lifetimes API", version = "1.0.0"))]
pub struct LifetimesApiDocs {}

#[test]
fn test_lifetimes() {
    assert_eq!(
        LifetimesApiDocs::openapi()
            .components
            .expect("no components")
            .schemas
            .len(),
        4
    )
}
