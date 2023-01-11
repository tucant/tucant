// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
//#![deny(unused_results)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]
#![feature(array_try_map)]

mod debug_adapter_protocol;
pub mod json_parser;
pub mod json_schema;
pub mod json_schema_codegen;
mod utils;

use crate::json_parser::JSONValue;

// https://datatracker.ietf.org/doc/html/draft-fge-json-schema-validation-00
pub fn parse() -> Result<(), syn::Error> {
    //let json = get_debug_adapter_protocol_json();
    let json_value: JSONValue =
        syn::parse2(quote::quote! { { "$schema": { "test": ["world"] } } })?;
    println!("{json_value:#?}");
    //let schema: JSONSchema = json_value.try_into()?;
    //println!("{schema:#?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn it_works() {
        parse().unwrap();
    }
}
