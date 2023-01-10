use proc_macro2::Span;
use syn::{
    braced, bracketed, parse::Parse, punctuated::Punctuated, token, LitBool, LitInt, LitStr, Token,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum JSONValue {
    Bool(LitBool),
    String(LitStr),
    Integer(LitInt),
    Array((token::Bracket, Punctuated<JSONValue, token::Comma>)),
    Object((token::Brace, Punctuated<KeyValue, token::Comma>)),
}

impl JSONValue {
    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            Self::Bool(value) => value.span(),
            Self::String(value) => value.span(),
            Self::Integer(value) => value.span(),
            Self::Array((value, _)) => value.span,
            Self::Object((value, _)) => value.span,
        }
    }
}

impl Parse for JSONValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Brace) {
            let content;
            let brace = braced!(content in input);
            Ok(Self::Object((
                brace,
                content.parse_terminated(KeyValue::parse)?,
            )))
        } else if lookahead.peek(token::Bracket) {
            let content;
            let bracket = bracketed!(content in input);
            Ok(Self::Array((
                bracket,
                content.parse_terminated(Self::parse)?,
            )))
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::String)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Self::Integer)
        } else if lookahead.peek(LitBool) {
            input.parse().map(Self::Bool)
        } else {
            Err(lookahead.error())
        }
    }
}

impl TryFrom<JSONValue> for LitStr {
    type Error = syn::Error;

    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        if let JSONValue::String(value) = value {
            Ok(value)
        } else {
            Err(syn::Error::new(value.span(), "Expected string"))
        }
    }
}

impl TryFrom<JSONValue> for (token::Brace, Punctuated<KeyValue, token::Comma>) {
    type Error = syn::Error;

    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        if let JSONValue::Object(value) = value {
            Ok(value)
        } else {
            Err(syn::Error::new(value.span(), "Expected object"))
        }
    }
}

impl TryFrom<JSONValue> for (token::Bracket, Punctuated<JSONValue, token::Comma>) {
    type Error = syn::Error;

    fn try_from(value: JSONValue) -> Result<Self, Self::Error> {
        if let JSONValue::Array(value) = value {
            Ok(value)
        } else {
            Err(syn::Error::new(value.span(), "Expected array"))
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct KeyValue {
    pub key: LitStr,
    colon: Token![:],
    pub value: JSONValue,
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
