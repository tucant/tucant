#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]
#![feature(proc_macro_diagnostic)]

// https://github.com/rust-lang/rfcs/pull/3200
// https://github.com/rust-lang/rust/pull/82682
// https://github.com/rust-lang/rustc-dev-guide/blob/master/src/diagnostics.md
// https://users.rust-lang.org/t/how-to-provide-quickfix-recommendations-in-proc-macros/61582/1
// https://internals.rust-lang.org/t/user-defined-quick-fixes/14926
// https://github.com/rust-lang/rust/issues/54140#issuecomment-802701867

use debug_adapter_protocol_macro_impl::{
    json_parser::JSONValue, json_schema::JSONSchema, json_schema_codegen::codegen,
};
use quote::quote;
use syn::parse_macro_input;

fn debug_adapter_protocol_macro_impl(
    input: JSONValue,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let input: JSONSchema = input.try_into()?;

    //println!("{_input:#?}");

    codegen(input)
}

#[proc_macro]
pub fn debug_adapter_protocol_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as JSONValue);

    proc_macro::TokenStream::from(
        debug_adapter_protocol_macro_impl(input).unwrap_or_else(syn::Error::into_compile_error),
    )
}
