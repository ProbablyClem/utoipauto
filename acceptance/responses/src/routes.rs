use crate::response::{Admin, Person, UserResponses};

#[utoipa::path(
    get,
    path = "/api/user",
    responses(
        UserResponses
    )
)]
fn get_user() -> UserResponses {
    UserResponses::NotFound
}

#[utoipa::path(
    get,
    path = "/api/person",
    responses(
        (status = 200, response = Person)
    )
)]
fn get_person() -> Person {
    Person::Admin(Admin {
        name: "name1".to_string(),
    })
}
