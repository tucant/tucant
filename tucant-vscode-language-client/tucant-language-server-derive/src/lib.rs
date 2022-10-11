use std::fs;

use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use syn::{parse::Nothing, parse_macro_input, Error, LitStr, token::{Brace, Enum}, Expr};

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
struct ArrayType {
    element: Type,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a base type like `string` or `DocumentUri`.
/// kind = "base"
struct BaseType {
    name: BaseTypes
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
    Null
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a boolean literal type (e.g. `kind: true`).
/// kind = "booleanLiteral"
struct BooleanLiteralType {
    value: bool
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
    supports_custom_values: bool,
    /// The type of the elements.
    #[serde(rename = "type")]
    _type: EnumerationType,
    /// The enum values.
    values: Vec<EnumerationEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    since: String,
    /// The value.
    value: StringOrNumber
}

#[derive(Serialize, Deserialize, Debug)]
enum StringOrIntegerOrUnsignedIntegerLiteral {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "uinteger")]
    UnsignedInteger
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// kind = "base"
struct EnumerationType {
    name: StringOrIntegerOrUnsignedIntegerLiteral
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents an integer literal type (e.g. `kind: 1`).
/// kind = "integerLiteral"
struct IntegerLiteralType {
    value: i64
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
    Base {
        name: UriOrDocumentUriOrStringOrInteger,
    },
    Reference(ReferenceType),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a JSON object map (e.g. `interface Map<K extends string | integer, V> { [key: K] => V; }`).
/// kind = "map"
struct MapType {
    key: MapKeyType,
    value: Type,
}

#[derive(Serialize, Deserialize, Debug)]
/// Indicates in which direction a message is sent in the protocol.
enum MessageDirection {
    #[serde(rename = "clientToServer")]
    ClientToServer,
    #[serde(rename = "serverToClient")]
    ServerToClient,
    #[serde(rename = "both")]
    Both
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct MetaData {
    /// The protocol version.
    version: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The actual meta model.
struct MetaModel {
    /// The enumerations.
    enumerations: Vec<Enumeration>,
    /// Additional meta data.
    meta_data: Value,
    /// The notifications.
    notifications: Vec<Value>, // Notification
    /// The requests.
    requests: Vec<Request>,
    /// The structures.
    structures: Vec<Value>, // Structure
    /// The type aliases.
    type_aliases: Vec<Value> // TypeAlias
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a reference to another type (e.g. `TextDocument`). This is either a `Structure`, a `Enumeration` or a `TypeAlias` in the same meta model.
/// kind = "reference"
struct ReferenceType {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a string literal type (e.g. `kind: 'rename'`).
/// kind = "stringLiteral"
struct StringLiteralType {
    value: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
enum Type {
    BaseType(BaseType),
    StringLiteralType(StringLiteralType),
    IntegerLiteralType(IntegerLiteralType),
    BooleanLiteralType(BooleanLiteralType)
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a LSP request
struct Request {
    /// An optional documentation
    documentation: Option<String>,
    /// An optional error data type.
    error_data: Option<Value>, // Type
    /// The direction in which this request is sent in the protocol.
    message_direction: String, // MessageDirection
    /// The request's method name.
    method: String,
    /// The parameter type(s) if any.
    params: Option<Value>, // Type or Vec<Type> (if we can parse as Vec<Type> always)
    /// Optional partial result type if the request supports partial result reporting.
    partial_result: Option<Value>, // Type
    /// Whether this is a proposed feature. If omitted the feature is final.
    #[serde(default)]
    proposed: bool,
    /// Optional a dynamic registration method if it different from the request's method.
    registration_method: Option<String>,
    /// Optional registration options if the request supports dynamic registration.
    registration_options: Option<Value>, // Type
    /// The result type.
    result: Value, // Type
    /// Since when (release number) this request is available. Is undefined if not known.
    since: Option<String>
}

fn handle_lit_fn() -> syn::Result<TokenStream> {
    let file = fs::File::open("src/metaModel.json")
    .expect("file should open read only");
    let json: MetaModel = serde_json::from_reader(file)
    .expect("file should be proper JSON");

    println!("test: {:?}", json);

    Ok(quote! {
        pub fn dsfsf() {

        }

        struct test {
            
        }
    })
}

// cargo expand --test meta_model
#[proc_macro]
pub fn magic(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // TODO FIXME I think this parses weird
    let input = parse_macro_input!(item as Nothing);

    proc_macro::TokenStream::from(handle_lit_fn().unwrap_or_else(Error::into_compile_error))
}
