use std::collections::HashMap;

use crate::schemas::{BorrowedResponse, CombinedResponse, NestedBorrowedResponse, NestedResponse, Person, Response};

#[utoipa::path(get,
    path = "/persons",
    responses(
(status = 200, description = "A Response<Person>", content_type = "application/json", body = Response<Person>),
    )
)]
pub fn get_persons() -> Response<Person> {
    Response {
        status: 200,
        data: Person {
            name: "John Doe".to_string(),
            age: 30,
        },
    }
}

#[utoipa::path(get,
    path = "/nested_persons",
    responses(
(status = 200, description = "A NestedResponse<Person>", content_type = "application/json", body = NestedResponse<Person>),
    )
)]
pub fn get_nested_persons() -> NestedResponse<Person> {
    NestedResponse {
        response: Response {
            status: 200,
            data: Person {
                name: "John Doe".to_string(),
                age: 30,
            },
        },
    }
}

#[utoipa::path(get,
    path = "/borrowed_persons",
    responses(
(status = 200, description = "A BorrowedResponse<'static>", content_type = "application/json", body = BorrowedResponse<'static>),
    )
)]
pub fn get_borrowed_persons() -> BorrowedResponse<'static> {
    let additional = HashMap::from([("first", &42), ("second", &-3)]);
    BorrowedResponse {
        data: "Test",
        additional,
    }
}

#[utoipa::path(get,
    path = "/nested_borrowed_persons",
    responses(
(status = 200, description = "A NestedBorrowedResponse<'static, Person>", content_type = "application/json", body = NestedBorrowedResponse<'static, Person>),
    )
)]
pub fn get_nested_borrowed_persons() -> NestedBorrowedResponse<'static, Person> {
    let person = Box::new(Person {
        name: "John Doe".to_string(),
        age: 30,
    });
    NestedBorrowedResponse {
        status: 200,
        data: Box::leak(person),
    }
}

#[utoipa::path(get,
    path = "/combined_persons",
    responses(
(status = 200, description = "A CombinedResponse<'static, Person>", content_type = "application/json", body = CombinedResponse<'static, Person>),
    )
)]
pub fn get_combined_persons() -> CombinedResponse<'static, Person> {
    let person = Box::new(Person {
        name: "John Doe".to_string(),
        age: 30,
    });
    let person_ref = Box::leak(person);
    CombinedResponse {
        nested_response: NestedResponse {
            response: Response {
                status: 200,
                data: person_ref.clone(),
            },
        },
        borrowed_response: NestedBorrowedResponse {
            status: 200,
            data: person_ref,
        },
    }
}
