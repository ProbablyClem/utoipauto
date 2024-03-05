use utoipa::ToSchema;

use crate::generic_full_path::more_schemas::MoreSchema;

#[derive(ToSchema)]
#[aliases(GenericMoreSchema = GenericSchema < MoreSchema >,
      GenericMoreSchema2 = GenericSchema < crate::generic_full_path::more_schemas::MoreSchema2 >)]
pub struct GenericSchema<T> {
    _data: T,
}
