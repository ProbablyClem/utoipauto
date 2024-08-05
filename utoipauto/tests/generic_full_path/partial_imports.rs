use utoipa::ToSchema;

use crate::generic_full_path::even_more_schemas as schemas;
use crate::generic_full_path::more_schemas;

#[derive(ToSchema)]
#[aliases(
    PartialImportGenericSchemaMoreSchema = PartialImportGenericSchema < more_schemas::MoreSchema >,
    PartialImportGenericSchemaMoreSchema2 = PartialImportGenericSchema < more_schemas::MoreSchema2 >
)]
pub struct PartialImportGenericSchema<T> {
    _data: T,
}

#[derive(ToSchema)]
#[aliases(
    PartialImportEvenMoreGenericSchemaAsImport = PartialImportGenericSchema < schemas::EvenMoreSchema >,
    PartialImportEvenMoreGenericSchema2AsImport = PartialImportGenericSchema < schemas::EvenMoreSchema2 >
)]
pub struct PartialImportGenericSchemaAsImport<T> {
    _data: T,
}
