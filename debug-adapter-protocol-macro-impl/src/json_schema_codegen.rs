use quote::{format_ident, quote, quote_spanned};

use crate::json_schema::JSONSchema;

pub fn codegen(schema: JSONSchema) -> Result<proc_macro2::TokenStream, syn::Error> {
    let definitions = schema.definitions.into_iter().map(|definition| {
        let name = format_ident!("{}", definition.key.value());
        let name = quote_spanned!(definition.key.span() => #name);
        quote! {
            struct #name {

            }
        }
    });
    let description = schema.description;
    Ok(quote! {
        #(#definitions)*
    })
}
