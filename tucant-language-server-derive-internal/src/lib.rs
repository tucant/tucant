use std::fs;

use derive_more::TryInto;
use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

use sha3::{Digest, Sha3_224};
use syn::{parse::Nothing, parse_macro_input, Error};

// Try https://crates.io/crates/schemafy (maybe not, e.g. anyOf would be badly named etc)

// this is manually extracted from the metaModel.schema.json (but we should probably generate this at some point)
// well it contains insufficient information e.g. no default for proposed so we need to probably do this manually

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents an `and`type (e.g. TextDocumentParams & WorkDoneProgressParams`).
/// kind = "and"
struct AndType {
    items: Vec<Type>,
}

/// Represents an array type (e.g. `TextDocument[]`).
/// kind = "array"
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct ArrayType {
    element: Box<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a base type like `string` or `DocumentUri`.
/// kind = "base"
struct BaseType {
    name: BaseTypes,
}

#[derive(Serialize, Deserialize, Debug)]
enum BaseTypes {
    #[serde(rename = "URI")]
    Uri,
    #[serde(rename = "DocumentUri")]
    DocumentUri,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "uinteger")]
    UnsignedInteger,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "RegExp")]
    RegExp,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "null")]
    Null,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a boolean literal type (e.g. `kind: true`).
/// kind = "booleanLiteral"
struct BooleanLiteralType {
    value: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Defines an enumeration.
struct Enumeration {
    /// An optional documentation.
    documentation: Option<String>,
    /// The name of the enumeration.
    name: String,
    /// Whether this is a proposed enumeration. If omitted, the enumeration is final.
    #[serde(default)]
    proposed: bool,
    /// Since when (release number) this enumeration is available. Is undefined if not known.
    since: Option<String>,
    /// Whether the enumeration supports custom values (e.g. values which are not part of the set defined in `values`). If omitted no custom values are supported.
    #[serde(default)]
    supports_custom_values: bool,
    /// The type of the elements.
    #[serde(rename = "type")]
    _type: EnumerationType,
    /// The enum values.
    values: Vec<EnumerationEntry>,
}

#[derive(Serialize, Deserialize, Debug, TryInto)]
#[try_into(owned, ref)]
#[serde(untagged)]
enum StringOrNumber {
    String(String),
    Number(i64),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Defines an enumeration entry.
struct EnumerationEntry {
    /// An optional documentation.
    documentation: Option<String>,
    /// The name of the enum item.
    name: String,
    /// Whether this is a proposed enumeration entry. If omitted, the enumeration entry is final.
    #[serde(default)]
    proposed: bool,
    /// Since when (release number) this enumeration entry is available. Is undefined if not known.
    since: Option<String>,
    /// The value.
    value: StringOrNumber,
}

#[derive(Serialize, Deserialize, Debug)]
enum StringOrIntegerOrUnsignedIntegerLiteral {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "uinteger")]
    UnsignedInteger,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
enum EnumerationType {
    #[serde(rename = "base")]
    Base {
        name: StringOrIntegerOrUnsignedIntegerLiteral,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents an integer literal type (e.g. `kind: 1`).
/// kind = "integerLiteral"
struct IntegerLiteralType {
    value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
enum UriOrDocumentUriOrStringOrInteger {
    #[serde(rename = "URI")]
    Uri,
    #[serde(rename = "DocumentUri")]
    DocumentUri,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "string")]
    String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
/// Represents a type that can be used as a key in a map type. If a reference type is used then the type must either resolve to a `string` or `integer` type. (e.g. `type ChangeAnnotationIdentifier === string`).
enum MapKeyType {
    #[serde(rename = "base")]
    Base {
        name: UriOrDocumentUriOrStringOrInteger,
    },
    #[serde(rename = "reference")]
    Reference(ReferenceType),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a JSON object map (e.g. `interface Map<K extends string | integer, V> { [key: K] => V; }`).
/// kind = "map"
struct MapType {
    key: MapKeyType,
    value: Box<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Indicates in which direction a message is sent in the protocol.
enum MessageDirection {
    #[serde(rename = "clientToServer")]
    ClientToServer,
    #[serde(rename = "serverToClient")]
    ServerToClient,
    #[serde(rename = "both")]
    Both,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct MetaData {
    /// The protocol version.
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// The actual meta model.
struct MetaModel {
    /// The enumerations.
    enumerations: Vec<Enumeration>,
    /// Additional meta data.
    meta_data: MetaData,
    /// The notifications.
    notifications: Vec<Notification>,
    /// The requests.
    requests: Vec<Request>,
    /// The structures.
    structures: Vec<Structure>,
    /// The type aliases.
    type_aliases: Vec<TypeAlias>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum TypeOrVecType {
    Type(Type),
    VecType(Vec<Type>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a LSP notification
struct Notification {
    /// An optional documentation
    documentation: Option<String>,
    /// The direction in which this notification is sent in the protocol.
    message_direction: MessageDirection,
    /// The request's method name.
    method: String,
    /// The parameter type(s) if any.
    params: Option<TypeOrVecType>,
    /// Whether this is a proposed notification. If omitted the notification is final.
    #[serde(default)]
    proposed: bool,
    /// Optional a dynamic registration method if it different from the request's method.
    registration_method: Option<String>,
    /// Optional registration options if the notification supports dynamic registration.
    registration_options: Option<Type>,
    /// Since when (release number) this notification is available. Is undefined if not known.
    since: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents an `or` type (e.g. `Location | LocationLink`).
/// kind = "or"
struct OrType {
    items: Vec<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents an object property.
struct Property {
    /// An optional documentation.
    documentation: Option<String>,
    /// The property name;
    name: String,
    /// Whether the property is optional. If omitted, the property is mandatory.
    #[serde(default)]
    optional: bool,
    /// Whether this is a proposed property. If omitted, the structure is final.
    #[serde(default)]
    proposed: bool,
    /// Since when (release number) this property is available. Is undefined if not known.
    since: Option<String>,
    /// The type of the property
    #[serde(rename = "type")]
    _type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a reference to another type (e.g. `TextDocument`). This is either a `Structure`, a `Enumeration` or a `TypeAlias` in the same meta model.
/// kind = "reference"
struct ReferenceType {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a LSP request
struct Request {
    /// An optional documentation
    documentation: Option<String>,
    /// An optional error data type.
    error_data: Option<Type>,
    /// The direction in which this request is sent in the protocol.
    message_direction: MessageDirection,
    /// The request's method name.
    method: String,
    /// The parameter type(s) if any.
    params: Option<TypeOrVecType>,
    /// Optional partial result type if the request supports partial result reporting.
    partial_result: Option<Type>,
    /// Whether this is a proposed feature. If omitted the feature is final.
    #[serde(default)]
    proposed: bool,
    /// Optional a dynamic registration method if it different from the request's method.
    registration_method: Option<String>,
    /// Optional registration options if the request supports dynamic registration.
    registration_options: Option<Type>,
    /// The result type.
    result: Type,
    /// Since when (release number) this request is available. Is undefined if not known.
    since: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a string literal type (e.g. `kind: 'rename'`).
/// kind = "stringLiteral"
struct StringLiteralType {
    value: String,
}

/// Defines the structure of an object literal.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Structure {
    /// An optional documentation
    documentation: Option<String>,
    /// Structures extended from. This structures form a polymorphic type hierarchy.
    #[serde(default)]
    extends: Vec<Type>,
    /// Structures to mix in. The properties of these structures are `copied` into this structure. Mixins don't form a polymorphic type hierarchy in LSP.
    #[serde(default)]
    mixins: Vec<Type>,
    /// The name of the structure.
    name: String,
    /// The properties.
    properties: Vec<Property>,
    /// Whether this is a proposed structure. If omitted, the structure is final.
    #[serde(default)]
    proposed: bool,
    /// Since when (release number) this structure is available. Is undefined if not known.
    since: Option<String>,
}

/// Defines a unnamed structure of an object literal.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct StructureLiteral {
    /// An optional documentation
    documentation: Option<String>,
    /// The properties.
    properties: Vec<Property>,
    /// Whether this is a proposed structure. If omitted, the structure is final.
    #[serde(default)]
    proposed: bool,
    /// Since when (release number) this structure is available. Is undefined if not known.
    since: Option<String>,
}

/// Represents a literal structure (e.g. `property: { start: uinteger; end: uinteger; }`).
/// kind = "literal"
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct StructureLiteralType {
    value: StructureLiteral,
}

/// Represents a `tuple` type (e.g. `[integer, integer]`).
/// kind = "tuple"
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct TupleType {
    items: Vec<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
enum Type {
    #[serde(rename = "base")]
    Base(BaseType),
    #[serde(rename = "reference")]
    Reference(ReferenceType),
    #[serde(rename = "array")]
    Array(ArrayType),
    #[serde(rename = "map")]
    Map(MapType),
    #[serde(rename = "and")]
    And(AndType),
    #[serde(rename = "or")]
    Or(OrType),
    #[serde(rename = "tuple")]
    Tuple(TupleType),
    #[serde(rename = "literal")]
    StructureLiteral(StructureLiteralType),
    #[serde(rename = "stringLiteral")]
    StringLiteral(StringLiteralType),
    #[serde(rename = "integerLiteral")]
    IntegerLiteral(IntegerLiteralType),
    #[serde(rename = "booleanLiteral")]
    BooleanLiteral(BooleanLiteralType),
}

/// Defines a type alias. (e.g. `type Definition = Location | LocationLink`)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct TypeAlias {
    /// An optional documentation.
    documentation: Option<String>,
    /// The name of the type alias.
    name: String,
    /// Whether this is a proposed type alias. If omitted, the type alias is final.
    #[serde(default)]
    proposed: bool,
    /// Since when (release number) this structure is available. Is undefined if not known.
    since: Option<String>,
    /// The aliased type.
    #[serde(rename = "type")]
    _type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
enum TypeKind {
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "reference")]
    Reference,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "tuple")]
    Tuple,
    #[serde(rename = "literal")]
    Literal,
    #[serde(rename = "stringLiteral")]
    StringLiteral,
    #[serde(rename = "integerLiteral")]
    IntegerLiteral,
    #[serde(rename = "booleanLiteral")]
    BooleanLiteral,
}

// what about letting this return two things, one is the actual return value and the second one is anonymous struct definitions
fn handle_type(random: &mut ChaCha20Rng, _type: &Type) -> syn::Result<(TokenStream, TokenStream)> {
    match _type {
        Type::Base(BaseType { name }) => match name {
            BaseTypes::Uri => Ok((quote! { String }, quote! {})),
            BaseTypes::DocumentUri => Ok((quote! { String }, quote! {})),
            BaseTypes::Integer => Ok((quote! { i64 }, quote! {})),
            BaseTypes::UnsignedInteger => Ok((quote! { u64 }, quote! {})),
            BaseTypes::Decimal => Ok((quote! { f64 }, quote! {})),
            BaseTypes::RegExp => Ok((quote! { String }, quote! {})),
            BaseTypes::String => Ok((quote! { String }, quote! {})),
            BaseTypes::Boolean => Ok((quote! { bool }, quote! {})),
            BaseTypes::Null => Ok((quote! { () }, quote! {})),
        },
        Type::Reference(ReferenceType { name }) => {
            let name = format_ident!("r#{}", name.to_upper_camel_case());
            Ok((quote! { Box<#name> }, quote! {}))
        }
        Type::Array(ArrayType { element }) => {
            let (element, rest) = handle_type(random, element)?;
            Ok((quote! { Vec<#element> }, quote! { #rest }))
        }
        Type::Map(MapType { key, value }) => {
            let (value_type, value_rest) = handle_type(random, value)?;
            let key_type = match key {
                MapKeyType::Base {
                    name: UriOrDocumentUriOrStringOrInteger::Uri,
                } => quote! { String },
                MapKeyType::Base {
                    name: UriOrDocumentUriOrStringOrInteger::DocumentUri,
                } => quote! { String },
                MapKeyType::Base {
                    name: UriOrDocumentUriOrStringOrInteger::String,
                } => quote! { String },
                MapKeyType::Base {
                    name: UriOrDocumentUriOrStringOrInteger::Integer,
                } => quote! { i64 },
                MapKeyType::Reference(ReferenceType { name }) => {
                    let name = format_ident!("r#{}", name.to_upper_camel_case());
                    quote! { Box<#name> }
                }
            };
            Ok((
                quote! { ::std::collections::HashMap<#key_type, #value_type> },
                quote! { #value_rest },
            ))
        }
        Type::And(AndType { items: _ }) => Err(Error::new(
            Span::call_site(),
            r#"we don't support and types yet"#,
        )),
        Type::Or(OrType { items }) => {
            let mut hasher = Sha3_224::new();
            hasher.update(format!("{:?}", items));
            hasher.update(random.gen::<[u8; 32]>());
            let result = hasher.finalize();
            let result = hex::encode(result);
            let name = format_ident!("H{}", result);

            let mut err = Ok(());
            let (items, rests): (Vec<TokenStream>, Vec<TokenStream>) = items
                .iter()
                .enumerate()
                .map(|(i, item)| -> syn::Result<(TokenStream, TokenStream)> {
                    let (item_type, item_rest) = handle_type(random, item)?;
                    let name = format_ident!("Variant{}", i);
                    Ok((
                        quote! {
                            #name(#item_type),
                        },
                        quote! { #item_rest },
                    ))
                })
                .scan(&mut err, until_err)
                .unzip();
            let return_value = Ok((
                quote! {
                    #name
                },
                quote! {
                    #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
                    #[serde(untagged)]
                    pub enum #name {
                        #(#items)*
                    }
                    #(#rests)*
                },
            ));
            err?;
            return_value
        }
        Type::Tuple(TupleType { items }) => {
            let mut err = Ok(());
            let (items, rests): (Vec<TokenStream>, Vec<TokenStream>) = items
                .iter()
                .map(|v| handle_type(random, v))
                .scan(&mut err, until_err)
                .unzip();
            let return_value = Ok((
                quote! {
                    (#(#items),*)
                },
                quote! {
                    #(#rests)*
                },
            ));
            err?;
            return_value
        }
        Type::StructureLiteral(StructureLiteralType { value }) => {
            let mut hasher = Sha3_224::new();
            hasher.update(format!("{:?}", value));
            hasher.update(random.gen::<[u8; 32]>());
            let result = hasher.finalize();
            let result = hex::encode(result);
            let name = format_ident!("H{}", result);

            let mut properties_err = Ok(());
            let (properties, rest): (Vec<TokenStream>, Vec<TokenStream>) = value
                .properties
                .iter()
                .map(|property| -> syn::Result<(TokenStream, TokenStream)> {
                    let name_text = &property.name;
                    let name = format_ident!("r#{}", property.name.to_snake_case());
                    let (mut converted_type, rest) = handle_type(random, &property._type)?;

                    if property.optional {
                        converted_type = quote! { Option<#converted_type> }
                    }

                    Ok((
                        quote! {
                            #[serde(rename = #name_text)]
                            pub #name: #converted_type,
                        },
                        rest,
                    ))
                })
                .scan(&mut properties_err, until_err)
                .unzip();

            let return_value = (
                quote! {
                    #name
                },
                quote! {
                    #[::serde_with::skip_serializing_none]
                    #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
                    pub struct #name {
                        #(#properties)*
                    }
                    #(#rest)*
                },
            );
            properties_err?;
            Ok(return_value)
        }
        Type::StringLiteral(StringLiteralType { value: _ }) => Ok((quote! { String }, quote! {})),
        Type::IntegerLiteral(IntegerLiteralType { value: _ }) => Ok((quote! { i64 }, quote! {})),
        Type::BooleanLiteral(BooleanLiteralType { value: _ }) => Ok((quote! { bool }, quote! {})),
    }
}

fn until_err<T, E>(err: &mut &mut Result<(), E>, item: Result<T, E>) -> Option<T> {
    match item {
        Ok(item) => Some(item),
        Err(e) => {
            **err = Err(e);
            None
        }
    }
}

// a totally different approach which would give us line number information would be to have a magic!{} macro inside which the json is *not* inside a string. so the json would be parsed into actual tokens. possibly this could be done with serde.
pub fn handle_magic() -> syn::Result<TokenStream> {
    let file = fs::File::open("src/metaModel.json").expect("file should open read only");
    let meta_model: MetaModel = serde_json::from_reader(file).expect("file should be proper JSON");

    let mut random = ChaCha20Rng::seed_from_u64(42);

    let mut structures_err = Ok(());
    let (structures, rest_structures): (Vec<TokenStream>, Vec<TokenStream>) = meta_model
        .structures
        .iter()
        .map(|structure| -> syn::Result<(TokenStream, TokenStream)> {
            let name = format_ident!("r#{}", structure.name.to_upper_camel_case());
            let documentation = structure.documentation.as_ref().map(|string| {
                quote! {
                    #[doc = #string]
                }
            });

            let mut extends_err = Ok(());
            let (extends, rest1): (Vec<TokenStream>, Vec<TokenStream>) = structure
                .extends
                .iter()
                .enumerate()
                .map(|(i, _type)| -> syn::Result<(TokenStream, TokenStream)> {
                    // TODO FIXME would probably be nicer to assert this is a reference type and then use that name
                    let name = format_ident!("r#variant{}", i);
                    let (converted_type, rest) = handle_type(&mut random, _type)?;
                    Ok((
                        quote! {
                            #[serde(flatten)]
                            pub #name: #converted_type,
                        },
                        rest,
                    ))
                })
                .scan(&mut extends_err, until_err)
                .unzip();

            let mut mixins_err = Ok(());
            let (mixins, rest2): (Vec<TokenStream>, Vec<TokenStream>) = structure
                .mixins
                .iter()
                .enumerate()
                .map(|(i, _type)| -> syn::Result<(TokenStream, TokenStream)> {
                    // TODO FIXME would probably be nicer to assert this is a reference type and then use that name
                    let name = format_ident!("r#variant{}", structure.extends.len() + i);
                    let (converted_type, rest) = handle_type(&mut random, _type)?;
                    Ok((
                        quote! {
                            #[serde(flatten)]
                            pub #name: #converted_type,
                        },
                        rest,
                    ))
                })
                .scan(&mut mixins_err, until_err)
                .unzip();

            let mut properties_err = Ok(());
            let (properties, rest3): (Vec<TokenStream>, Vec<TokenStream>) = structure
                .properties
                .iter()
                .map(|property| -> syn::Result<(TokenStream, TokenStream)> {
                    let name_text = &property.name;
                    let name = format_ident!("r#{}", property.name.to_snake_case());
                    let documentation = property.documentation.as_ref().map(|string| {
                        quote! {
                            #[doc = #string]
                        }
                    });
                    let (mut converted_type, rest) = handle_type(&mut random, &property._type)?;

                    if property.optional {
                        converted_type = quote! { Option<#converted_type> }
                    }

                    Ok((
                        quote! {
                            #documentation
                            #[serde(rename = #name_text)]
                            pub #name: #converted_type,
                        },
                        rest,
                    ))
                })
                .scan(&mut properties_err, until_err)
                .unzip();

            let return_value = (
                quote! {
                    #[::serde_with::skip_serializing_none]
                    #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
                    #documentation
                    pub struct #name {
                        #(#extends)*
                        #(#mixins)*
                        #(#properties)*
                    }
                },
                quote! { #(#rest1)*  #(#rest2)* #(#rest3)* },
            );
            extends_err?;
            properties_err?;
            Ok(return_value)
        })
        .scan(&mut structures_err, until_err)
        .unzip();

    let mut enumerations_err = Ok(());
    let enumerations = meta_model
        .enumerations
        .iter()
        .map(|enumeration| -> syn::Result<TokenStream> {
            let name = format_ident!("r#{}", enumeration.name.to_upper_camel_case());
            let documentation = enumeration.documentation.as_ref().map(|string| {
                quote! {
                    #[doc = #string]
                }
            });
            match enumeration._type {
                EnumerationType::Base {
                    name:
                        StringOrIntegerOrUnsignedIntegerLiteral::Integer
                        | StringOrIntegerOrUnsignedIntegerLiteral::UnsignedInteger,
                } => {
                    let mut values_err = Ok(());
                    let values = enumeration
                        .values
                        .iter()
                        .map(|value| -> syn::Result<TokenStream> {
                            let name = format_ident!("r#{}", value.name.to_upper_camel_case());
                            let value: &i64 = (&value.value).try_into().unwrap();
                            Ok(quote! {
                                #name = #value,
                            })
                        })
                        .scan(&mut values_err, until_err);

                    let return_value = quote! {
                        #[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug)]
                        #[repr(i64)]
                        #documentation
                        pub enum #name {
                            #(#values)*
                        }
                    };
                    values_err?;
                    Ok(return_value)
                }
                EnumerationType::Base {
                    name: StringOrIntegerOrUnsignedIntegerLiteral::String,
                } => {
                    let mut values_err = Ok(());
                    let values = enumeration
                        .values
                        .iter()
                        .map(|value| -> syn::Result<TokenStream> {
                            let name = format_ident!("r#{}", value.name.to_upper_camel_case());
                            let documentation = value.documentation.as_ref().map(|string| {
                                quote! {
                                    #[doc = #string]
                                }
                            });
                            let value: &String = (&value.value).try_into().unwrap();
                            Ok(quote! {
                                #[serde(rename = #value)]
                                #documentation
                                #name,
                            })
                        })
                        .scan(&mut values_err, until_err);

                    let supports_custom_value = if enumeration.supports_custom_values {
                        Some(quote! {
                            #[serde(other)]
                            Other,
                        })
                    } else {
                        None
                    };

                    let return_value = quote! {
                        #[derive(serde::Serialize, serde::Deserialize, Debug)]
                        #documentation
                        pub enum #name {
                            #(#values)*
                            #supports_custom_value
                        }
                    };
                    values_err?;
                    Ok(return_value)
                }
            }
        })
        .scan(&mut enumerations_err, until_err);

    let mut type_aliases_err = Ok(());
    let (type_aliases, rest_type_aliases): (Vec<TokenStream>, Vec<TokenStream>) = meta_model
        .type_aliases
        .iter()
        .map(|type_alias| -> syn::Result<(TokenStream, TokenStream)> {
            let name = format_ident!("r#{}", type_alias.name.to_upper_camel_case());
            let (converted_type, rest) = handle_type(&mut random, &type_alias._type)?;
            let documentation = type_alias.documentation.as_ref().map(|string| {
                quote! {
                    #[doc = #string]
                }
            });
            Ok((
                quote! {
                    #documentation
                    type #name = #converted_type;
                },
                rest,
            ))
        })
        .scan(&mut type_aliases_err, until_err)
        .unzip();

    let mut requests_err = Ok(());
    let (requests, requests_rest, request_enum, response_enum): (
        Vec<TokenStream>,
        Vec<TokenStream>,
        Vec<TokenStream>,
        Vec<TokenStream>,
    ) = meta_model
        .requests
        .iter()
        .map(
            |request| -> syn::Result<(TokenStream, TokenStream, TokenStream, TokenStream)> {
                let documentation = request.documentation.as_ref().map(|string| {
                    quote! {
                        #[doc = #string]
                    }
                });
                let (client_to_server, client_to_server_rest, request_enum) =
                    if let MessageDirection::ClientToServer | MessageDirection::Both =
                        request.message_direction
                    {
                        let method = &request.method;
                        let name = format_ident!(
                            "r#{}Request",
                            request.method.replace('_', " ").to_upper_camel_case()
                        );
                        let (params, rest) = match &request.params {
                            Some(TypeOrVecType::Type(_type)) => {
                                let (the_type, rest) = handle_type(&mut random, _type)?;
                                (quote! {
                                    pub params: #the_type
                                }, rest) 
                            }
                            Some(TypeOrVecType::VecType(vec_type)) => {
                                let mut params_err = Ok(());
                                let (types, rest): (Vec<TokenStream>, Vec<TokenStream>) = vec_type
                                    .iter()
                                    .map(|_type| -> syn::Result<(TokenStream, TokenStream)> {
                                        handle_type(&mut random, _type)
                                    })
                                    .scan(&mut params_err, until_err)
                                    .unzip();
                                let return_value = (
                                    quote! {
                                        pub params: (#(#types)*)
                                    },
                                    quote! { #(#rest)* },
                                );
                                params_err?;
                                return_value
                            }
                            None => (quote! { }, quote! {}),
                        };
                        (
                            quote! {
                                #[::serde_with::skip_serializing_none]
                                #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
                                #documentation
                                pub struct #name {
                                    pub jsonrpc: String,
                                    pub id: StringOrNumber,
                                    #params
                                }
                            },
                            rest,
                            quote! {
                                #[serde(rename = #method)]
                                #name(#name),
                            },
                        )
                    } else {
                        (quote! {}, quote! {}, quote! {})
                    };
                // TODO FIXME we now always generate this because the tagging doesn't work how I thought it would
                let (server_to_client, server_to_client_rest, response_enum) =
                    if let MessageDirection::ClientToServer
                    | MessageDirection::ServerToClient
                    | MessageDirection::Both = request.message_direction
                    {
                        let method = &request.method;
                        let name = format_ident!(
                            "r#{}Response",
                            request.method.replace('_', " ").to_upper_camel_case()
                        );
                        let (result_type, result_type_rest) =
                            handle_type(&mut random, &request.result)?;
                        let (error_type, error_type_rest) = request
                            .error_data
                            .as_ref()
                            .map(|e| -> syn::Result<(TokenStream, TokenStream)> {
                                let (error_type, rest) = handle_type(&mut random, &e)?;
                                Ok((
                                    quote! {
                                        pub error: Option<#error_type>
                                    },
                                    rest,
                                ))
                            })
                            .transpose()?
                            .map_or((None, None), |o| (Some(o.0), Some(o.1)));
                        (
                            quote! {
                                #[::serde_with::skip_serializing_none]
                                #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
                                #documentation
                                pub struct #name {
                                    pub jsonrpc: String,
                                    pub id: StringOrNumber,
                                    pub result: Option<#result_type>,
                                    #error_type
                                }
                            },
                            quote! { #result_type_rest #error_type_rest },
                            quote! {
                                #[serde(rename = #method)]
                                #name(#name),
                            },
                        )
                    } else {
                        (quote! {}, quote! {}, quote! {})
                    };
                Ok((
                    quote! {
                        #client_to_server
                        #server_to_client
                    },
                    quote! {
                        #client_to_server_rest
                        #server_to_client_rest
                    },
                    quote! {
                        #request_enum
                    },
                    response_enum,
                ))
            },
        )
        .scan(&mut requests_err, until_err)
        .multiunzip();

        let (notifications, notifications_rest, client_to_server_enum, server_to_client_notification): (
            Vec<TokenStream>,
            Vec<TokenStream>,
            Vec<TokenStream>,
            Vec<TokenStream>,
        ) = meta_model
            .notifications
            .iter()
            .map(
                |notification| -> syn::Result<(TokenStream, TokenStream, TokenStream, TokenStream)> {
                    let documentation = notification.documentation.as_ref().map(|string| {
                        quote! {
                            #[doc = #string]
                        }
                    });
                    let method = &notification.method;
                    let name = format_ident!(
                        "r#{}Notification",
                        notification.method.replace('_', " ").to_upper_camel_case()
                    );
                    let (params, rest) = match &notification.params {
                        Some(TypeOrVecType::Type(_type)) => handle_type(&mut random, _type)?,
                        Some(TypeOrVecType::VecType(vec_type)) => {
                            let mut params_err = Ok(());
                            let (types, rest): (Vec<TokenStream>, Vec<TokenStream>) = vec_type
                                .iter()
                                .map(|_type| -> syn::Result<(TokenStream, TokenStream)> {
                                    handle_type(&mut random, _type)
                                })
                                .scan(&mut params_err, until_err)
                                .unzip();
                            let return_value = (
                                quote! {
                                    (#(#types)*)
                                },
                                quote! { #(#rest)* },
                            );
                            params_err?;
                            return_value
                        }
                        None => (quote! { () }, quote! {}),
                    };
                    let client_to_server_notification = if let MessageDirection::ClientToServer | MessageDirection::Both = notification.message_direction {
                        quote! {
                            #[serde(rename = #method)]
                            #name(#name),
                        }
                    } else {
                        quote! {}
                    };
                    let server_to_client_notification = if let MessageDirection::ServerToClient | MessageDirection::Both = notification.message_direction {
                        quote! {
                            #[serde(rename = #method)]
                            #name(#name),
                        }
                    } else {
                        quote! {}
                    };
                    Ok((
                        quote! {
                            #[::serde_with::skip_serializing_none]
                            #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
                            #documentation
                            pub struct #name {
                                pub jsonrpc: String,
                                pub params: #params
                            }
                        },
                        rest,
                        client_to_server_notification,
                        server_to_client_notification
                    ))
                },
            )
            .scan(&mut requests_err, until_err)
            .multiunzip();
    

    let return_value = Ok(quote! {
        #(#structures)*
        #(#enumerations)*
        #(#type_aliases)*
        #(#rest_structures)*
        #(#rest_type_aliases)*
        #(#requests)*
        #(#requests_rest)*
        #(#notifications)*
        #(#notifications_rest)*

        #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
        #[serde(untagged)]
        pub enum StringOrNumber {
            String(String),
            Number(i64),
        }

        #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
        #[serde(tag = "method")]
        pub enum Requests {
            #(#request_enum)*
            #(#client_to_server_enum)*
        }

        #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
        #[serde(tag = "method")]
        pub enum Responses {
            #(#server_to_client_notification)*
        }
    });
    structures_err?;
    enumerations_err?;
    type_aliases_err?;
    requests_err?;
    return_value
}
