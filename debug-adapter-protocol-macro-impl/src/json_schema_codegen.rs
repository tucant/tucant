use quote::{format_ident, quote, quote_spanned};

use crate::json_schema::JSONSchema;

#[allow(clippy::wildcard_imports)] // false positive
#[must_use]
pub fn codegen(schema: JSONSchema) -> proc_macro2::TokenStream {
    let definitions = schema.definitions.into_iter().map(|definition| {
        let name = format_ident!("r#{}", definition.key.value());
        let name = quote_spanned!(definition.key.span() => #name);
        let description = definition.value.description.map(|d| quote! {
            #[doc = #d]
        });
        let title = definition.value.title.map(|t| quote! {
            #[doc = #t]
        });
        let result = match definition.value.definition_type {
            crate::json_schema::DefinitionType::AllOf(t) => {

                quote! {
                    pub struct #name {

                    }
                }
            },
            crate::json_schema::DefinitionType::OneOf(t) => {

                quote! {
                    pub struct #name {

                    }
                }
            },
            crate::json_schema::DefinitionType::Ref(t) => {

                quote! {
                    pub struct #name {

                    }
                }
            },
            crate::json_schema::DefinitionType::ObjectType(t) => {
                let properties = t.properties.into_iter().map(|p| {
                    let name = format_ident!("r#{}", p.key.value());
                    let name = quote_spanned!(p.key.span() => #name);
                    quote! {
                        #name: ()
                    }
                });
                quote! {
                    pub struct #name {
                        #(#properties),*
                    }
                }
            },
            crate::json_schema::DefinitionType::StringType(t) => {

                quote! {
                    pub struct #name {

                    }
                }
            },
            crate::json_schema::DefinitionType::ArrayType(t) => {

                quote! {
                    pub struct #name {

                    }
                }
            },
            crate::json_schema::DefinitionType::IntegerType(t) => {

                quote! {
                    pub struct #name {

                    }
                }
            },
            crate::json_schema::DefinitionType::DoubleType(t) => {

                quote! {
                    pub struct #name {

                    }
                }
            },
            crate::json_schema::DefinitionType::BooleanType(t) => {
                quote! {
                    type #name = bool
                }
            },
        };
        quote! {
            #title
            #description
            #result
        }
    });
    quote! {
        #(#definitions)*
    }
}
