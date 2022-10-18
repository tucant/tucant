mod schema;
mod type_converter;

use std::fs;

use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use schema::{
    Enumeration, EnumerationType, MessageDirection, MetaModel, Notification, Request,
    StringOrIntegerOrUnsignedIntegerLiteral, Structure, TypeAlias, TypeOrVecType,
};

use type_converter::{handle_type, until_err};

pub fn parse_structures(
    mut random: &mut ChaCha20Rng,
    structures: Vec<Structure>,
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut structures_err = Ok(());
    let return_type: (Vec<TokenStream>, Vec<TokenStream>) = structures
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

            // TODO FIXME merge this with above?
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
    structures_err?;
    Ok(return_type)
}

pub fn parse_enumerations(
    mut random: &mut ChaCha20Rng,
    enumerations: Vec<Enumeration>,
) -> syn::Result<Vec<TokenStream>> {
    let mut enumerations_err = Ok(());
    let enumerations = enumerations
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
        .scan(&mut enumerations_err, until_err)
        .collect();
    enumerations_err?;
    Ok(enumerations)
}

pub fn parse_type_aliases(
    mut random: &mut ChaCha20Rng,
    type_aliases: Vec<TypeAlias>,
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut type_aliases_err = Ok(());
    let return_value: (Vec<TokenStream>, Vec<TokenStream>) = type_aliases
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
    type_aliases_err?;
    Ok(return_value)
}

pub fn parse_requests(
    mut random: &mut ChaCha20Rng,
    requests: &Vec<Request>,
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut requests_err = Ok(());
    let return_value: (Vec<TokenStream>, Vec<TokenStream>) = requests
        .iter()
        .map(|request| -> syn::Result<(TokenStream, TokenStream)> {
            let method = request.method;
            let documentation = request.documentation.as_ref().map(|string| {
                quote! {
                    #[doc = #string]
                }
            });
            let name = format_ident!(
                "r#{}Request",
                method.replace('_', " ").to_upper_camel_case()
            );
            let (params, request_rest) = match &request.params {
                Some(TypeOrVecType::Type(_type)) => {
                    let (the_type, rest) = handle_type(&mut random, _type)?;
                    (
                        quote! {
                            #the_type
                        },
                        rest,
                    )
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
                            (#(#types)*)
                        },
                        quote! { #(#rest)* },
                    );
                    params_err?;
                    return_value
                }
                None => (quote! {
                    ()
                }, quote! {}),
            };

            let response_enum_1 = if let MessageDirection::ServerToClient | MessageDirection::Both =
                request.message_direction
            {
                quote! {
                    #[serde(rename = #method)]
                    #request_name(#request_name),
                }
            } else {
                quote! {}
            };
            let request_struct = quote! {
                #[::serde_with::skip_serializing_none]
                #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
                #documentation
                pub struct #name {
                    pub jsonrpc: String,
                    pub id: StringOrNumber,
                    pub params: #params
                }

                impl Requestable for #name {
                    type Request = #params;

                    type Response = ();

                    fn get_request_data(self) -> Self::Request {
                        self.params
                    }
                }
            };
            let response_name = format_ident!(
                "r#{}Response",
                method.replace('_', " ").to_upper_camel_case()
            );
            let (result_type, result_type_rest) = handle_type(&mut random, &request.result)?;
            let (error_type, error_type_rest) = request
                .error_data
                .as_ref()
                .map(|e| -> syn::Result<(TokenStream, TokenStream)> {
                    let (error_type, rest) = handle_type(&mut random, e)?;
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
                pub struct #response_name {
                    pub jsonrpc: String,
                    pub id: StringOrNumber,
                    pub result: Option<#result_type>,
                    #error_type
                }
            };
            let response_enum_2 = if let MessageDirection::ServerToClient | MessageDirection::Both =
                request.message_direction
            {
                quote! {
                    #[serde(rename = #method)]
                    #response_name(#response_name),
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
            ))
        })
        .scan(&mut requests_err, until_err)
        .unzip();
    requests_err?;
    Ok(return_value)
}

