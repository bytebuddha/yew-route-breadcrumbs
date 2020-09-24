use yew_route_breadcrumbs::{BreadCrumbs, Crumb};

#[derive(BreadCrumbs)]
#[breadcrumb("Index")]
enum Routes {
    #[breadcrumbs]
    Sub(SubRoutes),
}

#[derive(BreadCrumbs)]
#[breadcrumb("SubRoutes")]
enum SubRoutes {
    #[breadcrumb("A")]
    A,
    #[breadcrumb("B", route = "route")]
    B,
    #[breadcrumbs]
    #[breadcrumb("Overall")]
    C(SubSubRoutes),
}

#[derive(BreadCrumbs)]
enum SubSubRoutes {
    #[breadcrumb("D")]
    D,
    #[breadcrumb("E", route = "route")]
    E,
    #[breadcrumb("F")]
    F,
}

#[test]
fn nested() {
    assert_eq!(
        Some(vec![
            Crumb {
                text: "Index",
                route: None
            },
            Crumb {
                text: "SubRoutes",
                route: None
            },
            Crumb {
                text: "A",
                route: None
            }
        ]),
        Routes::Sub(SubRoutes::A).breadcrumbs()
    );
    assert_eq!(
        Some(vec![
            Crumb {
                text: "Index",
                route: None
            },
            Crumb {
                text: "SubRoutes",
                route: None
            },
            Crumb {
                text: "B",
                route: Some("route")
            }
        ]),
        Routes::Sub(SubRoutes::B).breadcrumbs()
    );
    assert_eq!(
        Some(vec![
            Crumb {
                text: "Index",
                route: None
            },
            Crumb {
                text: "SubRoutes",
                route: None
            },
            Crumb {
                text: "Overall",
                route: None
            },
            Crumb {
                text: "D",
                route: None
            }
        ]),
        Routes::Sub(SubRoutes::C(SubSubRoutes::D)).breadcrumbs()
    );
    assert_eq!(
        Some(vec![
            Crumb {
                text: "Index",
                route: None
            },
            Crumb {
                text: "SubRoutes",
                route: None
            },
            Crumb {
                text: "Overall",
                route: None
            },
            Crumb {
                text: "E",
                route: Some("route".into())
            }
        ]),
        Routes::Sub(SubRoutes::C(SubSubRoutes::E)).breadcrumbs()
    );
    assert_eq!(
        Some(vec![
            Crumb {
                text: "Index",
                route: None
            },
            Crumb {
                text: "SubRoutes",
                route: None
            },
            Crumb {
                text: "Overall",
                route: None
            },
            Crumb {
                text: "F",
                route: None
            }
        ]),
        Routes::Sub(SubRoutes::C(SubSubRoutes::F)).breadcrumbs()
    );
}
