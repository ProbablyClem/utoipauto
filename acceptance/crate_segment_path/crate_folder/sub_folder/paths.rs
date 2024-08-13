#![allow(dead_code)] // This code is used in the tests

#[utoipa::path(post, path = "/route1")]
pub fn route1() {}

#[utoipa::path(post, path = "/route2")]
pub fn route2() {}
