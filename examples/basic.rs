use yew_route_breadcrumbs::BreadCrumbs;

#[derive(BreadCrumbs, Debug)]
#[breadcrumb("Global Crumb", route = "/som/api/route")]
pub enum Routes {
    A,
    #[breadcrumb("Some Crumb")]
    B,
    #[breadcrumb("Has")]
    #[breadcrumb("Multiple")]
    C,
    #[breadcrumb("Route", route = "/some/api/route")]
    D,
    #[breadcrumb("Multiple")]
    #[breadcrumb("With Route", route = "/api/route")]
    E,
}

fn main() {
    println!("Routes::A - {:?}", Routes::A.breadcrumbs());
    println!("Routes::B - {:?}", Routes::B.breadcrumbs());
    println!("Routes::C - {:?}", Routes::C.breadcrumbs());
    println!("Routes::D - {:?}", Routes::D.breadcrumbs());
    println!("Routes::E - {:?}", Routes::E.breadcrumbs());
}
