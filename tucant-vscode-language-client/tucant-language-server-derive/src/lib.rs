use std::fs;

use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use syn::{parse::Nothing, parse_macro_input, Error, LitStr, token::Brace, Expr};

// this is manually extracted from the metaModel.schema.json (but we should probably generate this at some point)
// well it contains insufficient information e.g. no default for proposed so we need to probably do this manually
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MetaModel {
    meta_data: Value,
    requests: Vec<Request>,
    notifications: Vec<Value>,
    structures: Vec<Value>,
    enumerations: Vec<Value>,
    type_aliases: Vec<Value>
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
