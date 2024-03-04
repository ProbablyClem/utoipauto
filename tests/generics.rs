use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct NonGenericSchema;

#[derive(ToSchema)]
pub struct NonImportedSchema;

#[derive(ToSchema)]
#[aliases(GenricModelSchema = GenericSchema < NonGenericSchema >)]
pub struct GenericSchema<T> {
    _data: T,
}

#[derive(ToSchema)]
#[aliases(MultipleGenericModelSchema = MultipleGenericSchema < NonGenericSchema, NonImportedSchema >)]
pub struct MultipleGenericSchema<T, U> {
    _data: T,
    _data2: U,
}

#[derive(ToSchema)]
#[aliases(
MultipleAlaises1 = MultipleAliasesSchema < NonGenericSchema >,
MultipleAlaises2 = MultipleAliasesSchema < NonImportedSchema >
)]
pub struct MultipleAliasesSchema<T> {
    _data: T,
}
