use utoipa::{OpenApi, ToSchema};
use utoipauto_macro::utoipauto;

#[derive(ToSchema)]
#[aliases(ConstGenericStructSchema0 = ConstGenericStructSchema<0>)]
pub struct ConstGenericStructSchema<const N: usize> {
    foo: [u16; N],
}

#[derive(ToSchema)]
#[aliases(DoubleConstGenericStructSchema0 = DoubleConstGenericStructSchema<0, 0>)]
pub struct DoubleConstGenericStructSchema<const N: usize, const N2: usize> {
    foo: [u16; N],
    bar: [u16; N2],
}

#[derive(ToSchema)]
#[aliases(ConstGenericEnumSchema0 = ConstGenericEnumSchema<0>)]
pub enum ConstGenericEnumSchema<const N: usize> {
    Foo([u16; N]),
}

#[derive(ToSchema)]
#[aliases(DoubleConstGenericEnumSchema0 = DoubleConstGenericEnumSchema<0, 0>)]
pub enum DoubleConstGenericEnumSchema<const N: usize, const N2: usize> {
    Foo([u16; N], [u16; N2]),
}

/// Discover schema with const generics
#[utoipauto(paths = "./utoipauto/tests/default_features/const_generics.rs")]
#[derive(OpenApi)]
#[openapi(info(title = "Const Generic API", version = "1.0.0"))]
pub struct ConstGenericApiDocs {}

#[test]
fn test_const_generics() {
    assert_eq!(
        ConstGenericApiDocs::openapi()
            .components
            .expect("no components")
            .schemas
            .len(),
        4
    )
}
