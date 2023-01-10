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

pub mod json_parser;
pub mod json_schema;

use std::collections::BTreeMap;
use std::convert::identity;

use itertools::Itertools;
use json_parser::KeyValue;
use proc_macro2::Span;

use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma};
use syn::{token, LitStr};

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

#[allow(dead_code)]
#[derive(Debug)]
pub struct JSONSchema {
    schema: LitStr,
    title: LitStr,
    description: LitStr,
    r#type: LitStr,
    definitions: (), //(token::Brace, Punctuated<KeyValue, token::Comma>),
}

pub enum Definition {
    AllOf(),
    Type(),
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
    if result.iter().any(Result::is_err) || !map.is_empty() {
        let unexpected_keys = map.into_iter().map(|key| {
            return syn::Error::new(
                key.0 .0.span(),
                format!("Unexpected key {}", key.0 .0.token()),
            );
        });

        let results = unexpected_keys
            .chain(result.into_iter().filter_map(Result::err))
            .reduce(|mut e1, e2| {
                e1.combine(e2);
                e1
            })
            .unwrap();
        return Err(results);
    }
    result.try_map(identity)
}

fn unexpected_keys<T>(
    map: BTreeMap<LitStrOrd, JSONValue>,
    value: Result<T, syn::Error>,
) -> Result<T, syn::Error> {
    if value.is_err() || !map.is_empty() {
        let unexpected_keys = map.into_iter().map(|key| {
            return syn::Error::new(
                key.0 .0.span(),
                format!("Unexpected key {}", key.0 .0.token()),
            );
        });

        Err(std::iter::once(value)
            .filter_map(Result::err)
            .chain(unexpected_keys)
            .reduce(|mut e1, e2| {
                e1.combine(e2);
                e1
            })
            .unwrap())
    } else {
        value
    }
}

impl TryFrom<JSONValue> for Definition {
    type Error = syn::Error;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        let (brace, value): (token::Brace, Punctuated<KeyValue, token::Comma>) =
            value.try_into()?;

        let mut map: BTreeMap<_, _> = value
            .into_iter()
            .map(|e| (LitStrOrd(e.key), e.value))
            .collect();

        let _description = map
            .remove(&LitStrOrd(LitStr::new("description", Span::call_site())))
            .map(TryInto::<LitStr>::try_into)
            .transpose()?;
        let _title = map
            .remove(&LitStrOrd(LitStr::new("title", Span::call_site())))
            .map(TryInto::<LitStr>::try_into)
            .transpose()?;

        let r#ref = map.remove(&LitStrOrd(LitStr::new("$ref", Span::call_site())));