pub fn parse_notifications(
    mut random: &mut ChaCha20Rng,
    notifications: &Vec<Notification>,
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut requests_err = Ok(());
    let return_type: (Vec<TokenStream>, Vec<TokenStream>) = notifications
        .iter()
        .map(|notification| -> syn::Result<(TokenStream, TokenStream)> {
            let documentation = notification.documentation.as_ref().map(|string| {
                quote! {
                    #[doc = #string]
                }
            });
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
            if let MessageDirection::ServerToClient
            | MessageDirection::Both =
                notification.message_direction {
                // sendable
            }
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
            ))
        })
        .scan(&mut requests_err, until_err)
        .multiunzip();
    Ok(return_type)
}

// a totally different approach which would give us line number information would be to have a magic!{} macro inside which the json is *not* inside a string. so the json would be parsed into actual tokens. possibly this could be done with serde.
pub fn handle_magic() -> syn::Result<TokenStream> {
    let file = fs::File::open("src/metaModel.json").expect("file should open read only");
    let meta_model: MetaModel = serde_json::from_reader(file).expect("file should be proper JSON");

    let mut random = ChaCha20Rng::seed_from_u64(42);

    let (structures, rest_structures) = parse_structures(&mut random, meta_model.structures)?;

    let enumerations = parse_enumerations(&mut random, meta_model.enumerations)?;

    let (type_aliases, rest_type_aliases) =
        parse_type_aliases(&mut random, meta_model.type_aliases)?;

    let (requests, requests_rest) = parse_requests(&mut random, &meta_model.requests)?;

    let request_enum: Vec<TokenStream> = meta_model
        .requests
        .iter()
        .map(|request| -> TokenStream {
            let method = &request.method;
            let request_name = format_ident!(
                "r#{}Request",
                request.method.replace('_', " ").to_upper_camel_case()
            );
            let request_enum = if let MessageDirection::ClientToServer | MessageDirection::Both =
                request.message_direction
            {
                quote! {
                    #[serde(rename = #method)]
                    #request_name(#request_name),
                }
            } else {
                quote! {}
            };
            let response_name = format_ident!(
                "r#{}Response",
                request.method.replace('_', " ").to_upper_camel_case()
            );
            
                request_enum
              
        }).collect();

    let (notifications, notifications_rest) =
        parse_notifications(&mut random, &meta_model.notifications)?;

    let client_to_server_enum: 
        Vec<TokenStream>
     = meta_model
        .notifications
        .iter()
        .map(|notification| -> TokenStream {
            let method = &notification.method;
            let name = format_ident!(
                "r#{}Notification",
                notification.method.replace('_', " ").to_upper_camel_case()
            );
            let client_to_server_notification = if let MessageDirection::ClientToServer
            | MessageDirection::Both =
                notification.message_direction
            {
                quote! {
                    #[serde(rename = #method)]
                    #name(#name),
                }
            } else {
                quote! {}
            };
            client_to_server_notification
        })
        .collect();

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

        use serde::{Deserialize, Serialize};
                
        pub trait Sendable {
            type Request: ::serde::Serialize;
            type Response: ::core::any::Any + Send + Sync + ::serde::Serialize + 'static;

            fn get_request_data(self) -> Self::Request;
        }

        /*
        impl<Req, Res> Request<Req, Res> {
            pub fn new(value: Req) -> Self {
                
                Self {
                    id: rand_string,
                    params: value,
                    phantom_data: ::core::marker::PhantomData,
                }
            }

            pub async fn respond(&self, handler: ::std::sync::Arc<Server>, value: Res) {
                
            }
        }
        */

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Notification<T> {
            pub params: T,
        }

        impl<T> Notification<T> {
            pub fn new(value: T) -> Self {
                Self {
                    params: value
                }
            }
        }
    });
    return_value
}
