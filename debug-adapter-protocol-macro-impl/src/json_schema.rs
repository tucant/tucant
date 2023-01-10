use std::collections::BTreeMap;

use itertools::Itertools;
use proc_macro2::Span;
use syn::{
    punctuated::Punctuated,
    token::{self, Brace, Comma},
    LitStr,
};

use crate::{
    json_parser::{JSONValue, KeyValue},
    utils::{extract_keys, unexpected_keys, LitStrOrd},
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct JSONSchema {
    schema: LitStr,
    title: LitStr,
    description: LitStr,
    r#type: LitStr,
    definitions: (), //(token::Brace, Punctuated<KeyValue, token::Comma>),
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

pub enum Definition {
    AllOf(),
    Type(),
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
