use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::Nothing,
    parse_macro_input,
    spanned::Spanned,
    visit::{Visit},
    Error, Item, ItemEnum, ItemFn, Pat, PatIdent, PatType, Meta, NestedMeta, Lit, DeriveInput, Data, DataStruct, DataEnum,
};

// RUSTFLAGS="-Z macro-backtrace" cargo test

struct InnermostTypeVisitor(Option<TokenStream>);

impl<'ast> Visit<'ast> for InnermostTypeVisitor {
    fn visit_ident(&mut self, ident: &'ast Ident) {
        self.0 = Some(ident.to_token_stream());
    }

    fn visit_angle_bracketed_generic_arguments(
        &mut self,
        i: &'ast syn::AngleBracketedGenericArguments,
    ) {
        if let Some(v) = i.args.first() { self.visit_generic_argument(v) }
    }

    fn visit_type_tuple(&mut self, i: &'ast syn::TypeTuple) {
        self.0 = Some(i.to_token_stream());
    }
}


    fn handle_item_fn(node: &ItemFn) -> TokenStream {
        let return_type = match node.sig.output {
            syn::ReturnType::Default => format_ident!("void").to_token_stream(),
            syn::ReturnType::Type(_, ref path) => {
                let mut innermost_type_visitor = InnermostTypeVisitor(None);
                innermost_type_visitor.visit_type(path);
                innermost_type_visitor.0.unwrap()
            }
        };

        let actix_macro = node.attrs.iter().find(|attr| {
            attr.path.get_ident().map(Ident::to_string) == Some("get".to_string())
            || attr.path.get_ident().map(Ident::to_string) == Some("post".to_string())
        });

        if actix_macro.is_none() {
            return  Error::new_spanned(
                    node,
                    r#"could not find actix get or post attribute macro"#,
                )
                .to_compile_error()
            
        }
        let url_path = actix_macro.unwrap().parse_meta().unwrap();
        let url_path = match url_path {
            Meta::List(meta_list) => {
                match meta_list.nested.iter().next() {
                    Some(NestedMeta::Lit(Lit::Str(str))) => {
                        str.value()
                    }
                    _ => panic!()
                }
            }
            _ => panic!()
        };

        let arg_type = node
            .sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(PatType { pat, ty, .. }) => {
                    if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                        if *ident == "input" || *ident == "_input" {
                            let mut innermost_type_visitor = InnermostTypeVisitor(None);
                            innermost_type_visitor.visit_type(ty);
                            return Some(innermost_type_visitor.0.unwrap().to_token_stream());
                        }
                    }
                    None
                }
            })
            .next();

        if let Some(arg_type) = arg_type {
            let name = &node.sig.ident;
            let name_string = node.sig.ident.to_string();
            return quote! {
                #node

                impl tucant::typescript::Typescriptable for #name {
                    fn name() -> String {
                        #name_string.to_string()
                    }

                    fn code() -> ::std::collections::HashSet<String> {
                        let mut result = ::std::collections::HashSet::from(["export async function ".to_string() + &<#name as tucant::typescript::Typescriptable>::name() + "(input: " + &<#arg_type as tucant::typescript::Typescriptable>::name() + ")"
                        + ": Promise<" + &<#return_type as tucant::typescript::Typescriptable>::name() + "> {" +
                        r#"
    const response = await fetch("http://localhost:8080"# + #url_path + r#"", {
    credentials: "include",
    method: "POST",
    headers: {
        "Content-Type": "application/json",
        "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
    });
    return await response.json() as "# + &<#return_type as tucant::typescript::Typescriptable>::name() +
"\n}"]);
                        result.extend(<#arg_type as tucant::typescript::Typescriptable>::code());
                        result.extend(<#return_type as tucant::typescript::Typescriptable>::code());       
                        result                
                    }
                }
            }
        } else {
            return Error::new(
                    node.sig.inputs.span(),
                    r#"name one of the parameters `input` or `_input`"#,
                )
                .to_compile_error()
                    }
    }


fn typescriptable_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
        let name_string = input.ident.to_string();

        let (members, members_code) = match &input.data {
            Data::Struct(DataStruct { fields: syn::Fields::Named(fields_named), .. }) => {
                fields_named.named.iter().map(|field| {
                    let ident_string = field.ident.as_ref().unwrap().to_string();
                    let field_type = &field.ty;
                    (quote! {
                       "  " + #ident_string + ": " + &<#field_type as tucant::typescript::Typescriptable>::name() + ",\n"
                    }, quote! {
                        result.extend(<#field_type as tucant::typescript::Typescriptable>::code());
                    })
                }).fold((quote! {}, quote! {}), |(accx, accy), (x, y)| (quote! {
                    #accx + #x
                }, quote! {
                    #accy
                    #y
                }))
            },
            Data::Enum(DataEnum { variants, .. }) => {
                variants
                .iter()
                .map(|field| {
                    let ident_string = field.ident.to_string();
                    let (field_type, codes) = match &field.fields {
                        syn::Fields::Named(_) => todo!(),
                        syn::Fields::Unnamed(fields) => fields
                            .unnamed
                            .iter()
                            .map(|field| {
                                let field_type = &field.ty;
                                (quote! {
                                   &<#field_type as tucant::typescript::Typescriptable>::name() + ",\n"
                                }, quote! {
                                    result.extend(<#field_type as tucant::typescript::Typescriptable>::code());
                                })
                            })
                            .fold((quote! {}, quote! {}), |(accx, accy), (x, y)| {
                                (quote! {
                                    #accx + #x
                                },  quote! {
                                    #accy
                                    #y
                                })
                            }),
                        syn::Fields::Unit => todo!(),
                    };
                    (quote! {
                       "  " + #ident_string + ": [" #field_type + "],\n"
                    }, codes)
                })
                .fold((quote! {}, quote! {}), |(accx, accy), (x, y)| {
                    (quote! {
                        #accx + #x
                    }, quote! {
                        #accy
                        #y
                    })
                })
            }
            _ => panic!()
        };

        quote! {
            #input

            impl tucant::typescript::Typescriptable for #name {
                fn name() -> String {
                    #name_string.to_string()
                }

                fn code() -> ::std::collections::HashSet<String> {
                    let mut result = ::std::collections::HashSet::from(["type ".to_string() + &#name::name() + " = {\n"
                    #members
                    + "}"]);
                    #members_code
                    result
                }
            }
        }
}

#[proc_macro_attribute]
pub fn ts(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(attr as Nothing);
    let input = parse_macro_input!(item as ItemFn);

    proc_macro::TokenStream::from(handle_item_fn(&input))
}

#[proc_macro_derive(Typescriptable, attributes(ts_type))]
pub fn typescriptable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    proc_macro::TokenStream::from(typescriptable_impl(input))
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn it_works() {}
}
