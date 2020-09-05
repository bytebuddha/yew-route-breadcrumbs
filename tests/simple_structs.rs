use yew_route_breadcrumbs::{BreadCrumbs, StaticCrumb};

#[derive(BreadCrumbs)]
struct EmptyUnit;

#[derive(BreadCrumbs)]
#[breadcrumb("Crumb")]
struct SingleUnit;

#[derive(BreadCrumbs)]
#[breadcrumb("Crumb1")]
#[breadcrumb("Crumb2")]
struct MultipleUnit;

#[derive(BreadCrumbs)]
#[breadcrumb("Crumb", route = "/some/route")]
struct SingleUnitWithRoute;

#[derive(BreadCrumbs)]
#[breadcrumb("Crumb1", route = "/with/route")]
#[breadcrumb("Crumb2", route = "/other")]
struct MultipleUnitWithRoute;

#[test]
fn simple_structs() {
    assert_eq!(None, EmptyUnit.breadcrumbs());
    assert_eq!(
        Some(vec![StaticCrumb {
            text: "Crumb",
            route: None
        }]),
        SingleUnit.breadcrumbs()
    );
    assert_eq!(
        Some(vec![
            StaticCrumb {
                text: "Crumb1",
                route: None
            },
            StaticCrumb {
                text: "Crumb2",
                route: None
            }
        ]),
        MultipleUnit.breadcrumbs()
    );
    assert_eq!(
        Some(vec![StaticCrumb {
            text: "Crumb",
            route: Some("/some/route".into())
        }]),
        SingleUnitWithRoute.breadcrumbs()
    );
    assert_eq!(
        Some(vec![
            StaticCrumb {
                text: "Crumb1",
                route: Some("/with/route".into())
            },
            StaticCrumb {
                text: "Crumb2",
                route: Some("/other".into())
            }
        ]),
        MultipleUnitWithRoute.breadcrumbs()
    );
}
