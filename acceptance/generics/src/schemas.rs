use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub struct Response<T: ToSchema> {
    pub status: u16,
    pub data: T,
}

#[derive(Debug, Clone, ToSchema)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

// Nested Generics
#[derive(Debug, ToSchema)]
pub struct NestedResponse<T: ToSchema> {
    pub response: Response<T>,
}

// Lifetime Generics
#[derive(Debug, ToSchema)]
pub struct BorrowedResponse<'a, T: ToSchema> {
    pub status: u16,
    pub data: &'a T,
}

// Combined Generics
#[derive(Debug, ToSchema)]
pub struct CombinedResponse<'a, T: ToSchema> {
    pub nested_response: NestedResponse<T>,
    pub borrowed_response: BorrowedResponse<'a, T>,
}
