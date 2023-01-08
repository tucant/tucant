#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]

use debug_adapter_protocol::{JSONValue, JSONSchema};
use quote::quote;
use syn::parse_macro_input;

fn internal_debug_adapter_protocol_macro(input: JSONValue) -> Result<proc_macro2::TokenStream, syn::Error> {
    let input: JSONSchema = input.try_into()?;

    Ok(quote!())
}

#[proc_macro]
pub fn debug_adapter_protocol_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as JSONValue);

    proc_macro::TokenStream::from(
        internal_debug_adapter_protocol_macro(input).unwrap_or_else(syn::Error::into_compile_error),
    )
}
