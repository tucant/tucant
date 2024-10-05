use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse2, parse_macro_input, DeriveInput, Ident, Token,
};

struct HtmlElement {
    open_start: Token![<],
    element: Ident,
}

impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(HtmlElement {
            open_start: input.parse()?,
            element: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as HtmlElement);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}
