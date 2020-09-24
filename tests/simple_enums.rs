use yew_route_breadcrumbs::{BreadCrumbs, Crumb};

#[derive(BreadCrumbs)]
enum Simple {
    #[breadcrumb("A")]
    A,
    #[breadcrumb("B", route = "/route")]
    B,
    #[breadcrumb("One")]
    #[breadcrumb("Two", route = "/some/route")]
    C,
    #[breadcrumb("D", route = "route")]
    D(()),
    #[breadcrumb("E")]
    E((), ()),
    #[breadcrumb("F")]
    F { _field: () },
    #[breadcrumb("One")]
    #[breadcrumb("Two", route = "/some/route")]
    G { _field1: (), _field2: () },
}

#[test]
fn simple_enums() {
    assert_eq!(
        Some(vec![Crumb {
            text: "A",
            route: None
        }]),
        Simple::A.breadcrumbs()
    );
    assert_eq!(
        Some(vec![Crumb {
            text: "B",
            route: Some("/route".into())
        }]),
        Simple::B.breadcrumbs()
    );
    assert_eq!(
        Some(vec![
            Crumb {
                text: "One",
                route: None
            },
            Crumb {
                text: "Two",
                route: Some("/some/route".into())
            }
        ]),
        Simple::C.breadcrumbs()
    );
    assert_eq!(
        Some(vec![Crumb {
            text: "D",
            route: Some("route")
        }]),
        Simple::D(()).breadcrumbs()
    );
    assert_eq!(
        Some(vec![Crumb {
            text: "E",
            route: None
        }]),
        Simple::E((), ()).breadcrumbs()
    );
    assert_eq!(
        Some(vec![Crumb {
            text: "F",
            route: None
        }]),
        Simple::F { _field: () }.breadcrumbs()
    );
    assert_eq!(
        Some(vec![
            Crumb {
                text: "One",
                route: None
            },
            Crumb {
                text: "Two",
                route: Some("/some/route".into())
            }
        ]),
        Simple::G {
            _field1: (),
            _field2: ()
        }
        .breadcrumbs()
    );
}
