use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Nothing, parse_macro_input, Error, LitStr, token::Brace, Expr};

fn handle_lit_fn(node: &Expr) -> syn::Result<TokenStream> {
    println!("{:?}", node);
    Ok(quote! {
        let a = #node;
    })
}

// cargo expand --test meta_model
#[proc_macro]
pub fn magic(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // TODO FIXME I think this parses weird
    let input = parse_macro_input!(item as Expr);

    proc_macro::TokenStream::from(handle_lit_fn(&input).unwrap_or_else(Error::into_compile_error))
}
