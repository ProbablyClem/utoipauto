use std::collections::HashMap;

use utoipa::{ToResponse, ToSchema};

#[derive(Debug, ToSchema, ToResponse)]
pub struct Response<T: ToSchema> {
    pub status: u16,
    pub data: T,
}

#[derive(Debug, Clone, ToSchema, ToResponse)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

// Nested Generics
#[derive(Debug, ToSchema, ToResponse)]
pub struct NestedResponse<T: ToSchema> {
    pub response: Response<T>,
}

// Lifetime Generics
#[derive(Debug, ToSchema, ToResponse)]
pub struct BorrowedResponse<'a> {
    pub data: &'a str,
    pub additional: HashMap<&'a str, &'a i32>,
}

// Lifetime + nested Generics
#[derive(Debug, ToSchema, ToResponse)]
pub struct NestedBorrowedResponse<'a, T: ToSchema> {
    pub status: u16,
    pub data: &'a T,
}

// Combined Generics
#[derive(Debug, ToSchema, ToResponse)]
pub struct CombinedResponse<'a, T: ToSchema> {
    pub nested_response: NestedResponse<T>,
    pub borrowed_response: NestedBorrowedResponse<'a, T>,
}
