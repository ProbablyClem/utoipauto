#![allow(dead_code)] // This code is used in the tests

pub use utoipa_auto_macro::*;

#[cfg(test)]
mod test {
    use utoipa::OpenApi;
    use utoipa_auto_macro::utoipauto;

    #[utoipa::path(post, path = "/route1")]
    pub fn route1() {}

    #[utoipa::path(post, path = "/route2")]
    pub fn route2() {}

    #[utoipa::path(post, path = "/route3")]
    pub fn route3() {}

    /// Discover from the crate root auto
    #[utoipauto]
    #[derive(OpenApi)]
    #[openapi(info(title = "Percentage API", version = "1.0.0"))]
    pub struct CrateAutoApiDocs {}

    #[test]
    fn test_crate_auto_import_path() {
        assert_eq!(CrateAutoApiDocs::openapi().paths.paths.len(), 3)
    }
}
