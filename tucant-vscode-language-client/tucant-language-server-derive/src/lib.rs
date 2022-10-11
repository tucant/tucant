use std::fs;

use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use syn::{parse::Nothing, parse_macro_input, Error, LitStr, token::Brace, Expr};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MetaModel {
    meta_data: Value,
    requests: Vec<Value>,
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
