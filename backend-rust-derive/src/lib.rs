#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse::Nothing, parse_macro_input, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput,
    Error, ItemFn, Pat, PatIdent, PatType, TypeParam,
};

// RUSTFLAGS="-Z macro-backtrace" cargo test

fn handle_item_fn(node: &ItemFn) -> syn::Result<TokenStream> {
    let return_type = match node.sig.output {
        syn::ReturnType::Default => {
            return Err(Error::new(
                node.sig.output.span(),
                r#"unexpected return type"#,
            ))
        }
        syn::ReturnType::Type(_, ref path) => path.to_token_stream(),
    };

    let arg_type = node.sig.inputs.iter().find_map(|arg| match arg {
        syn::FnArg::Receiver(_) => None,
        syn::FnArg::Typed(PatType { pat, ty, .. }) => {
            if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                if *ident == "input" || *ident == "_input" {
                    return Some(ty.to_token_stream());
                }
            }
            None
        }
    });

    arg_type.map_or_else(|| Err(Error::new(
            node.sig.inputs.span(),
            r#"name one of the parameters `input` or `_input`"#,
        )), |arg_type| {
        let name = &node.sig.ident;

        let (impl_generics, ty_generics, where_clause) = node.sig.generics.split_for_impl();

        let name_string = node.sig.ident.to_string();

        let name_ts = format_ident!("{}Ts", name.to_string().to_upper_camel_case());

        let typescriptable_arg_type_name = quote_spanned! {arg_type.span()=>
            <#arg_type as tucant_derive_lib::Typescriptable>::name()
        };

        let typescriptable_arg_type_code = quote_spanned! {arg_type.span()=>
            <#arg_type as tucant_derive_lib::Typescriptable>::code()
        };

        let typescriptable_return_type_name = quote_spanned! {return_type.span()=>
            <#return_type as tucant_derive_lib::Typescriptable>::name()
        };

        let typescriptable_return_type_code = quote_spanned! {return_type.span()=>
            <#return_type as tucant_derive_lib::Typescriptable>::code()
        };

        Ok(quote! {
            #node

            pub struct #name_ts;

            impl #impl_generics tucant_derive_lib::TypescriptRoute for #name_ts #ty_generics #where_clause {

                fn code(path: &str) -> ::std::collections::BTreeSet<String> {
                    let mut result = ::std::collections::BTreeSet::from(["export async function ".to_string() + #name_string + "(input: " + &#typescriptable_arg_type_name + ")"
                    + ": Promise<" + &#typescriptable_return_type_name + "> {" +
                    r#"
        return await genericFetch("http://localhost:8080"# + path + r#"", input) as "# + &#typescriptable_return_type_name +
        "\n}"]);
                    result.extend(#typescriptable_arg_type_code);
                    result.extend(#typescriptable_return_type_code);
                    result
                }
            }
        })
    })
}

#[allow(clippy::too_many_lines)]
fn typescriptable_impl(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let name_string = input.ident.to_string();

    let (members, members_code) = match &input.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            syn::Fields::Named(fields_named) => {
                let (members, members_code) = fields_named
                .named
                .iter()
                .map(|field| {
                    let ident_string = field.ident.as_ref().unwrap().to_string();

                    let ts_type_attr = field.attrs.iter().find(|attr| {
                        attr.path().is_ident("ts_type")
                    });

                    let serde_attr = field.attrs.iter().find(|attr| {
                        attr.path().is_ident("serde")
                    });

                    // #[ts_type(String)]
                    let field_type = if let Some(ts_type_attr) = ts_type_attr {
                        let mut the_type = None;
                        ts_type_attr.parse_nested_meta(|meta| {
                            the_type = Some(meta.path.to_token_stream());
                            Ok(())
                        })?;
                        if let Some(the_type) = the_type {
                            the_type
                        } else {
                            return Err(Error::new(serde_attr.span(), r#"`ts_type` attribute macro expects list with type"#))
                        }
                    } else {
                        // #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
                        if let Some(serde_attr) = serde_attr {
                            let mut has_de_serialize_with = false;
                            serde_attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("serialize_with") || meta.path.is_ident("deserialize_with") {
                                    has_de_serialize_with = true;
                                }
                                Ok(())
                            })?;
                            if has_de_serialize_with {
                                return Err(Error::new(serde_attr.span(), r#"`serde` attribute macro `serialize_with` or `deserialize_with` requires `ts_type` attribute macro to clarify type"#))
                            }
                        }

                        field.ty.to_token_stream()
                    };

                    let typescriptable_field_type_name = quote_spanned! {field_type.span()=>
                        <#field_type as tucant_derive_lib::Typescriptable>::name()
                    };

                    let typescriptable_field_type_code = quote_spanned! {field_type.span()=>
                        <#field_type as tucant_derive_lib::Typescriptable>::code()
                    };

                    Ok((
                        quote! {
                           "  " + #ident_string + ": " + &#typescriptable_field_type_name + ",\n"
                        },
                        quote! {
                            result.extend(#typescriptable_field_type_code);
                        },
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .fold((quote! {}, quote! {}), |(accx, accy), (x, y)| {
                    (
                        quote! {
                            #accx + #x
                        },
                        quote! {
                            #accy
                            #y
                        },
                    )
                });
                (quote! {
                    + "{\n"
                    #members
                    + "}"
                }, members_code)
            },
            err => {
                return Err(Error::new(
                    err.span(),
                    r#"only structs with named fields supported"#,
                ))
            }
        },
        Data::Enum(DataEnum { variants, .. }) => variants
            .iter()
            .map(|variant| match &variant.fields {
                syn::Fields::Named(err) => Err(Error::new(
                    err.span(),
                    r#"only enums with unnamed fields allowed"#,
                )),
                syn::Fields::Unnamed(fields) => {
                    let mut iter = fields.unnamed.iter();
                    iter.next().map_or_else(|| Err(Error::new(
                            variant.fields.span(),
                            r#"exactly one field in enum allowed"#,
                        )), |field| iter.next().map_or(Ok((variant, field)), |field| Err(Error::new(
                                field.span(),
                                r#"exactly one field in enum allowed"#,
                            ))))
                }
                syn::Fields::Unit => Err(Error::new(
                    variant.fields.span(),
                    r#"only enums with unnamed fields allowed"#,
                )),
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|(variant, field)| {
                let ident_string = variant.ident.to_string();
                let field_type = &field.ty;

                let typescriptable_field_type_name = quote_spanned! {field_type.span()=>
                    <#field_type as tucant_derive_lib::Typescriptable>::name()
                };

                let typescriptable_field_type_code = quote_spanned! {field_type.span()=>
                    <#field_type as tucant_derive_lib::Typescriptable>::code()
                };

                (
                    quote! {
                       "{ type: \"" + #ident_string + "\", value: " + &#typescriptable_field_type_name + " }\n"
                    },
                    quote! {
                        result.extend(#typescriptable_field_type_code);
                    },
                )
            })
            .fold((quote! {}, quote! {}), |(accx, accy), (x, y)| {
                (
                    quote! {
                        #accx + " | " + #x
                    },
                    quote! {
                        #accy
                        #y
                    },
                )
            }),
        Data::Union(err) => {
            return Err(Error::new(
                err.union_token.span(),
                r#"unions not supported"#,
            ))
        }
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ty_generics_turbofish = ty_generics.as_turbofish();

    let generics = input
        .generics
        .type_params()
        .map(|TypeParam { ident, .. }| {
            quote! {
                <#ident as tucant_derive_lib::Typescriptable>::name()
            }
        })
        .fold(quote! {}, |acc, val| {
            quote! {
                #acc + &BASE64_URL_SAFE_NO_PAD.encode(
                    #val,
                )
            }
        });

    Ok(quote! {
        impl #impl_generics tucant_derive_lib::Typescriptable for #name #ty_generics #where_clause {
            fn name() -> String {
                // TODO FIXME actual generic typescript types would be way nicer
                #name_string.to_string() #generics
            }

            fn code() -> ::std::collections::BTreeSet<String> {
                let mut result = ::std::collections::BTreeSet::from(["export type ".to_string() + &#name #ty_generics_turbofish::name() + " =\n"
                #members
                ]);
                #members_code
                result
            }
        }
    })
}

#[proc_macro_attribute]
pub fn ts(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(attr as Nothing);
    let input = parse_macro_input!(item as ItemFn);

    proc_macro::TokenStream::from(handle_item_fn(&input).unwrap_or_else(Error::into_compile_error))
}

#[proc_macro_derive(Typescriptable, attributes(ts_type))]
pub fn typescriptable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    proc_macro::TokenStream::from(
        typescriptable_impl(&input).unwrap_or_else(Error::into_compile_error),
    )
}
