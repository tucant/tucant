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

struct HtmlCommands {
    commands: Vec<HtmlCommand>,
}

impl Parse for HtmlCommands {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut commands = Vec::new();
        while !input.is_empty() {
            commands.push(input.parse()?);
        }
        Ok(Self { commands })
    }
}

enum HtmlCommand {
    ElementOpen(HtmlElement),
    Whitespace(HtmlWhitespace),
    ElementClose(HtmlElementClose),
}

impl Parse for HtmlCommand {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![_]) {
            input.parse().map(Self::Whitespace)
        } else if lookahead.peek(Token![<]) {
            if input.peek2(Token![/]) {
                input.parse().map(Self::ElementClose)
            } else {
                input.parse().map(Self::ElementOpen)
            }
        } else {
            Err(lookahead.error())
        }
    }
}

struct HtmlWhitespace {
    underscore: Token![_],
}

impl Parse for HtmlWhitespace {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            underscore: input.parse()?,
        })
    }
}

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

struct HtmlElementClose {
    close: Token![<],
    close_slash: Token![/],
    element: Ident,
    open_end: Token![>],
}

impl Parse for HtmlElementClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let close = input.parse()?;
        let close_slash = input.parse()?;
        let element = input.call(Ident::parse_any)?;
        let open_end = input.parse()?;
        Ok(Self {
            close,
            close_slash,
            element,
            open_end,
        })
    }
}

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as HtmlCommands);

    let expanded = input.commands.iter().map(|command| {
        match command {
            HtmlCommand::ElementOpen(input) => {
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
                quote! {
                    #open
                    #(
                        #attributes
                    )*
                    let html_handler = html_handler.tag_open_end();
                }
            }
            HtmlCommand::Whitespace(html_whitespace) => {
                quote_spanned! {html_whitespace.underscore.span()=>
                    let html_handler = html_handler.skip_whitespace();
                }
            }
            HtmlCommand::ElementClose(html_element_close) => {
                quote_spanned! {html_element_close.element.span()=>
                    let html_handler = html_handler.close_element();
                }
            }
        }
    });
    let result = quote! {
        #(#expanded)*
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(result)
}