        if let Some(_ref) = r#ref {
            unexpected_keys(map, Ok(Self::Type()))
        } else {
            let r#type = map.remove(&LitStrOrd(LitStr::new("type", Span::call_site())));

            if let Some(r#type) = r#type {
                // https://datatracker.ietf.org/doc/html/draft-fge-json-schema-validation-00#section-5.5.2
                match r#type {
                    JSONValue::String(r#type) => {
                        // TODO FIXME run partition_result

                        // https://datatracker.ietf.org/doc/html/draft-zyp-json-schema-04#section-3.5
                        if r#type.value() == "object" {
                            // TODO FIXME use extract_keys
                            let _required =
                                map.remove(&LitStrOrd(LitStr::new("required", Span::call_site())))
                                    .map(
                                        TryInto::<(
                                            token::Bracket,
                                            Punctuated<JSONValue, token::Comma>,
                                        )>::try_into,
                                    )
                                    .transpose()?;

                            // this is optional
                            let properties = map
                        .remove(&LitStrOrd(LitStr::new("properties", Span::call_site())))
                        .map(
                            TryInto::<(token::Brace, Punctuated<KeyValue, token::Comma>)>::try_into,
                        )
                        .transpose()?;

                            if let Some((_, properties)) = properties {
                                let (_parsed_properties, failed_properties): (Vec<_>, Vec<_>) =
                                    properties
                                        .into_iter()
                                        .map(|property| TryInto::<Self>::try_into(property.value))
                                        .partition_result();

                                if let Some(error) =
                                    failed_properties.into_iter().reduce(|mut e1, e2| {
                                        e1.combine(e2);
                                        e1
                                    })
                                {
                                    return Err(error);
                                }
                            }

                            let additional_properties = map.remove(&LitStrOrd(LitStr::new(
                                "additionalProperties",
                                Span::call_site(),
                            )));

                            if let Some(additional_properties) = additional_properties {
                                match additional_properties {
                                    JSONValue::Bool(_) => {}
                                    JSONValue::Object(_) => {
                                        TryInto::<Self>::try_into(additional_properties)?;
                                    }
                                    value => {
                                        return Err(syn::Error::new(
                                            value.span(),
                                            "Expected boolean of object",
                                        ))
                                    }
                                }
                            }

                            unexpected_keys(map, Ok(Self::Type()))
                        } else if r#type.value() == "string" {
                            // https://datatracker.ietf.org/doc/html/draft-fge-json-schema-validation-00#section-5.5.1
                            let r#enum =
                                map.remove(&LitStrOrd(LitStr::new("enum", Span::call_site())))
                                    .map(
                                        TryInto::<(
                                            token::Bracket,
                                            Punctuated<JSONValue, token::Comma>,
                                        )>::try_into,
                                    )
                                    .transpose()?;

                            if let Some(_enum) = r#enum {
                                let _enum_descriptions = map
                                    .remove(&LitStrOrd(LitStr::new(
                                        "enumDescriptions",
                                        Span::call_site(),
                                    )))
                                    .map(
                                        TryInto::<(
                                            token::Bracket,
                                            Punctuated<JSONValue, token::Comma>,
                                        )>::try_into,
                                    )
                                    .transpose()?;
                            }

                            // additional enum values allowed
                            let r#enum =
                                map.remove(&LitStrOrd(LitStr::new("_enum", Span::call_site())))
                                    .map(
                                        TryInto::<(
                                            token::Bracket,
                                            Punctuated<JSONValue, token::Comma>,
                                        )>::try_into,
                                    )
                                    .transpose()?;

                            if let Some(_enum) = r#enum {
                                let _enum_descriptions = map
                                    .remove(&LitStrOrd(LitStr::new(
                                        "enumDescriptions",
                                        Span::call_site(),
                                    )))
                                    .map(
                                        TryInto::<(
                                            token::Bracket,
                                            Punctuated<JSONValue, token::Comma>,
                                        )>::try_into,
                                    )
                                    .transpose()?;
                            }

                            unexpected_keys(map, Ok(Self::Type()))
                        } else if ["integer", "number", "boolean"]
                            .contains(&r#type.value().as_str())
                        {
                            unexpected_keys(map, Ok(Self::Type()))
                        } else if r#type.value() == "array" {
                            let _items = map
                                .remove(&LitStrOrd(LitStr::new("items", Span::call_site())))
                                .map(TryInto::<Self>::try_into)
                                .transpose()?;

                            unexpected_keys(map, Ok(Self::Type()))
                        } else {
                            return Err(syn::Error::new(r#type.span(), "Expected \"object\""));
                        }
                    }
                    JSONValue::Array(_type) => unexpected_keys(map, Ok(Self::Type())),
                    _ => Err(syn::Error::new(r#type.span(), "Expected string or array")),
                }
            } else {
                let all_of = map.remove(&LitStrOrd(LitStr::new("allOf", Span::call_site())));

                if let Some(all_of) = all_of {
                    // https://datatracker.ietf.org/doc/html/draft-fge-json-schema-validation-00#section-5.5.3

                    let (_, all_of): (token::Bracket, Punctuated<JSONValue, token::Comma>) =
                        all_of.try_into()?;

                    let (_parsed_definitions, failed_definitions): (Vec<_>, Vec<_>) = all_of
                        .into_iter()
                        .map(TryInto::<Self>::try_into)
                        .partition_result();

                    if let Some(error) = failed_definitions.into_iter().reduce(|mut e1, e2| {
                        e1.combine(e2);
                        e1
                    }) {
                        return Err(error);
                    }

                    unexpected_keys(map, Ok(Self::AllOf()))
                } else {
                    let one_of = map.remove(&LitStrOrd(LitStr::new("oneOf", Span::call_site())));

                    if let Some(one_of) = one_of {
                        // https://datatracker.ietf.org/doc/html/draft-fge-json-schema-validation-00#section-5.5.5

                        let (_, one_of): (token::Bracket, Punctuated<JSONValue, token::Comma>) =
                            one_of.try_into()?;

                        let (_parsed_definitions, failed_definitions): (Vec<_>, Vec<_>) = one_of
                            .into_iter()
                            .map(TryInto::<Self>::try_into)
                            .partition_result();

                        if let Some(error) = failed_definitions.into_iter().reduce(|mut e1, e2| {
                            e1.combine(e2);
                            e1
                        }) {
                            return Err(error);
                        }

                        unexpected_keys(map, Ok(Self::AllOf()))
                    } else {
                        Err(syn::Error::new(brace.span, "Unknown definition"))
                    }
                }
            }
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

            let (_, definitions): (Brace, Punctuated<KeyValue, Comma>) = definitions.try_into()?;

            let (_parsed_definitions, failed_definitions): (Vec<_>, Vec<_>) = definitions
                .into_iter()
                .map(|definition| -> Result<_, syn::Error> {
                    TryInto::<Definition>::try_into(definition.value)
                })
                .partition_result();

            if let Some(error) = failed_definitions.into_iter().reduce(|mut e1, e2| {
                e1.combine(e2);
                e1
            }) {
                return Err(error);
            }

            Ok(Self {
                schema: schema.try_into()?,
                title: title.try_into()?,
                description: description.try_into()?,
                r#type: r#type.try_into()?,
                definitions: (),
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
