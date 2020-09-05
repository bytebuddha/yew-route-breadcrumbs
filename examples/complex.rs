use yew_route_breadcrumbs::BreadCrumbs;

#[derive(BreadCrumbs, Debug)]
#[breadcrumb("Index", route = "/index/")]
pub enum Routes {
    #[breadcrumbs]
    Admin(AdminRoutes),
}

#[derive(BreadCrumbs, Debug)]
#[breadcrumb("Admin", route = "/admin/")]
pub enum AdminRoutes {
    #[breadcrumb("Dashboard")]
    Dashboard,
    #[breadcrumbs]
    Users(UserRoutes),
    #[breadcrumbs]
    Email(EmailRoutes),
}

#[derive(BreadCrumbs, Debug)]
#[breadcrumb("Users", route = "/admin/users")]
pub enum UserRoutes {
    Users,
    #[breadcrumb("Create")]
    Create,
    #[breadcrumb("Edit")]
    Edit,
    #[breadcrumb("Show")]
    Show,
}

#[derive(BreadCrumbs, Debug)]
#[breadcrumb("Emails", route = "/admin/emails")]
pub enum EmailRoutes {
    #[breadcrumb("Inbox", route = "/admin/emails/inbox")]
    Inbox,
    #[breadcrumbs]
    Spam(SpamRoutes),
}

#[derive(BreadCrumbs, Debug)]
#[breadcrumbs("Spam")]
pub struct SpamRoutes;

fn main() {
    println!("{:?}", Routes::Admin(AdminRoutes::Dashboard).breadcrumbs());
    println!(
        "{:?}",
        Routes::Admin(AdminRoutes::Users(UserRoutes::Create)).breadcrumbs()
    );
    println!(
        "{:?}",
        Routes::Admin(AdminRoutes::Users(UserRoutes::Edit)).breadcrumbs()
    );
    println!(
        "{:?}",
        Routes::Admin(AdminRoutes::Users(UserRoutes::Show)).breadcrumbs()
    );
    println!(
        "{:?}",
        Routes::Admin(AdminRoutes::Email(EmailRoutes::Inbox)).breadcrumbs()
    );
    println!(
        "{:?}",
        Routes::Admin(AdminRoutes::Email(EmailRoutes::Spam(SpamRoutes))).breadcrumbs()
    );
}
