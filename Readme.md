# yew-route-breadcrumbs

Small library for generating UI breadcrumbs from the nested enums used as routes
in yew.

### Example
 ```rust
use yew_route_breadcrumbs::BreadCrumbs;

#[derive(Debug, BreadCrumbs)]
pub enum AppRoutes {
  Index,
  #[breadcrumb("Contact")]
  Contact,
  #[breadcrumbs]
  Admin(AdminRoutes)
}

#[derive(Debug, BreadCrumbs)]
#[breadcrumb("Admin", route = "/admin/")]
pub enum AdminRoutes {
  #[breadcrumb("Dashboard")]
  Dashboard,
  #[breadcrumb("Users")]
  Users,
  #[breadcrumb("Users", route = "/admin/users/")]
  #[breadcrumb("Create")]
  CreateUser
}
 ```
