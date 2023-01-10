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
    definitions: Vec<Definition>,
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

            let (parsed_definitions, failed_definitions): (Vec<_>, Vec<_>) = definitions
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
                definitions: parsed_definitions,
            })
        } else {
            Err(syn::Error::new(value.span(), "Expected object"))
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Definition {
    title: Option<LitStr>,
    description: Option<LitStr>,
    definition_type: DefinitionType,
}

impl TryFrom<JSONValue> for Definition {
    type Error = syn::Error;

    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        let (brace, value): (token::Brace, Punctuated<KeyValue, token::Comma>) =
            value.try_into()?;

        let mut map: BTreeMap<_, _> = value
            .into_iter()
            .map(|e| (LitStrOrd(e.key), e.value))
            .collect();

        let title = map
            .remove(&LitStrOrd(LitStr::new("title", Span::call_site())))
            .map(TryInto::<LitStr>::try_into)
            .transpose()?;

        let description = map
            .remove(&LitStrOrd(LitStr::new("description", Span::call_site())))
            .map(TryInto::<LitStr>::try_into)
            .transpose()?;

        Ok(Self {
            title,
            description,
            definition_type: (brace, map).try_into()?,
        })
    }
}

#[derive(Debug)]
pub enum DefinitionType {
    AllOf(AllOfDefinition),
    Type(TypeDefinition),
    OneOf(OneOfDefinition),
    Ref(RefDefinition),
}

impl TryFrom<(Brace, BTreeMap<LitStrOrd, JSONValue>)> for DefinitionType {
    type Error = syn::Error;

