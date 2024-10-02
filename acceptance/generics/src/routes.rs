use crate::schemas::{ArrayResponse, BorrowedResponse, CombinedResponse, NestedResponse, Person, Response};

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
    path = "/array_persons",
    responses(
(status = 200, description = "An ArrayResponse<Person, 3>", content_type = "application/json", body = ArrayResponse<Person, 3>),
    )
)]
pub fn get_array_persons() -> ArrayResponse<Person, 3> {
    ArrayResponse {
        status: 200,
        data: [
            Person { name: "John Doe".to_string(), age: 30 },
            Person { name: "Jane Doe".to_string(), age: 25 },
            Person { name: "Jim Doe".to_string(), age: 20 },
        ],
    }
}

#[utoipa::path(get,
    path = "/borrowed_persons",
    responses(
(status = 200, description = "A BorrowedResponse<'static, Person>", content_type = "application/json", body = BorrowedResponse<'static, Person>),
    )
)]
pub fn get_borrowed_persons() -> BorrowedResponse<'static, Person> {
    let person = Box::new(Person {
        name: "John Doe".to_string(),
        age: 30,
    });
    BorrowedResponse {
        status: 200,
        data: Box::leak(person),
    }
}

#[utoipa::path(get,
    path = "/combined_persons",
    responses(
(status = 200, description = "A CombinedResponse<'static, Person, 3>", content_type = "application/json", body = CombinedResponse<'static, Person, 3>),
    )
)]
pub fn get_combined_persons() -> CombinedResponse<'static, Person, 3> {
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
        array_response: ArrayResponse {
            status: 200,
            data: [
                person_ref.clone(),
                Person { name: "Jane Doe".to_string(), age: 25 },
                Person { name: "Jim Doe".to_string(), age: 20 },
            ],
        },
        borrowed_response: BorrowedResponse {
            status: 200,
            data: person_ref,
        },
    }
}
