use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct NonGenericSchema;

#[derive(ToSchema)]
pub struct NonGenericSchema2;

#[derive(ToSchema)]
#[aliases(GenricModelSchema = GenericSchema < NonGenericSchema >)]
pub struct GenericSchema<T> {
    _data: T,
}

#[derive(ToSchema)]
#[aliases(MultipleGenericModelSchema = MultipleGenericSchema < NonGenericSchema, NonGenericSchema2 >)]
pub struct MultipleGenericSchema<T, U> {
    _data: T,
    _data2: U,
}

#[derive(ToSchema)]
#[aliases(
MultipleAlaises1 = MultipleAliasesSchema < NonGenericSchema >,
MultipleAlaises2 = MultipleAliasesSchema < NonGenericSchema2 >
)]
pub struct MultipleAliasesSchema<T> {
    _data: T,
}

#[derive(ToSchema)]
#[aliases(NestedGenericsSchema = NestedGenerics < GenericSchema < NonGenericSchema > >)]
pub struct NestedGenerics<T> {
    _data: T,
}
