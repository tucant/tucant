use std::collections::BTreeMap;

use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned};

use crate::json_schema::{Definition, JSONSchema, DefinitionType};

#[allow(clippy::too_many_lines)]
#[allow(clippy::wildcard_imports)] // false positive
#[must_use]
// first return value is the code and second return value is the type identifier
pub fn codegen_definition(
    name: &Ident,
    definition: &Definition,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let description = definition.description.as_ref().map(|d| {
        quote! {
            #[allow(clippy::doc_markdown)]
            #[doc = #d]
        }
    });
    let title = definition.title.as_ref().map(|t| {
        quote! {
            #[allow(clippy::doc_markdown)]
            #[doc = #t]
        }
    });
    match &definition.definition_type {
        crate::json_schema::DefinitionType::AllOf(_) => unreachable!(),
        crate::json_schema::DefinitionType::OneOf(t) => {
            let (properties_code, member_names, member_types): (Vec<_>, Vec<_>, Vec<_>) = t
                .definitions
                .iter()
                .enumerate()
                .map(|(id, p)| {
                    let name =
                        format_ident!("r#{}Enum{}", name, id.to_string().to_upper_camel_case());
                    let key = format_ident!("r#O{}", id);
                    let code = codegen_definition(&name, p);
                    (code.0, key, code.1)
                })
                .multiunzip();

            (
                quote! {
                    #(#properties_code)*

                    #title
                    #description
                    pub enum #name {
                        #(#member_names(#member_types)),*
                    }
                },
                quote! { #name },
            )
        }
        crate::json_schema::DefinitionType::Ref(t) => {
            let ref_name =
                format_ident!("r#{}", t.name.value().trim_start_matches("#/definitions/"));
            (
                quote! {
                    #title
                    #description
                    // sometimes needed for the $ref types
                    pub type #name = #ref_name;
                },
                quote! { #ref_name },
            )
        }
        crate::json_schema::DefinitionType::ObjectType(t) => {
            let (properties_code, member_names, member_types): (Vec<_>, Vec<_>, Vec<_>) = t
                .properties
                .iter()
                .map(|p| {
                    let name =
                        format_ident!("r#{}Struct{}", name, p.key.value().to_upper_camel_case());
                    let key = format_ident!("r#{}", p.key.value().to_snake_case());
                    let key = quote_spanned! {p.key.span()=> #key};
                    let code = codegen_definition(&name, &p.value.0);
                    (code.0, key, code.1)
                })
                .multiunzip();
            (
                quote! {
                    #(#properties_code)*

                    #title
                    #description
                    pub struct #name {
                        #(pub #member_names: #member_types),*
                    }
                },
                quote! { #name },
            )
        }
        crate::json_schema::DefinitionType::StringType(_t) => {
            (
                quote! {
                    #title
                    #description
                    // sometimes needed for the $ref types
                    pub type #name = String;
                },
                quote! { String },
            )
        }
        crate::json_schema::DefinitionType::ArrayType(t) => {
            let array_name = format_ident!("r#{}Array", name);
            let (code, ident) = codegen_definition(&array_name, &t.item_type);
            (
                quote! {
                    #code

                    #title
                    #description
                    // sometimes needed for the $ref types
                    pub type #name = Vec<#ident>;
                },
                quote! { Vec<#ident> },
            )
        }
        crate::json_schema::DefinitionType::IntegerType(_t) => {
            (
                quote! {
                    #title
                    #description
                    // sometimes needed for the $ref types
                    pub type #name = i32;
                },
                quote! { i32 },
            )
        }
        crate::json_schema::DefinitionType::DoubleType(_t) => {
            (
                quote! {
                    #title
                    #description
                    // sometimes needed for the $ref types
                    pub type #name = f64;
                },
                quote! { f64 },
            )
        }
        crate::json_schema::DefinitionType::BooleanType(_t) => {
            (
                quote! {
                    #title
                    #description
                    // sometimes needed for the $ref types
                    pub type #name = bool;
                },
                quote! { bool },
            )
        }
    }
}

#[allow(clippy::wildcard_imports)] // false positive
#[must_use]
pub fn codegen(schema: JSONSchema) -> proc_macro2::TokenStream {
    let mut definitions = schema
        .definitions
        .into_iter()
        .map(|definition| {
            match definition.value {
                Definition { definition_type: DefinitionType::AllOf(all_of), .. } => {
                    let (base, derived) = all_of.definitions.iter().collect_tuple().unwrap();

                    let base_class = match base {
                        Definition { definition_type: DefinitionType::Ref(def), .. } => {
                            def.name.value().trim_start_matches("#/definitions/").to_string()
                        }
                        _ => panic!()
                    };
        
                    let code = codegen_definition(&format_ident!("{}", definition.key.value()), derived);
        
                    (Some(base_class), code)
                }
                _ => {
                    let name = format_ident!("r#{}", definition.key.value());
                    (None, codegen_definition(&name, &definition.value))
                },
            }
        }).fold(BTreeMap::<Option<String>, Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream)>>::new(), |mut acc, (a, b)| {
            acc.entry(a).or_default().push(b);
            acc
        });

    let free_definitions = definitions.remove(&None).unwrap_or_default();
    let event_definitions = definitions.remove(&Some("Event".to_string())).unwrap_or_default();
    let request_definitions = definitions.remove(&Some("Request".to_string())).unwrap_or_default();
    let response_definitions = definitions.remove(&Some("Response".to_string())).unwrap_or_default();
    let protocol_message_definitions = definitions.remove(&Some("ProtocolMessage".to_string())).unwrap_or_default();
    assert_eq!(protocol_message_definitions.len(), 3);
    assert_eq!(definitions.keys().len(), 0);

    let free_definitions_code = free_definitions.unzip();

    quote! {
        #(#free_definitions)*
        #(#event_definitions)*
        #(#request_definitions)*
        #(#response_definitions)*
    }
}
