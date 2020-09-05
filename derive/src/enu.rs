use syn::spanned::Spanned;
use syn::export::TokenStream2;
use syn::{Fields, Ident, Attribute, DeriveInput, Variant, DataEnum};

use crate::attr::{has_breadcrumbs_attribute, get_attribute_crumbs};

/// Generate the enum implementation of the `breadcrumbs` function.
pub fn handle_enum(global: &Vec<Attribute>, enu: &DataEnum, input: &DeriveInput) -> TokenStream2 {
    let ty = &input.ident;
    let iterator = enu.variants.iter().map(|item| handle_enum_variant(global, ty, item));
    quote! {
        match self {
            #(#iterator,)*
            _ => None
        }
    }
}

/// Handle a single enum variant returning a TokenStream of the Variants section in the resulting
/// match statement.
fn handle_enum_variant(type_attr: &Vec<Attribute>, ty: &Ident, var: &Variant) -> TokenStream2 {
    let var_ident = &var.ident;
    let attributes = get_attribute_crumbs(&var.attrs);
    let type_breadcrumbs = get_attribute_crumbs(&type_attr);
    match &var.fields {
        Fields::Unit => quote! { #ty::#var_ident => Some(vec![#(#type_breadcrumbs,)* #(#attributes,)*]) },
        Fields::Named(_) => quote! { #ty::#var_ident { .. } => Some(vec![#(#type_breadcrumbs,)* #(#attributes,)*]) },
        Fields::Unnamed(fields) => {
            let count = fields.unnamed.iter().count();
            if has_breadcrumbs_attribute(&var.attrs) {
                if count != 1 {
                    error!(fields, "Nested Attribute can only be applied to variants with one unnamed field");
                    panic!("Inget_attribute_crumbsvalid breadcrumb attribute");
                }
                quote! { #ty::#var_ident(inner) => inner.breadcrumbs().map(|items| {
                    let mut final_items = vec![#(#type_breadcrumbs,)* #(#attributes,)*];
                    final_items.extend(items);
                    final_items
                })}
            } else {
                let skip_iter = (0..count).map(|_| quote! {_,});
                quote! { #ty::#var_ident(#(#skip_iter)*) => Some(vec![#(#type_breadcrumbs,)* #(#attributes,)*]) }
            }
        }
    }
}
