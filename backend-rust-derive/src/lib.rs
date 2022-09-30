use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

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

// https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs

#[proc_macro_attribute]
pub fn typescript(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as DeriveInput);

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
    fn it_works() {
    }
}
