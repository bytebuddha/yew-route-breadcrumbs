# yew-route-breadcrumbs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/yew-route-breadcrumbs.svg)](https://crates.io/crates/yew-route-breadcrumbs)
[![Docs.rs](https://docs.rs/yew-route-breadcrumbs/badge.svg)](https://docs.rs/yew-route-breadcrumbs)
[![Build Status](https://travis-ci.org/bytebuddha/yew-route-breadcrumbs.svg?branch=master)](https://travis-ci.org/bytebuddha/yew-route-breadcrumbs)

Small library for generating UI breadcrumbs from the nested enums used as routes
in yew.

### Example
 ```rust
use yew_route_breadcrumbs::BreadCrumbs;

#[derive(Debug, BreadCrumbs)]
pub enum AppRoutes {
  Index,
  #[breadcrumb("Contact")]
  Contact,  // Contact
  #[breadcrumbs]
  Admin(AdminRoutes)
}

#[derive(Debug, BreadCrumbs)]
#[breadcrumb("Admin", route = "/admin/")]
pub enum AdminRoutes {
  #[breadcrumb("Dashboard")]
  Dashboard, // Admin > Dashboard
  #[breadcrumb("Users")]
  Users, // Admin > Users
  #[breadcrumb("Users", route = "/admin/users/")]
  #[breadcrumb("Create")]
  CreateUser // Admin > Users > Create
}
 ```
