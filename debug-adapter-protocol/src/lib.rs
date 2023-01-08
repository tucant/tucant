use syn::punctuated::Punctuated;
use syn::token::Colon;
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parse().unwrap()
    }
}
