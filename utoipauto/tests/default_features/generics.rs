use utoipa::OpenApi;
use utoipauto_macro::utoipauto;

/// Combined Struct - type & const
#[derive(utoipa::ToSchema)]
#[aliases(
    TypeGenericAndConstGenericStructSchemaU32 = TypeGenericAndConstGenericStructSchema<core::primitive::u32, 1>,
    TypeGenericAndConstGenericStructSchemaU64 = TypeGenericAndConstGenericStructSchema<core::primitive::u64, 2>,
)]
pub struct TypeGenericAndConstGenericStructSchema<T, const N: usize> {
    foo: T,
    bar: [u16; N],
}

/// Combined Struct - lifetime & type
#[derive(utoipa::ToSchema)]
#[aliases(
    LifetimeAndTypeGenericGenericStructSchemaU32 = LifetimeAndTypeGenericGenericStructSchema<'a, core::primitive::u32>,
    LifetimeAndTypeGenericGenericStructSchemaU64 = LifetimeAndTypeGenericGenericStructSchema<'a, core::primitive::u64>,
)]
pub struct LifetimeAndTypeGenericGenericStructSchema<'a, T> {
    foo: &'a str,
    bar: T,
}

/// Combined Struct - lifetime & const
#[derive(utoipa::ToSchema)]
#[aliases(LifetimeAndConstGenericGenericStructSchema2 = LifetimeAndConstGenericGenericStructSchema<'a, 2>)]
pub struct LifetimeAndConstGenericGenericStructSchema<'a, const N: usize> {
    foo: &'a str,
    bar: [u16; N],
}

/// Combined Struct - lifetime & const & type
#[derive(utoipa::ToSchema)]
#[aliases(
    LifetimeAndConstAndTypeGenericGenericStructSchema1 = LifetimeAndConstAndTypeGenericGenericStructSchema<'a, 5, 1, core::primitive::u32>,
    LifetimeAndConstAndTypeGenericGenericStructSchema2 = LifetimeAndConstAndTypeGenericGenericStructSchema<'a, 6, 2, core::primitive::u64>,
)]
pub struct LifetimeAndConstAndTypeGenericGenericStructSchema<'a, const N: usize, const N2: usize, T> {
    a: &'a str,
    foo: [u16; N],
    bar: [u16; N2],
    t: T,
}

/// Combined Enum - type & const
#[derive(utoipa::ToSchema)]
#[aliases(
    TypeGenericAndConstGenericEnumSchemaU32 = TypeGenericAndConstGenericEnumSchema<core::primitive::u32, 1>,
    TypeGenericAndConstGenericEnumSchemaU64 = TypeGenericAndConstGenericEnumSchema<core::primitive::u64, 2>,
)]
pub enum TypeGenericAndConstGenericEnumSchema<T, const N: usize> {
    Foo(T, [u16; N]),
}

/// Combined Enum - lifetime & type
#[derive(utoipa::ToSchema)]
#[aliases(
    LifetimeAndTypeGenericGenericEnumSchemaU32 = LifetimeAndTypeGenericGenericEnumSchema<'a, core::primitive::u32>,
    LifetimeAndTypeGenericGenericEnumSchemaU64 = LifetimeAndTypeGenericGenericEnumSchema<'a, core::primitive::u64>,
)]
pub enum LifetimeAndTypeGenericGenericEnumSchema<'a, T> {
    Foo(&'a str, T),
}

/// Combined Enum - lifetime & const
#[derive(utoipa::ToSchema)]
#[aliases(LifetimeAndConstGenericGenericEnumSchema2 = LifetimeAndConstGenericGenericEnumSchema<'a, 2>)]
pub enum LifetimeAndConstGenericGenericEnumSchema<'a, const N: usize> {
    Foo(&'a str, [u16; N]),
}

/// Combined Enum - lifetime & const & type
#[derive(utoipa::ToSchema)]
#[aliases(
    LifetimeAndConstAndTypeGenericGenericEnumSchema1 = LifetimeAndConstAndTypeGenericGenericEnumSchema<'a, 5, 1, core::primitive::u32>,
    LifetimeAndConstAndTypeGenericGenericEnumSchema2 = LifetimeAndConstAndTypeGenericGenericEnumSchema<'a, 6, 2, core::primitive::u64>,
)]
pub enum LifetimeAndConstAndTypeGenericGenericEnumSchema<'a, const N: usize, const N2: usize, T> {
    Foo(&'a str, [u16; N], [u16; N2], T),
}

/// Discover schema with generics
#[utoipauto(paths = "./utoipauto/tests/default_features/generics.rs")]
#[derive(OpenApi)]
#[openapi(info(title = "Const Generic API", version = "1.0.0"))]
pub struct GenericsApiDocs {}

#[test]
fn test_generics() {
    assert_eq!(
        GenericsApiDocs::openapi()
            .components
            .expect("no components")
            .schemas
            .len(),
        14
    )
}
