use itertools::Itertools;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    ext::IdentExt as _,
    parse::{Parse, ParseStream},
    parse2, parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    DeriveInput, Ident, LitStr, Token,
};

struct HtmlAttribute {
    ident: Punctuated<Ident, Token![-]>,
    equals: Token![=],
    value: LitStr,
}

impl Parse for HtmlAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident: Punctuated<Ident, Token![-]> = Punctuated::new();
        ident.push_value(input.call(Ident::parse_any)?);
        while input.peek(Token![-]) {
            ident.push_punct(input.parse()?);
            ident.push_value(input.parse()?);
        }
        let equals = input.parse()?;
        let value = input.parse()?;
        Ok(Self {
            ident,
            equals,
            value,
        })
    }
}

struct HtmlElement {
    open_start: Token![<],
    element: Ident,
    attributes: Vec<HtmlAttribute>,
    open_end: Token![>],
}

impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let open_start = input.parse()?;
        let element = input.call(Ident::parse_any)?;
        let mut attributes = Vec::new();
        while !input.peek(Token!(>)) {
            attributes.push(input.parse()?);
        }
        let open_end = input.parse()?;
        Ok(HtmlElement {
            open_start,
            element,
            attributes,
            open_end,
        })
    }
}

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as HtmlElement);

    let tag = input.element.to_string();

    let attributes = input.attributes.iter().map(|iter| {
        let name = iter.ident.iter().map(|e| e.to_string()).join("-");
        let value = &iter.value;
        quote_spanned! {iter.ident.span()=>
            let html_handler = html_handler.attribute(#name, #value);
        }
    });

    let open = quote_spanned! {input.element.span()=>
        let html_handler = html_handler.next_child_tag_open_start(#tag);
    };

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        #open
        #(
            #attributes
        )*
        let html_handler = html_handler.tag_open_end();
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}
