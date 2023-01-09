#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]
#![feature(proc_macro_diagnostic)]

// https://github.com/rust-lang/rfcs/pull/3200
// https://github.com/rust-lang/rust/pull/82682

use debug_adapter_protocol_macro_impl::{JSONSchema, JSONValue};
use proc_macro::Span;
use quote::quote;
use syn::parse_macro_input;

fn debug_adapter_protocol_macro_impl(
    input: JSONValue,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let _input: JSONSchema = input.try_into()?;

    Ok(quote!())
}

#[proc_macro]
pub fn debug_adapter_protocol_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    Span::call_site().help("remove this").emit();

    let input = parse_macro_input!(input as JSONValue);

    proc_macro::TokenStream::from(
        debug_adapter_protocol_macro_impl(input).unwrap_or_else(syn::Error::into_compile_error),
    )
}
