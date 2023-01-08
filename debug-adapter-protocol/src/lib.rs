#![feature(array_try_map)]

use std::collections::{HashMap, HashSet};

use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Colon, Comma};
//mod debugAdapterProtocol;
//use crate::debugAdapterProtocol::get_debug_adapter_protocol_json;
use syn::parse::Parse;
use syn::{braced, bracketed, token, LitInt, LitStr, Token};

#[derive(Debug)]
pub enum JSONValue {
    String(LitStr),
    Integer(LitInt),
    Array((token::Bracket, Punctuated<JSONValue, token::Comma>)),
    Object((token::Brace, Punctuated<KeyValue, token::Comma>)),
}

impl JSONValue {
    fn span(&self) -> Span {
        match self {
            JSONValue::String(value) => value.span(),
            JSONValue::Integer(value) => value.span(),
            JSONValue::Array((value, _)) => value.span,
            JSONValue::Object((value, _)) => value.span,
        }
    }
}

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
            Ok(JSONValue::Object((
                brace,
                content.parse_terminated(KeyValue::parse)?,
            )))
        } else if lookahead.peek(token::Bracket) {
            let content;
            let bracket = bracketed!(content in input);
            Ok(JSONValue::Array((
                bracket,
                content.parse_terminated(JSONValue::parse)?,
            )))
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::String)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Self::Integer)
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn parse() -> Result<(), syn::Error> {
    //let json = get_debug_adapter_protocol_json();
    let json_value: JSONValue = syn::parse2(quote::quote! { { "hello": { "test": ["world"] } } })?;
    println!("{:#?}", json_value);
    let schema: JSONSchema = json_value.try_into()?;
    println!("{:#?}", schema);
    Ok(())
}

#[derive(Debug)]
pub struct JSONSchema {
    schema: LitStr,
    title: LitStr,
    description: LitStr,
    r#type: LitStr,
    definitions: (token::Brace, Punctuated<KeyValue, token::Comma>),
}

fn extract_keys<const N: usize>(
    (brace, value): (Brace, Punctuated<KeyValue, Comma>),
    keys: [&str; N],
) -> Result<[JSONValue; N], syn::Error> {
    let mut map: HashMap<_, _> = value.into_iter().map(|e| (e.key, e.value)).collect();
    let result = keys.try_map(|key| {
        let corresponding_value = map.remove(&LitStr::new(key, Span::call_site()));
        corresponding_value
            .ok_or_else(|| syn::Error::new(brace.span, format!("Could not find key {}", key)))
    });
    if let Some(key) = map.into_iter().next() {
        return Err(syn::Error::new(
            key.0.span(),
            format!("Unexpected key {}", key.0.token()),
        ));
    }
    result
}

impl TryFrom<JSONValue> for JSONSchema {
    type Error = syn::Error;

    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        if let JSONValue::Object(value) = value {
            let schema = extract_keys(value, ["test"])?;

            Ok(Self {
                schema: todo!(),
                title: todo!(),
                description: todo!(),
                r#type: todo!(),
                definitions: todo!(),
            })
        } else {
            Err(syn::Error::new(value.span(), "Expected object"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parse().unwrap();
    }
}
