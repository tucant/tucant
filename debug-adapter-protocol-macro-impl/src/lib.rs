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

use std::collections::BTreeMap;
use std::convert::identity;

use proc_macro2::Span;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma};
use syn::{braced, bracketed, token, LitBool, LitInt, LitStr, Token};

// https://datatracker.ietf.org/doc/html/draft-fge-json-schema-validation-00

#[allow(dead_code)]
#[derive(Debug)]
pub enum JSONValue {
    Bool(LitBool),
    String(LitStr),
    Integer(LitInt),
    Array((token::Bracket, Punctuated<JSONValue, token::Comma>)),
    Object((token::Brace, Punctuated<KeyValue, token::Comma>)),
}

impl JSONValue {
    fn span(&self) -> Span {
        match self {
            Self::Bool(value) => value.span(),
            Self::String(value) => value.span(),
            Self::Integer(value) => value.span(),
            Self::Array((value, _)) => value.span,
            Self::Object((value, _)) => value.span,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct KeyValue {
    key: LitStr,
    colon: Token![:],
    value: JSONValue,
}

impl Parse for KeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            colon: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl Parse for JSONValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Brace) {
            let content;
            let brace = braced!(content in input);
            Ok(Self::Object((
                brace,
                content.parse_terminated(KeyValue::parse)?,
            )))
        } else if lookahead.peek(token::Bracket) {
            let content;
            let bracket = bracketed!(content in input);
            Ok(Self::Array((
                bracket,
                content.parse_terminated(Self::parse)?,
            )))
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::String)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Self::Integer)
        } else if lookahead.peek(LitBool) {
            input.parse().map(Self::Bool)
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn parse() -> Result<(), syn::Error> {
    //let json = get_debug_adapter_protocol_json();
    let json_value: JSONValue =
        syn::parse2(quote::quote! { { "$schema": { "test": ["world"] } } })?;
    println!("{json_value:#?}");
    //let schema: JSONSchema = json_value.try_into()?;
    //println!("{schema:#?}");
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct JSONSchema {
    schema: LitStr,
    title: LitStr,
    description: LitStr,
    r#type: LitStr,
    definitions: (token::Brace, Punctuated<KeyValue, token::Comma>),
}

#[derive(Eq, PartialEq)]
pub struct LitStrOrd(LitStr);

impl PartialOrd for LitStrOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.value().partial_cmp(&other.0.value())
    }
}

impl Ord for LitStrOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.value().cmp(&other.0.value())
    }
}

fn extract_keys<const N: usize>(
    (brace, value): (Brace, Punctuated<KeyValue, Comma>),
    keys: [&str; N],
) -> Result<[JSONValue; N], syn::Error> {
    let mut map: BTreeMap<_, _> = value
        .into_iter()
        .map(|e| (LitStrOrd(e.key), e.value))
        .collect();
    let result = keys.map(|key| {
        let corresponding_value = map.remove(&LitStrOrd(LitStr::new(key, Span::call_site())));
        corresponding_value
            .ok_or_else(|| syn::Error::new(brace.span, format!("Could not find key {key}")))
    });
    if result.iter().any(Result::is_err) {
        let results = result
            .into_iter()
            .filter_map(|r| r.err())
            .reduce(|mut e1, e2| {
                e1.combine(e2);
                e1
            })
            .unwrap();
        return Err(results);
    }
    if let Some(key) = map.into_iter().next() {
        return Err(syn::Error::new(
            key.0 .0.span(),
            format!("Unexpected key {}", key.0 .0.token()),
        ));
    }
    result.try_map(identity)
}

impl TryFrom<JSONValue> for LitStr {
    type Error = syn::Error;

    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        if let JSONValue::String(value) = value {
            Ok(value)
        } else {
            Err(syn::Error::new(value.span(), "Expected string"))
        }
    }
}

impl TryFrom<JSONValue> for (token::Brace, Punctuated<KeyValue, token::Comma>) {
    type Error = syn::Error;

    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        if let JSONValue::Object(value) = value {
            Ok(value)
        } else {
            Err(syn::Error::new(value.span(), "Expected object"))
        }
    }
}

impl TryFrom<JSONValue> for JSONSchema {
    type Error = syn::Error;

    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        if let JSONValue::Object(value) = value {
            let [schema, title, description, r#type, definitions] = extract_keys(
                value,
                ["$schema", "title", "description", "type", "definitions"],
            )?;

            Ok(Self {
                schema: schema.try_into()?,
                title: title.try_into()?,
                description: description.try_into()?,
                r#type: r#type.try_into()?,
                definitions: definitions.try_into()?,
            })
        } else {
            Err(syn::Error::new(value.span(), "Expected object"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn it_works() {
        parse().unwrap();
    }
}
