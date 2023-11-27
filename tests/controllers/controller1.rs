use utoipa_auto_discovery::utoipa_ignore;

#[utoipa::path(post, path = "/route1")]
pub fn route1() {}

#[utoipa_ignore]
#[utoipa::path(post, path = "/route-ignored")]
pub fn route_ignored() {}