    fn try_from(
        (brace, map): (Brace, BTreeMap<LitStrOrd, JSONValue>),
    ) -> Result<Self, Self::Error> {
        if map.contains_key(&LitStrOrd(LitStr::new("$ref", Span::call_site()))) {
            map.try_into().map(Self::Ref)
        } else if map.contains_key(&LitStrOrd(LitStr::new("type", Span::call_site()))) {
            map.try_into().map(Self::Type)
        } else if map.contains_key(&LitStrOrd(LitStr::new("allOf", Span::call_site()))) {
            map.try_into().map(Self::AllOf)
        } else if map.contains_key(&LitStrOrd(LitStr::new("oneOf", Span::call_site()))) {
            map.try_into().map(Self::OneOf)
        } else {
            Err(syn::Error::new(brace.span, "Unknown definition"))
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RefDefinition {
    name: LitStr,
}

impl TryFrom<BTreeMap<LitStrOrd, JSONValue>> for RefDefinition {
    type Error = syn::Error;

    fn try_from(mut map: BTreeMap<LitStrOrd, JSONValue>) -> Result<Self, Self::Error> {
        let r#ref = map
            .remove(&LitStrOrd(LitStr::new("$ref", Span::call_site())))
            .map(TryInto::<LitStr>::try_into)
            .transpose()?
            .unwrap();

        unexpected_keys(map, Ok(Self { name: r#ref }))
    }
}

#[derive(Debug)]
pub struct AllOfDefinition {
    definitions: Vec<Definition>,
}

impl TryFrom<BTreeMap<LitStrOrd, JSONValue>> for AllOfDefinition {
    type Error = syn::Error;

    fn try_from(mut map: BTreeMap<LitStrOrd, JSONValue>) -> Result<Self, Self::Error> {
        // https://datatracker.ietf.org/doc/html/draft-fge-json-schema-validation-00#section-5.5.3
        let all_of = map
            .remove(&LitStrOrd(LitStr::new("allOf", Span::call_site())))
            .unwrap();

        let (_, all_of): (token::Bracket, Punctuated<JSONValue, token::Comma>) =
            all_of.try_into()?;

        let (parsed_definitions, failed_definitions): (Vec<_>, Vec<_>) = all_of
            .into_iter()
            .map(TryInto::<Definition>::try_into)
            .partition_result();

        if let Some(error) = failed_definitions.into_iter().reduce(|mut e1, e2| {
            e1.combine(e2);
            e1
        }) {
            return Err(error);
        }

        unexpected_keys(
            map,
            Ok(Self {
                definitions: parsed_definitions,
            }),
        )
    }
}

#[derive(Debug)]
pub struct OneOfDefinition {
    definitions: Vec<Definition>,
}

impl TryFrom<BTreeMap<LitStrOrd, JSONValue>> for OneOfDefinition {
    type Error = syn::Error;

    fn try_from(mut map: BTreeMap<LitStrOrd, JSONValue>) -> Result<Self, Self::Error> {
        // https://datatracker.ietf.org/doc/html/draft-fge-json-schema-validation-00#section-5.5.5
        let one_of = map
            .remove(&LitStrOrd(LitStr::new("oneOf", Span::call_site())))
            .unwrap();

        let (_, one_of): (token::Bracket, Punctuated<JSONValue, token::Comma>) =
            one_of.try_into()?;

        let (parsed_definitions, failed_definitions): (Vec<_>, Vec<_>) = one_of
            .into_iter()
            .map(TryInto::<Definition>::try_into)
            .partition_result();

        if let Some(error) = failed_definitions.into_iter().reduce(|mut e1, e2| {
            e1.combine(e2);
            e1
        }) {
            return Err(error);
        }

        unexpected_keys(
            map,
            Ok(Self {
                definitions: parsed_definitions,
            }),
        )
    }
}

#[derive(Debug)]
pub enum TypeDefinition {
    ObjectType(ObjectType),
    StringType(StringType),
    ArrayType(ArrayType),
    IntegerType(IntegerType),
    DoubleType(DoubleType),
    BooleanType(BooleanType),
}

#[derive(Debug)]
pub struct ObjectType {}

#[derive(Debug)]
pub struct StringType {}

#[derive(Debug)]
pub struct ArrayType {
    item_type: Option<Box<Definition>>,
}

#[derive(Debug)]
pub struct IntegerType {}

#[derive(Debug)]
pub struct DoubleType {}

#[derive(Debug)]
pub struct BooleanType {}

impl TryFrom<BTreeMap<LitStrOrd, JSONValue>> for TypeDefinition {
    type Error = syn::Error;

    #[allow(clippy::too_many_lines)]
    fn try_from(mut map: BTreeMap<LitStrOrd, JSONValue>) -> Result<Self, Self::Error> {
        let r#type = map
            .remove(&LitStrOrd(LitStr::new("type", Span::call_site())))
            .unwrap();

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
                        let (_parsed_properties, failed_properties): (Vec<_>, Vec<_>) = properties
                            .into_iter()
                            .map(|property| TryInto::<Definition>::try_into(property.value))
                            .partition_result();

                        if let Some(error) = failed_properties.into_iter().reduce(|mut e1, e2| {
                            e1.combine(e2);
                            e1
                        }) {
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
                                TryInto::<Definition>::try_into(additional_properties)?;
                            }
                            value => {
                                return Err(syn::Error::new(
                                    value.span(),
                                    "Expected boolean of object",
                                ))
                            }
                        }
                    }

                    unexpected_keys(map, Ok(Self::ObjectType(ObjectType {})))
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

                    unexpected_keys(map, Ok(Self::StringType(StringType {})))
                } else if "integer" == r#type.value() {
                    unexpected_keys(map, Ok(Self::IntegerType(IntegerType {})))
                } else if "boolean" == r#type.value() {
                    unexpected_keys(map, Ok(Self::BooleanType(BooleanType {})))
                } else if "number" == r#type.value() {
                    unexpected_keys(map, Ok(Self::DoubleType(DoubleType {})))
                } else if r#type.value() == "array" {
                    let item_type = map
                        .remove(&LitStrOrd(LitStr::new("items", Span::call_site())))
                        .map(TryInto::<Definition>::try_into)
                        .transpose()?
                        .map(Box::new);

                    unexpected_keys(map, Ok(Self::ArrayType(ArrayType { item_type })))
                } else {
                    return Err(syn::Error::new(
                        r#type.span(),
                        r#"Expected an array of primitive types or one of the primitive types "object", "string", "integer", "number", "boolean" or "array""#,
                    ));
                }
            }
            JSONValue::Array(_type) => {
                // we're doomed, this is basically a oneOf?
                unexpected_keys(map, Ok(Self {}))
            }
            _ => Err(syn::Error::new(r#type.span(), "Expected string or array")),
        }
    }
}
