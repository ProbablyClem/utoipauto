use utoipa::ToSchema;

use crate::generic_full_path::more_schemas::AsSchema as AsSchemaImport;
use crate::generic_full_path::more_schemas::MoreSchema;
use crate::generic_full_path::more_schemas::MoreSchema2;

#[derive(ToSchema)]
#[aliases(GenericMoreSchema = GenericSchema < MoreSchema >,
GenericMoreSchema2 = GenericSchema < crate::generic_full_path::more_schemas::MoreSchema2 >)]
pub struct GenericSchema<T> {
    _data: T,
}

#[derive(ToSchema)]
#[aliases(MoreGenericSchema2 = MoreGenericSchema < AsSchemaImport >)]
pub struct MoreGenericSchema<T> {
    _data: T,
}

#[derive(ToSchema)]
#[aliases(MultipleGenericsSchema = MultipleGenerics < MoreSchema, MoreSchema2 >)]
pub struct MultipleGenerics<T, U> {
    _data: T,
    _data2: U,
}

#[derive(ToSchema)]
#[aliases(NestedGenericsSchema = NestedGenerics < MoreGenericSchema < MoreSchema > >)]
pub struct NestedGenerics<T> {
    _data: T,
}
