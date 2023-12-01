#![allow(dead_code)] // This code is used in the tests
use utoipa_auto_macro::utoipa_ignore;

#[utoipa::path(post, path = "/route1")]
pub fn route1() {}

#[utoipa_ignore]
#[utoipa::path(post, path = "/route-ignored")]
pub fn route_ignored() {}
