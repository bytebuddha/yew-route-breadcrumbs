//! This crate is intended to be used with yew-router though it is not required
//! to build this library.
//!
//! The BreadCrumbs derive macro is the main way to interact with this crate. It derives a
//! [`BreadCrumbs`](./trait.BreadCrumbs.html) implementation block and a single `breadcrumbs`
//! helper function of the same type as the [`BreadCrumbs`](./trait.BreadCrumbs.html) trait.
//!
//! It gives access to 2 attribute macros.
//! - ### `breadcrumb`
//! The breadcrumb attribute is used to create a breadcrumb. It expects the first argument
//! to be a string literal that will be used as the breadcrumb text. The `route` argument
//! can be used to set the breadcrumb route.
//! - ### `breadcrumbs`
//! The breadcrumbs attribute is used to mark the field as nested. This only functions on
//! structs with a single unnamed field.
//!
//! ##### Example
//! ```
//! use yew_route_breadcrumbs::BreadCrumbs;
//!
//! #[derive(BreadCrumbs)]
//! #[breadcrumb("Global1")]
//! enum AppRoutes {
//!     #[breadcrumb("Blog")]
//!     Blog, // Global1 > Blog
//!     #[breadcrumb("Auth")]
//!     Auth(AuthRoutes),
//!     #[breadcrumbs]
//!     Admin(AdminRoutes)
//! }
//!
//! #[derive(BreadCrumbs)]
//! #[breadcrumb("Admin")]
//! enum AdminRoutes {
//!     #[breadcrumb("Users", route = "/admin/users")]
//!     Users, // Global1 > Admin > Users
//!     #[breadcrumb("Roles")]
//!     Roles // Global1 > Admin > Roles
//! }
//!
//! #[derive(BreadCrumbs)]
//! enum AuthRoutes {
//!     #[breadcrumb("Login", route = "/auth/login")]
//!     Login, // Global1 > Auth > Login
//!     #[breadcrumb("Register")]
//!     Register // Global1 > Auth > Register
//! }
//! ```
pub use yew_route_breadcrumbs_derive::BreadCrumbs;

/// A single UI BreadCrumb. A Vector of `Crumbs` can be used to render
/// the route position in the UI.
#[derive(Debug, PartialEq, Clone)]
pub struct Crumb {
    pub text: String,
    pub route: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StaticCrumb {
    pub text: &'static str,
    pub route: Option<&'static str>,
}

/// Helper trait used by the library
pub trait BreadCrumbs {
    /// The resulting vector should always contain atleast one element.
    fn breadcrumbs(&self) -> Option<Vec<StaticCrumb>>;

    fn breadcrumbs_owned(&self) -> Option<Vec<Crumb>> {
        self.breadcrumbs().map(|item|item.into_iter().map(|item| Crumb {
            text: item.text.to_string(),
            route: item.route.map(|item| item.to_string())
        }).collect())
    }
}
