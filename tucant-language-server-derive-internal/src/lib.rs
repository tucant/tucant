mod schema;
mod type_converter;

use std::fs;

use derive_more::TryInto;
use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use schema::{MetaModel, BaseType, Type, BaseTypes, UriOrDocumentUriOrStringOrInteger, MapKeyType, ReferenceType, StructureLiteralType, StringOrIntegerOrUnsignedIntegerLiteral, EnumerationType, TypeOrVecType, MessageDirection, TupleType, ArrayType, MapType, AndType, OrType, IntegerLiteralType, StringLiteralType, BooleanLiteralType};
use serde::{Deserialize, Serialize};

use sha3::{Digest, Sha3_224};
use syn::{parse::Nothing, parse_macro_input, Error};
use type_converter::{until_err, handle_type};

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
                let method = &request.method;
                let name = format_ident!(
                    "r#{}Request",
                    request.method.replace('_', " ").to_upper_camel_case()
                );
                let (params, request_rest) = match &request.params {
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
                let request_struct = quote! {
                    #[::serde_with::skip_serializing_none]
                    #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
                    #documentation
                    pub struct #name {
                        pub jsonrpc: String,
                        pub id: StringOrNumber,
                        #params
                    }
                };
                let request_enum =
                if let MessageDirection::ClientToServer | MessageDirection::Both = request.message_direction
                {
                    quote! {
                        #[serde(rename = #method)]
                        #name(#name),
                    }
                } else {
                    quote! {}
                };
                let response_enum_1 =
                    if let
                    | MessageDirection::ServerToClient
                    | MessageDirection::Both = request.message_direction
                    {
                        quote! {
                            #[serde(rename = #method)]
                            #name(#name),
                        }
                    } else {
                        quote! {}
                    };
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
                let response_struct = quote! {
                    #[::serde_with::skip_serializing_none]
                    #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
                    #documentation
                    pub struct #name {
                        pub jsonrpc: String,
                        pub id: StringOrNumber,
                        pub result: Option<#result_type>,
                        #error_type
                    }
                };
                let response_enum_2 =
                    if let
                    | MessageDirection::ServerToClient
                    | MessageDirection::Both = request.message_direction
                    {
                        quote! {
                            #[serde(rename = #method)]
                            #name(#name),
                        }
                    } else {
                        quote! {}
                    };
                Ok((
                    quote! {
                        #request_struct
                        #response_struct
                    },
                    quote! {
                        #request_rest
                        #result_type_rest
                        #error_type_rest
                    },
                    request_enum,
                    quote! {
                        #response_enum_1
                        #response_enum_2
                    },
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
        pub enum IncomingStuff {
            #(#request_enum)*
            #(#client_to_server_enum)*
        }

        #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
        #[serde(tag = "method")]
        pub enum Responses {
            #(#server_to_client_notification)*
            #(#response_enum)*
        }

        pub trait MagicServer {


        }
    });
    structures_err?;
    enumerations_err?;
    type_aliases_err?;
    requests_err?;
    return_value
}
