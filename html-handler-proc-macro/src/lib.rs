use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn my_macro(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {}
