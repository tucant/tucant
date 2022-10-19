use derive_more::TryInto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents an `and`type (e.g. TextDocumentParams & WorkDoneProgressParams`).
/// kind = "and"
pub struct AndType {
    pub items: Vec<Type>,
}

/// Represents an array type (e.g. `TextDocument[]`).
/// kind = "array"
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ArrayType {
    pub element: Box<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a base type like `string` or `DocumentUri`.
/// kind = "base"
pub struct BaseType {
    pub name: BaseTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BaseTypes {
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
pub struct BooleanLiteralType {
    pub value: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Defines an enumeration.
pub struct Enumeration {
    /// An optional documentation.
    pub documentation: Option<String>,
    /// The name of the enumeration.
    pub name: String,
    /// Whether this is a proposed enumeration. If omitted, the enumeration is final.
    #[serde(default)]
    pub proposed: bool,
    /// Since when (release number) this enumeration is available. Is undefined if not known.
    pub since: Option<String>,
    /// Whether the enumeration supports custom values (e.g. values which are not part of the set defined in `values`). If omitted no custom values are supported.
    #[serde(default)]
    pub supports_custom_values: bool,
    /// The type of the elements.
    #[serde(rename = "type")]
    pub _type: EnumerationType,
    /// The enum values.
    pub values: Vec<EnumerationEntry>,
}

#[derive(Serialize, Deserialize, Debug, TryInto)]
#[try_into(owned, ref)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(i64),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Defines an enumeration entry.
pub struct EnumerationEntry {
    /// An optional documentation.
    pub documentation: Option<String>,
    /// The name of the enum item.
    pub name: String,
    /// Whether this is a proposed enumeration entry. If omitted, the enumeration entry is final.
    #[serde(default)]
    pub proposed: bool,
    /// Since when (release number) this enumeration entry is available. Is undefined if not known.
    pub since: Option<String>,
    /// The value.
    pub value: StringOrNumber,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StringOrIntegerOrUnsignedIntegerLiteral {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "uinteger")]
    UnsignedInteger,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum EnumerationType {
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
pub struct IntegerLiteralType {
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UriOrDocumentUriOrStringOrInteger {
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
pub enum MapKeyType {
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
pub struct MapType {
    pub key: MapKeyType,
    pub value: Box<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Indicates in which direction a message is sent in the protocol.
pub enum MessageDirection {
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
pub struct MetaData {
    /// The protocol version.
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// The actual meta model.
pub struct MetaModel {
    /// The enumerations.
    pub enumerations: Vec<Enumeration>,
    /// Additional meta data.
    pub meta_data: MetaData,
    /// The notifications.
    pub notifications: Vec<Notification>,
    /// The requests.
    pub requests: Vec<Request>,
    /// The structures.
    pub structures: Vec<Structure>,
    /// The type aliases.
    pub type_aliases: Vec<TypeAlias>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TypeOrVecType {
    Type(Type),
    VecType(Vec<Type>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a LSP notification
pub struct Notification {
    /// An optional documentation
    pub documentation: Option<String>,
    /// The direction in which this notification is sent in the protocol.
    pub message_direction: MessageDirection,
    /// The request's method name.
    pub method: String,
    /// The parameter type(s) if any.
    pub params: Option<TypeOrVecType>,
    /// Whether this is a proposed notification. If omitted the notification is final.
    #[serde(default)]
    pub proposed: bool,
    /// Optional a dynamic registration method if it different from the request's method.
    pub registration_method: Option<String>,
    /// Optional registration options if the notification supports dynamic registration.
    pub registration_options: Option<Type>,
    /// Since when (release number) this notification is available. Is undefined if not known.
    pub since: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents an `or` type (e.g. `Location | LocationLink`).
/// kind = "or"
pub struct OrType {
    pub items: Vec<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents an object property.
pub struct Property {
    /// An optional documentation.
    pub documentation: Option<String>,
    /// The property name;
    pub name: String,
    /// Whether the property is optional. If omitted, the property is mandatory.
    #[serde(default)]
    pub optional: bool,
    /// Whether this is a proposed property. If omitted, the structure is final.
    #[serde(default)]
    pub proposed: bool,
    /// Since when (release number) this property is available. Is undefined if not known.
    pub since: Option<String>,
    /// The type of the property
    #[serde(rename = "type")]
    pub _type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a reference to another type (e.g. `TextDocument`). This is either a `Structure`, a `Enumeration` or a `TypeAlias` in the same meta model.
/// kind = "reference"
pub struct ReferenceType {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a LSP request
pub struct Request {
    /// An optional documentation
    pub documentation: Option<String>,
    /// An optional error data type.
    pub error_data: Option<Type>,
    /// The direction in which this request is sent in the protocol.
    pub message_direction: MessageDirection,
    /// The request's method name.
    pub method: String,
    /// The parameter type(s) if any.
    pub params: Option<TypeOrVecType>,
    /// Optional partial result type if the request supports partial result reporting.
    pub partial_result: Option<Type>,
    /// Whether this is a proposed feature. If omitted the feature is final.
    #[serde(default)]
    pub proposed: bool,
    /// Optional a dynamic registration method if it different from the request's method.
    pub registration_method: Option<String>,
    /// Optional registration options if the request supports dynamic registration.
    pub registration_options: Option<Type>,
    /// The result type.
    pub result: Type,
    /// Since when (release number) this request is available. Is undefined if not known.
    pub since: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
/// Represents a string literal type (e.g. `kind: 'rename'`).
/// kind = "stringLiteral"
pub struct StringLiteralType {
    pub value: String,
}

/// Defines the structure of an object literal.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Structure {
    /// An optional documentation
    pub documentation: Option<String>,
    /// Structures extended from. This structures form a polymorphic type hierarchy.
    #[serde(default)]
    pub extends: Vec<Type>,
    /// Structures to mix in. The properties of these structures are `copied` into this structure. Mixins don't form a polymorphic type hierarchy in LSP.
    #[serde(default)]
    pub mixins: Vec<Type>,
    /// The name of the structure.
    pub name: String,
    /// The properties.
    pub properties: Vec<Property>,
    /// Whether this is a proposed structure. If omitted, the structure is final.
    #[serde(default)]
    pub proposed: bool,
    /// Since when (release number) this structure is available. Is undefined if not known.
    pub since: Option<String>,
}

/// Defines a unnamed structure of an object literal.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StructureLiteral {
    /// An optional documentation
    pub documentation: Option<String>,
    /// The properties.
    pub properties: Vec<Property>,
    /// Whether this is a proposed structure. If omitted, the structure is final.
    #[serde(default)]
    pub proposed: bool,
    /// Since when (release number) this structure is available. Is undefined if not known.
    pub since: Option<String>,
}

/// Represents a literal structure (e.g. `property: { start: uinteger; end: uinteger; }`).
/// kind = "literal"
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StructureLiteralType {
    pub value: StructureLiteral,
}

/// Represents a `tuple` type (e.g. `[integer, integer]`).
/// kind = "tuple"
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TupleType {
    pub items: Vec<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum Type {
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
pub struct TypeAlias {
    /// An optional documentation.
    pub documentation: Option<String>,
    /// The name of the type alias.
    pub name: String,
    /// Whether this is a proposed type alias. If omitted, the type alias is final.
    #[serde(default)]
    pub proposed: bool,
    /// Since when (release number) this structure is available. Is undefined if not known.
    pub since: Option<String>,
    /// The aliased type.
    #[serde(rename = "type")]
    pub _type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TypeKind {
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
