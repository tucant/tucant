use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};

use crate::{
    json_parser::KeyValue,
    json_schema::{Definition, JSONSchema},
};

pub fn codegen_definition(name: &Ident, definition: &Definition) -> proc_macro2::TokenStream {
    let description = definition.description.as_ref().map(|d| {
        quote! {
            #[doc = #d]
        }
    });
    let title = definition.title.as_ref().map(|t| {
        quote! {
            #[doc = #t]
        }
    });
    let result = match &definition.definition_type {
        crate::json_schema::DefinitionType::AllOf(t) => {
            quote! {
                pub struct #name {

                }
            }
        }
        crate::json_schema::DefinitionType::OneOf(t) => {
            quote! {
                pub struct #name {

                }
            }
        }
        crate::json_schema::DefinitionType::Ref(t) => {
            quote! {
                pub struct #name {

                }
            }
        }
        crate::json_schema::DefinitionType::ObjectType(t) => {
            let (properties_code, member_names, member_types): (Vec<_>, Vec<_>, Vec<_>) = t
                .properties
                .iter()
                .map(|p| {
                    let name = format_ident!("r#{}_{}", name, p.key.value());
                    let key = format_ident!("r#{}", p.key.value());
                    (codegen_definition(&name, &p.value.0), key, name)
                })
                .multiunzip();
            quote! {
                #(#properties_code)*

                pub struct #name {
                    #(#member_names: #member_types),*
                }
            }
        }
        crate::json_schema::DefinitionType::StringType(t) => {
            quote! {
                pub struct #name {

                }
            }
        }
        crate::json_schema::DefinitionType::ArrayType(t) => {
            let name = format_ident!("r#{}_array", name);
            let code = codegen_definition(&name, &t.item_type);
            quote! {
                #code
                type #name = Vec<#name>;
            }
        }
        crate::json_schema::DefinitionType::IntegerType(t) => {
            quote! {
                type #name = i32;
            }
        }
        crate::json_schema::DefinitionType::DoubleType(t) => {
            quote! {
                type #name = f64;
            }
        }
        crate::json_schema::DefinitionType::BooleanType(t) => {
            quote! {
                type #name = bool;
            }
        }
    };
    quote! {
        #title
        #description
        #result
    }
}

#[allow(clippy::wildcard_imports)] // false positive
#[must_use]
pub fn codegen(schema: JSONSchema) -> proc_macro2::TokenStream {
    let definitions = schema.definitions.into_iter().map(|definition| {
        let name = format_ident!("r#{}", definition.key.value());
        codegen_definition(&name, &definition.value)
    });
    quote! {
        #(#definitions)*
    }
}
