use crate::schema::{
    AndType, ArrayType, BaseType, BaseTypes, BooleanLiteralType, IntegerLiteralType, MapKeyType,
    MapType, OrType, ReferenceType, StringLiteralType, StructureLiteralType, TupleType, Type,
    UriOrDocumentUriOrStringOrInteger,
};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use sha3::Digest;
use sha3::Sha3_224;

pub fn handle_type(
    random: &mut ChaCha20Rng,
    _type: &Type,
) -> syn::Result<(TokenStream, TokenStream)> {
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
            // TODO FIXME decide Boxed - this is still not optimal, only the parent place needs to me non-recursive
            if name == "r#SelectionRange" {
                Ok((quote! { Box<#name> }, quote! {}))
            } else {
                Ok((quote! { #name }, quote! {}))
            }
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
                    // TODO FIXME decide boxed
                    quote! { #name }
                }
            };
            Ok((
                quote! { ::std::collections::HashMap<#key_type, #value_type> },
                quote! { #value_rest },
            ))
        }
        Type::And(AndType { items: _ }) => Ok((quote! { () }, quote! {}))/*Err(Error::new(
            Span::call_site(),
            r#"we don't support and types yet"#,
        ))*/,
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
                    #[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq, Clone)]
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
                    #[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq, Clone)]
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

pub fn until_err<T, E>(err: &mut &mut Result<(), E>, item: Result<T, E>) -> Option<T> {
    match item {
        Ok(item) => Some(item),
        Err(e) => {
            **err = Err(e);
            None
        }
    }
}
