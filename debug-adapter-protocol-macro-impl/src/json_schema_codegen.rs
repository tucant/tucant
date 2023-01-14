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
        crate::json_schema::DefinitionType::AllOf(t) => {
            let (base, derived) = t.definitions.iter().collect_tuple().unwrap();

            // TODO FIXME "extends" def1

            let (code, ident) = codegen_definition(&format_ident!("{}", name), derived);

            (
                quote! {
                    #code
                },
                quote! { #name },
            )
        }
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
    // TODO FIXME all top level things are allOf

    let (definitions, _): (Vec<_>, Vec<_>) = schema
        .definitions
        .into_iter()
        .map(|definition| {
            match definition.value {
                Definition { definition_type: DefinitionType::AllOf(all_of), .. } => {
                    todo!()
                }
                _ => panic!(),
            }
            /*let name = format_ident!("r#{}", definition.key.value());
            codegen_definition(&name, &definition.value)*/
        })
        .unzip();
    quote! {
        #(#definitions)*
    }
}
