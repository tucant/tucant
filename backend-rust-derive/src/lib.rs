use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Nothing,
    parse_macro_input,
    spanned::Spanned,
    visit::{self, Visit},
    Error, Item, ItemFn, Type, TypePath,
};

/*
struct Struct1 {
    val1: Type1,
    val2: Type2
}
-->
static Struct1_typescript: &str = R#"
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

struct FnVisitor;

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        println!("Function with name={}", node.sig.ident);
        let return_type = match &node.sig.output {
            syn::ReturnType::Default => "void".to_string(),
            syn::ReturnType::Type(_, path) => match &**path {
                Type::Path(TypePath { path, .. }) => format!(
                    "{:?}",
                    path.segments
                        .iter()
                        .map(|s| s.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("_")
                ),
                _ => panic!(),
            },
        };
        println!("Function with return type={}", return_type);

        visit::visit_item_fn(self, node);
    }
}

#[proc_macro_attribute]
pub fn typescript(attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(attr as Nothing);
    let input = parse_macro_input!(item as Item);

    match input {
        Item::Fn(function) => FnVisitor.visit_item_fn(&function),
        Item::Struct(structure) => {}
        wrong_item => {
            return TokenStream::from(
                Error::new(wrong_item.span(), "expected function or struct").to_compile_error(),
            )
        }
    }

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
