use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, format_ident};
use syn::{
    parse::Nothing,
    parse_macro_input,
    spanned::Spanned,
    visit::{self, Visit},
    Error, Item, ItemFn, Pat, PatIdent, PatType, Type, TypePath,
};

// RUSTFLAGS="-Z macro-backtrace" cargo test

/*
struct Struct1 {
    val1: Type1,
    val2: Type2
}
-->
static Struct1_typescript: &str = r#"
type Struct1 {
    val1: Type1,
    val2: Type2
}
"#;

#[post("/login")]
async fn login(
    session: Session,
    tucan: web::Data<Tucan>,
    login: web::Json<Login>,
) -> Result<Json<LoginResult>>, MyError> {
    let tucan_user = tucan.login(&login.username, &login.password).await?;
    session.insert("session", tucan_user.session).unwrap();
    Ok(web::Json(LoginResult { success: true }))
}
-->
static login_typescript: &str = r#"
async function login(login: Login): LoginResult {
    fetch("/login")...
}
"#;
*/

/*
if syn is epic it may be possible to somehow extract the data based on a ref to the other types
*/

/*
typescript_app!(app, index, login, logout, get_modules)

->

app.service(index).service(login).service(logout),....

write_to_file(login_typescript, ...)
*/

fn type_to_string(the_type: &Box<Type>) -> String {
    match &**the_type {
        Type::Path(TypePath { path, .. }) => format!(
            "{:?}",
            path.segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("_")
        ),
        _ => panic!(),
    }
}

fn pat_to_string(pat: &Box<Pat>) -> String {
    match &**pat {
        Pat::Ident(PatIdent { ident, .. }) => ident.to_string(),
        _ => panic!(),
    }
}

struct FnVisitor(Option<TokenStream>);

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        println!("Function with name={}", node.sig.ident);
        let return_type = match node.sig.output {
            syn::ReturnType::Default => format_ident!("void").to_token_stream(),
            syn::ReturnType::Type(_, ref path) => path.to_token_stream(),
        };
        println!("Function with return type={}", return_type);

        /*let args = node
            .sig
            .inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(_) => todo!(),
                syn::FnArg::Typed(PatType { pat, ty, .. }) => {
                    (pat_to_string(pat), type_to_string(ty))
                }
            })
            .collect::<Vec<_>>();*/

        let arg = node.sig.inputs.iter().next().unwrap();
        let arg_type = match arg {
            syn::FnArg::Receiver(_) => todo!(),
            syn::FnArg::Typed(PatType { ty, .. }) => {
                ty
            }
        };

        let name = &node.sig.ident;
        let name_string = node.sig.ident.to_string();
        self.0 = Some(quote! {
            #node

            impl ::tucant::typescript::Typescriptable for #name {
                fn name() -> String {
                    #name_string.to_string()
                }

                fn code() -> String {
                    "function ".to_string() + &#name::name() + "(input: " + &#arg_type::name() + ")"
                    + " -> " + &#return_type::name() + " {\n"

                    + "\n}"
                }
            }
        });
    }
}

struct StructVisitor(Option<TokenStream>);

impl<'ast> Visit<'ast> for StructVisitor {
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        let name = &node.ident;
        let name_string = node.ident.to_string();

        let members = match &node.fields {
            syn::Fields::Named(fields_named) => {
                fields_named.named.iter().map(|field| {
                    let ident_string = field.ident.as_ref().unwrap().to_string();
                    let field_type = &field.ty;
                    quote! {
                       "  " + #ident_string + ": " + &<#field_type as ::tucant::typescript::Typescriptable>::name() + ",\n"
                    }
                }).fold(quote! {}, |acc, x| quote! {
                    #acc + #x
                })
            },
            syn::Fields::Unnamed(_) => todo!(),
            syn::Fields::Unit => todo!(),
        };

        self.0 = Some(quote! {
            impl ::tucant::typescript::Typescriptable for #name {
                fn name() -> String {
                    #name_string.to_string()
                }

                fn code() -> String {
                    "type ".to_string() + &#name::name() + " = {\n"
                    #members
                    + "}"
                }
            }
        })
    }
}

fn typescript_impl(input: Item) -> TokenStream {
    match &input {
        Item::Fn(function) => {
            let mut visitor = FnVisitor(None);
            visitor.visit_item_fn(&function);
            visitor.0.unwrap()
        }
        Item::Struct(structure) => {
            let mut visitor = StructVisitor(None);
            visitor.visit_item_struct(&structure);
            let typescript_code = visitor.0.unwrap();
            quote! {
                #typescript_code
                #input
            }
        }
        // TODO for enums add #[serde(tag = "type")]
        wrong_item => {
            return Error::new(wrong_item.span(), "expected function or struct").to_compile_error()
        }
    }
}

#[proc_macro_attribute]
pub fn ts(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_macro_input!(attr as Nothing);
    let input = parse_macro_input!(item as Item);

    proc_macro::TokenStream::from(typescript_impl(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
