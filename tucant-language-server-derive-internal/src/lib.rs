mod schema;
mod type_converter;

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
    random: &mut ChaCha20Rng,
    structures: &[Structure],
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut structures_err = Ok(());
    let return_type: (Vec<TokenStream>, Vec<TokenStream>) = structures
        .iter()
        .map(|structure| -> syn::Result<(TokenStream, TokenStream)> {
            let name = format_ident!("r#{}", structure.name.to_upper_camel_case());
            let documentation = structure.documentation.as_ref().map(|string| {
                quote! {
                    #[allow(clippy::doc_markdown)]
                    #[doc = #string]
                }
            });

            let mut extends_err = Ok(());
            let (extends, rest1): (Vec<TokenStream>, Vec<TokenStream>) = structure
                .extends
                .iter()
                .enumerate()
                .map(|(i, the_type)| -> syn::Result<(TokenStream, TokenStream)> {
                    // TODO FIXME would probably be nicer to assert this is a reference type and then use that name
                    let name = format_ident!("r#variant{}", i);
                    let (converted_type, rest) = handle_type(random, the_type)?;
                    Ok((
                        quote! {
                            #[serde(flatten)]
                            pub #name: #converted_type,
                        },
                        rest,
                    ))
                })
                .map_while(until_err(&mut extends_err))
                .unzip();

            // TODO FIXME merge this with above?
            let mut mixins_err = Ok(());
            let (mixins, rest2): (Vec<TokenStream>, Vec<TokenStream>) = structure
                .mixins
                .iter()
                .enumerate()
                .map(|(i, the_type)| -> syn::Result<(TokenStream, TokenStream)> {
                    // TODO FIXME would probably be nicer to assert this is a reference type and then use that name
                    let name = format_ident!("r#variant{}", structure.extends.len() + i);
                    let (converted_type, rest) = handle_type(random, the_type)?;
                    Ok((
                        quote! {
                            #[serde(flatten)]
                            pub #name: #converted_type,
                        },
                        rest,
                    ))
                })
                .map_while(until_err(&mut mixins_err))
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
                            #[allow(clippy::doc_markdown)]
                            #[doc = #string]
                        }
                    });
                    let (mut converted_type, rest) = handle_type(random, &property.r#type)?;

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
                .map_while(until_err(&mut properties_err))
                .unzip();

            let return_value = (
                quote! {
                    #[allow(clippy::derive_partial_eq_without_eq)]
                    #[::serde_with::skip_serializing_none]
                    #[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq, Clone)]
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
        .map_while(until_err(&mut structures_err))
        .unzip();
    structures_err?;
    Ok(return_type)
}

pub fn parse_enumerations(
    _random: &mut ChaCha20Rng,
    enumerations: &[Enumeration],
) -> syn::Result<Vec<TokenStream>> {
    let mut enumerations_err = Ok(());
    let enumerations = enumerations
        .iter()
        .map(|enumeration| -> syn::Result<TokenStream> {
            let name = format_ident!("r#{}", enumeration.name.to_upper_camel_case());
            let documentation = enumeration.documentation.as_ref().map(|string| {
                quote! {
                    #[allow(clippy::doc_markdown)]
                    #[doc = #string]
                }
            });
            match enumeration.r#type {
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
                        .map_while(until_err(&mut values_err));

                    let return_value = quote! {
                        #[allow(clippy::derive_partial_eq_without_eq)]
                        #[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, PartialEq, Clone)]
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
                                    #[allow(clippy::doc_markdown)]
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
                        .map_while(until_err(&mut values_err));

                    let supports_custom_value = if enumeration.supports_custom_values {
                        Some(quote! {
                            #[serde(other)]
                            Other,
                        })
                    } else {
                        None
                    };

                    let return_value = quote! {
                        #[allow(clippy::derive_partial_eq_without_eq)]
                        #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
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
        .map_while(until_err(&mut enumerations_err))
        .collect();
    enumerations_err?;
    Ok(enumerations)
}

pub fn parse_type_aliases(
    random: &mut ChaCha20Rng,
    type_aliases: &[TypeAlias],
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut type_aliases_err = Ok(());
    let return_value: (Vec<TokenStream>, Vec<TokenStream>) = type_aliases
        .iter()
        .map(|type_alias| -> syn::Result<(TokenStream, TokenStream)> {
            let name = format_ident!("r#{}", type_alias.name.to_upper_camel_case());
            let (converted_type, rest) = handle_type(random, &type_alias.the_type)?;
            let documentation = type_alias.documentation.as_ref().map(|string| {
                quote! {
                    #[allow(clippy::doc_markdown)]
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
        .map_while(until_err(&mut type_aliases_err))
        .unzip();
    type_aliases_err?;
    Ok(return_value)
}

pub fn parse_requests(
    random: &mut ChaCha20Rng,
    requests: &[Request],
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut requests_err = Ok(());
    let return_value: (Vec<TokenStream>, Vec<TokenStream>) = requests
        .iter()
        .map(|request| -> syn::Result<(TokenStream, TokenStream)> {
            let method = &request.method;
            let documentation = request.documentation.as_ref().map(|string| {
                quote! {
                    #[allow(clippy::doc_markdown)]
                    #[doc = #string]
                }
            });
            let request_name = format_ident!(
                "r#{}Request",
                method.replace('_', " ").to_upper_camel_case()
            );
            let (params, request_rest) = match &request.params {
                Some(TypeOrVecType::Type(the_type)) => {
                    let (the_type, rest) = handle_type(random, the_type)?;
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
                        .map(|the_type| -> syn::Result<(TokenStream, TokenStream)> {
                            handle_type(random, the_type)
                        })
                        .map_while(until_err(&mut params_err))
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
                None => (
                    quote! {
                        ()  // TODO FIXME this needs to be "nothing"
                    },
                    quote! {},
                ),
            };
            let response_name = format_ident!(
                "r#{}Response",
                method.replace('_', " ").to_upper_camel_case()
            );
            let (result_type, result_type_rest) = handle_type(random, &request.result)?;
            let (error_type, error_type_rest) = request
                .error_data
                .as_ref()
                .map(|e| -> syn::Result<(TokenStream, TokenStream)> {
                    let (error_type, rest) = handle_type(random, e)?;
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
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[::serde_with::skip_serializing_none]
                #[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq, Clone)]
                #documentation
                pub struct #response_name {
                    pub jsonrpc: String,
                    pub id: StringOrNumber,
                    pub result: Option<#result_type>,
                    #error_type
                }
            };
            let sendable_1 = if let MessageDirection::ServerToClient | MessageDirection::Both =
                request.message_direction
            {
                quote! {
                    impl Sendable for #request_name {
                        type Request = #params;

                        type Response = #result_type;

                        // sometimes () type
                        #[allow(clippy::semicolon_if_nothing_returned)]
                        fn get_request_data(self) -> Self::Request {
                            self.params
                        }

                        fn name() -> String {
                            #method.to_string()
                        }

                        fn id(&self) -> &StringOrNumber {
                            &self.id
                        }
                    }
                }
            } else {
                quote! {}
            };
            let receivable_1 = if let MessageDirection::ClientToServer | MessageDirection::Both =
                request.message_direction
            {
                quote! {
                    impl Receivable for #request_name {
                        type Request = #params;

                        type Response = #result_type;

                        // sometimes () type
                        #[allow(clippy::semicolon_if_nothing_returned)]
                        fn get_request_data(self) -> Self::Request {
                            self.params
                        }

                        fn name() -> String {
                            #method.to_string()
                        }

                        fn id(&self) -> &StringOrNumber {
                            &self.id
                        }
                    }
                }
            } else {
                quote! {}
            };
            let request_struct = quote! {
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[::serde_with::skip_serializing_none]
                #[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq, Clone)]
                #documentation
                pub struct #request_name {
                    pub jsonrpc: String,
                    pub id: StringOrNumber,
                    pub params: #params
                }

                #sendable_1
                #receivable_1
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
        .map_while(until_err(&mut requests_err))
        .unzip();
    requests_err?;
    Ok(return_value)
}

pub fn parse_notifications(
    random: &mut ChaCha20Rng,
    notifications: &[Notification],
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut requests_err = Ok(());
    let return_type: (Vec<TokenStream>, Vec<TokenStream>) = notifications
        .iter()
        .map(|notification| -> syn::Result<(TokenStream, TokenStream)> {
            let documentation = notification.documentation.as_ref().map(|string| {
                quote! {
                    #[allow(clippy::doc_markdown)]
                    #[doc = #string]
                }
            });
            let method = &notification.method;
            let name = format_ident!(
                "r#{}Notification",
                method.replace('_', " ").to_upper_camel_case()
            );
            let (params, rest) = match &notification.params {
                Some(TypeOrVecType::Type(the_type)) => handle_type(random, the_type)?,
                Some(TypeOrVecType::VecType(vec_type)) => {
                    let mut params_err = Ok(());
                    let (types, rest): (Vec<TokenStream>, Vec<TokenStream>) = vec_type
                        .iter()
                        .map(|the_type| -> syn::Result<(TokenStream, TokenStream)> {
                            handle_type(random, the_type)
                        })
                        .map_while(until_err(&mut params_err))
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
                None => (quote! { () }, quote! {}), // TODO FIXME this needs to be "nothing"
            };
            let sendable_2 = if let MessageDirection::ServerToClient | MessageDirection::Both =
                notification.message_direction
            {
                quote! {
                    impl SendableAndForget for #name {
                        type Request = #params;

                        // sometimes () type
                        #[allow(clippy::semicolon_if_nothing_returned)]
                        fn get_request_data(self) -> Self::Request {
                            self.params
                        }

                        fn name() -> String {
                            #method.to_string()
                        }
                    }
                }
            } else {
                quote! {}
            };
            Ok((
                quote! {
                    #[allow(clippy::derive_partial_eq_without_eq)]
                    #[::serde_with::skip_serializing_none]
                    #[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq, Clone)]
                    #documentation
                    pub struct #name {
                        pub jsonrpc: String,
                        pub params: #params
                    }

                    #sendable_2
                },
                rest,
            ))
        })
        .map_while(until_err(&mut requests_err))
        .multiunzip();
    Ok(return_type)
}

// a totally different approach which would give us line number information would be to have a magic!{} macro inside which the json is *not* inside a string. so the json would be parsed into actual tokens. possibly this could be done with serde.
pub fn handle_magic() -> syn::Result<TokenStream> {
    //let file = fs::File::open("metaModel.json").expect("file should open read only");
    let meta_model_json = include_str!("./metaModel.json");
    let meta_model: MetaModel =
        serde_json::from_str(meta_model_json).expect("file should be proper JSON");

    let mut random = ChaCha20Rng::seed_from_u64(42);

    let (structures, rest_structures) = parse_structures(&mut random, &meta_model.structures)?;

    let enumerations = parse_enumerations(&mut random, &meta_model.enumerations)?;

    let (type_aliases, rest_type_aliases) =
        parse_type_aliases(&mut random, &meta_model.type_aliases)?;

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
            let _response_name = format_ident!(
                "r#{}Response",
                request.method.replace('_', " ").to_upper_camel_case()
            );

            request_enum
        })
        .collect();

    let (notifications, notifications_rest) =
        parse_notifications(&mut random, &meta_model.notifications)?;

    let client_to_server_enum: Vec<TokenStream> = meta_model
        .notifications
        .iter()
        .map(|notification| -> TokenStream {
            let method = &notification.method;
            let name = format_ident!(
                "r#{}Notification",
                notification.method.replace('_', " ").to_upper_camel_case()
            );

            if let MessageDirection::ClientToServer | MessageDirection::Both =
                notification.message_direction
            {
                quote! {
                    #[serde(rename = #method)]
                    #name(#name),
                }
            } else {
                quote! {}
            }
        })
        .collect();

    Ok(quote! {
        #(#structures)*
        #(#enumerations)*
        #(#type_aliases)*
        #(#rest_structures)*
        #(#rest_type_aliases)*
        #(#requests)*
        #(#requests_rest)*
        #(#notifications)*
        #(#notifications_rest)*

        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq, Clone)]
        #[serde(untagged)]
        pub enum StringOrNumber {
            String(String),
            Number(i64),
        }

        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(::serde::Serialize, ::serde::Deserialize, Debug, PartialEq, Clone)]
        #[serde(tag = "method")]
        pub enum IncomingStuff {
            #(#request_enum)*
            #(#client_to_server_enum)*
        }

        use serde::{Deserialize, Serialize};

        pub trait Sendable {
            type Request: ::serde::Serialize + std::marker::Send;
            type Response: ::core::any::Any + Send + Sync + ::serde::Serialize + ::serde::de::DeserializeOwned + 'static;

            fn get_request_data(self) -> Self::Request;

            fn name() -> String; // TODO 'static str

            fn id(&self) -> &StringOrNumber;
        }

        pub trait SendableAndForget {
            type Request: ::serde::Serialize + std::marker::Send;

            fn get_request_data(self) -> Self::Request;

            fn name() -> String; // TODO 'static str
        }

        pub trait Receivable {
            type Request: ::serde::Serialize;
            type Response: ::core::any::Any + Send + Sync + ::serde::Serialize + ::serde::de::DeserializeOwned + 'static;

            fn get_request_data(self) -> Self::Request;

            fn name() -> String;

            fn id(&self) -> &StringOrNumber;
        }

        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
    })
}
