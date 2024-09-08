use utoipa::ToSchema;

#[derive(ToSchema)]
#[aliases(ConstGenericStructSchema0 = ConstGenericStructSchema<0>)]
pub struct ConstGenericStructSchema<const N: usize> {
    foo: [u16; N],
}

#[derive(ToSchema)]
#[aliases(ConstGenericEnumSchema0 = ConstGenericEnumSchema<0>)]
pub enum ConstGenericEnumSchema<const N: usize> {
    Foo([u16; N]),
}
