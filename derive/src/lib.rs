//! Procedural macro that generates UI breadcrumbs from nested enum's used as routes in yew router.

#![feature(proc_macro_diagnostic)]
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

#[macro_use]
mod macros;
mod attr;
mod enu;

use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[proc_macro_derive(BreadCrumbs, attributes(breadcrumb, breadcrumbs))]
pub fn derive_breadcrumbs(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let crate_path = quote! {::yew_route_breadcrumbs};
    let ident = &derive_input.ident;
    let implementation = match &derive_input.data {
        syn::Data::Enum(enu) => enu::handle_enum(&derive_input.attrs, enu, &derive_input),
        syn::Data::Struct(_) => {
            let breadcrumbs = attr::get_attribute_crumbs(&derive_input.attrs);
            if breadcrumbs.is_empty() {
                quote! {None}
            } else {
                quote! { Some(vec![#(#breadcrumbs),*]) }
            }
        }
        _ => {
            derive_input
                .span()
                .unwrap()
                .error("BreadCrumbs can only be applied to enums and structs")
                .emit();
            quote! {}
        }
    };
    let output = quote! {
        impl #crate_path::BreadCrumbs for #ident {
            fn breadcrumbs(&self) -> Option<Vec<#crate_path::Crumb>> {
                #implementation
            }
        }
        impl #ident {
            pub fn breadcrumbs(&self) -> Option<Vec<#crate_path::Crumb>> {
                #crate_path::BreadCrumbs::breadcrumbs(self)
            }
        }
    };
    output.into()
}
