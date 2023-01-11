use quote::{format_ident, quote, quote_spanned};

use crate::json_schema::JSONSchema;

#[allow(clippy::wildcard_imports)] // false positive
#[must_use]
pub fn codegen(schema: JSONSchema) -> proc_macro2::TokenStream {
    let definitions = schema.definitions.into_iter().map(|definition| {
        let name = format_ident!("{}", definition.key.value());
        let name = quote_spanned!(definition.key.span() => #name);
        quote! {
            pub struct #name {

            }
        }
    });
    quote! {
        #(#definitions)*
    }
}
