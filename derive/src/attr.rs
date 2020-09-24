use syn::spanned::Spanned;
use syn::export::TokenStream2;
use syn::{Lit, Meta, NestedMeta, Attribute};

/// Determine whether the attribute vector contains the `breadcrumbs` attribute.
pub fn has_breadcrumbs_attribute(attrs: &Vec<Attribute>) -> bool {
    for attr in attrs {
        if let Some(name) = attr.path.get_ident() {
            if "breadcrumbs" == format!("{}", name) {
                return true;
            }
        }
    }
    false
}

/// Iterate through the attribute vector and turn any `breadcrumb` attributes into a
/// corresponding `Crumb` creation TokenStream.
pub fn get_attribute_crumbs(attrs: &Vec<Attribute>) -> Vec<TokenStream2> {
    let mut final_crumbs = vec![];
    for attr in attrs {
        if let Some(ident) = attr.path.get_ident() {
            if &format!("{}", &ident) == "breadcrumb" {
                match attr.parse_meta().expect("Failed to parse attribute meta") {
                    Meta::List(list) => parse_attribute_meta(&mut final_crumbs, list),
                    _ => {error!(attr, "Expected Meta List Attribute");panic!("Invalid breadcrumb attribute")}
                }
            }
        }
    }
    final_crumbs
}

fn parse_attribute_meta(breadcrumbs: &mut Vec<TokenStream2>, list: syn::MetaList) {
    let text;
    let mut route = None;
    let mut list_iter = list.nested.iter();
    let first = list_iter.next();

    if let Some(NestedMeta::Lit(Lit::Str(string))) = first {
        text = Some(string);
    } else {
        error!(first, "Expected first argument to be a static string");
        panic!("Invalid breadcrum attribute");
    }
    if let Some(NestedMeta::Meta(Meta::NameValue(named))) = list_iter.next() {
        if let Some(ident) = named.path.get_ident() {
            match format!("{}", ident).as_ref() {
                "route" => {
                    match &named.lit {
                        Lit::Str(string) => route = Some(string),
                        _ => error!(named, "Expected route param to be a string")
                    }
                },
                _ => {}
            }
        }
    }
    if let Some(text) = text {
        let route = route.map(|item|quote!{Some(#item)}).unwrap_or_else(|| quote!{None});
        breadcrumbs.push(quote! {
            ::yew_route_breadcrumbs::Crumb { text: #text, route: #route }
        });
    }
}
