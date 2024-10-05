#[derive(utoipa::ToSchema)]
pub struct Admin {
    pub name: String,
}
#[derive(utoipa::ToSchema)]
pub struct Admin2 {
    pub name: String,
    pub id: i32,
}

#[derive(utoipa::ToResponse)]
pub enum Person {
    #[response(examples(
         ("Person1" = (value = json!({"name": "name1"}))),
         ("Person2" = (value = json!({"name": "name2"})))
    ))]
    Admin(#[content("application/vnd-custom-v1+json")] Admin),

    #[response(example = json!({"name": "name3", "id": 1}))]
    Admin2(#[content("application/vnd-custom-v2+json")]
           #[to_schema] Admin2),
}

#[derive(utoipa::ToSchema)]
pub struct BadRequest {
    message: String,
}

#[derive(utoipa::IntoResponses)]
pub enum UserResponses {
    /// Success response
    #[response(status = 200)]
    Success { value: String },

    #[response(status = 404)]
    NotFound,

    #[response(status = 400)]
    BadRequest(BadRequest),
}
