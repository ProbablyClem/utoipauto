#![allow(dead_code)] // This code is used in the tests

#[utoipa::path(post, path = "/route1new")]
pub fn route1new() {}

#[utoipa::path(post, path = "/route2new")]
pub fn route2new() {}
