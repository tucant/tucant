use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned};

use crate::json_schema::{Definition, JSONSchema};

#[allow(clippy::too_many_lines)]
#[allow(clippy::wildcard_imports)] // false positive
#[must_use]
pub fn codegen_definition(name: &Ident, definition: &Definition) -> proc_macro2::TokenStream {
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
    let result = match &definition.definition_type {
        crate::json_schema::DefinitionType::AllOf(t) => {
            let (properties_code, member_names, member_types): (Vec<_>, Vec<_>, Vec<_>) = t
                .definitions
                .iter()
                .enumerate()
                .map(|(id, p)| {
                    let name =
                        format_ident!("r#{}Struct{}", name, id.to_string().to_upper_camel_case());
                    let key = format_ident!("r#o{}", id.to_string().to_snake_case());
                    (codegen_definition(&name, p), key, name)
                })
                .multiunzip();

            quote! {
                #(#properties_code)*

                #title
                #description
                pub struct #name {
                    #(pub #member_names: #member_types),*
                }
            }
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
                    (codegen_definition(&name, p), key, name)
                })
                .multiunzip();

            quote! {
                #(#properties_code)*

                #title
                #description
                pub enum #name {
                    #(#member_names(#member_types)),*
                }
            }
        }
        crate::json_schema::DefinitionType::Ref(t) => {
            let ref_name =
                format_ident!("r#{}", t.name.value().trim_start_matches("#/definitions/"));
            quote! {
                #title
                #description
                pub type #name = #ref_name;
            }
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
                    (codegen_definition(&name, &p.value.0), key, name)
                })
                .multiunzip();
            quote! {
                #(#properties_code)*

                #title
                #description
                pub struct #name {
                    #(pub #member_names: #member_types),*
                }
            }
        }
        crate::json_schema::DefinitionType::StringType(_t) => {
            quote! {
                #title
                #description
                pub type #name = String;
            }
        }
        crate::json_schema::DefinitionType::ArrayType(t) => {
            let array_name = format_ident!("r#{}Array", name);
            let code = codegen_definition(&array_name, &t.item_type);
            quote! {
                #code

                #title
                #description
                pub type #name = Vec<#array_name>;
            }
        }
        crate::json_schema::DefinitionType::IntegerType(_t) => {
            quote! {
                #title
                #description
                pub type #name = i32;
            }
        }
        crate::json_schema::DefinitionType::DoubleType(_t) => {
            quote! {
                #title
                #description
                pub type #name = f64;
            }
        }
        crate::json_schema::DefinitionType::BooleanType(_t) => {
            quote! {
                #title
                #description
                pub type #name = bool;
            }
        }
    };
    quote! {
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
