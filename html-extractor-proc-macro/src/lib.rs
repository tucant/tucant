use itertools::Itertools;
use proc_macro::TokenTree;
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
    Comment(HtmlComment),
    Text(HtmlText),
}

impl Parse for HtmlCommand {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input.parse().map(Self::Text)
        } else if lookahead.peek(Ident::peek_any) {
            input.parse().map(Self::Whitespace)
        } else if lookahead.peek(Token![<]) {
            if input.peek2(Token![/]) {
                input.parse().map(Self::ElementClose)
            } else if input.peek2(Token![!]) {
                input.parse().map(Self::Comment)
            } else {
                input.parse().map(Self::ElementOpen)
            }
        } else {
            Err(lookahead.error())
        }
    }
}

struct HtmlText {
    text: LitStr,
}

impl Parse for HtmlText {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            text: input.parse()?,
        })
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

enum DashOrColon {
    Dash(Token![-]),
    Colon(Token![:]),
}

impl DashOrColon {
    fn span(&self) -> proc_macro2::Span {
        match self {
            DashOrColon::Dash(minus) => minus.span(),
            DashOrColon::Colon(colon) => colon.span(),
        }
    }
}

impl Parse for DashOrColon {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![-]) {
            input.parse().map(Self::Dash)
        } else if lookahead.peek(Token![:]) {
            input.parse().map(Self::Colon)
        } else {
            Err(lookahead.error())
        }
    }
}

enum HtmlAttributeValue {
    Literal(LitStr),
    Variable(Ident),
}

impl Parse for HtmlAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Ident::peek_any) {
            input.call(Ident::parse_any).map(Self::Variable)
        } else {
            Err(lookahead.error())
        }
    }
}

struct HtmlAttribute {
    ident: Punctuated<Ident, DashOrColon>,
    equals: Token![=],
    value: HtmlAttributeValue,
}

impl Parse for HtmlAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident: Punctuated<Ident, DashOrColon> = Punctuated::new();
        ident.push_value(input.call(Ident::parse_any)?);
        while input.peek(Token![-]) || input.peek(Token![:]) {
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

struct HtmlComment {
    open1: Token![<],
    open2: Token![!],
    open3: Token![-],
    open4: Token![-],
    comment: LitStr,
    close1: Token![-],
    close2: Token![-],
    close3: Token![>],
}

impl Parse for HtmlComment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            open1: input.parse()?,
            open2: input.parse()?,
            open3: input.parse()?,
            open4: input.parse()?,
            comment: input.parse()?,
            close1: input.parse()?,
            close2: input.parse()?,
            close3: input.parse()?,
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
                    let name = iter
                        .ident
                        .pairs()
                        .map(|p| {
                            p.value().to_string()
                                + match p.punct() {
                                    Some(DashOrColon::Colon(_)) => ":",
                                    Some(DashOrColon::Dash(_)) => "-",
                                    None => "",
                                }
                        })
                        .join("");
                    let value = &iter.value;
                    match value {
                        HtmlAttributeValue::Literal(lit_str) => {
                            quote_spanned! {iter.ident.first().unwrap().span()=>
                                let html_handler = html_handler.attribute(#name, #lit_str);
                            }
                        }
                        HtmlAttributeValue::Variable(ident) => {
                            quote_spanned! {iter.ident.first().unwrap().span()=>
                                let (html_handler, #ident) = html_handler.attribute_value(#name);
                            }
                        }
                    }
                });

                let open = quote_spanned! {input.element.span()=>
                    let html_handler = html_handler.next_child_tag_open_start(#tag);
                };

                let close = quote_spanned! {input.open_end.span()=>
                    let html_handler = html_handler.tag_open_end();
                };

                // Build the output, possibly using quasi-quotation
                quote! {
                    #open
                    #(
                        #attributes
                    )*
                    #close
                }
            }
            HtmlCommand::Whitespace(html_whitespace) => {
                quote_spanned! {html_whitespace.underscore.span()=>
                    let html_handler = html_handler.skip_whitespace();
                }
            }
            HtmlCommand::ElementClose(html_element_close) => {
                let name = html_element_close.element.to_string();
                quote_spanned! {html_element_close.element.span()=>
                    let html_handler = html_handler.close_element(#name);
                }
            }
            HtmlCommand::Comment(html_comment) => {
                let comment = &html_comment.comment;
                quote_spanned! {html_comment.comment.span()=>
                    let html_handler = html_handler.skip_comment(#comment);
                }
            }
            HtmlCommand::Text(html_text) => {
                let text = &html_text.text;
                quote_spanned! {text.span()=>
                    let html_handler = html_handler.skip_text(#text);
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
