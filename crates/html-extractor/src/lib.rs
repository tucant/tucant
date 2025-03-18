use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    Block, Expr, ExprStruct, Ident, LitStr, Stmt, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Brace,
};

#[derive(Debug)]
struct HtmlCommands {
    commands: Vec<HtmlCommand>,
}

impl HtmlCommands {
    fn span(&self) -> Span {
        self.commands.iter().map(|command| command.span()).reduce(|a, b| a.join(b).unwrap_or(a)).unwrap_or(Span::call_site())
    }
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

#[derive(Debug)]
enum HtmlCommand {
    ElementOpen(HtmlElement),
    Whitespace(HtmlWhitespace),
    ElementClose(HtmlElementClose),
    Comment(HtmlComment),
    Text(StringLiteralOrVariable),
    Use(Expr),
    Extern(Block),
    Let(HtmlLet),
}

impl HtmlCommand {
    pub fn span(&self) -> Span {
        match self {
            HtmlCommand::ElementOpen(html_element) => html_element.span(),
            HtmlCommand::Whitespace(html_whitespace) => html_whitespace.span(),
            HtmlCommand::ElementClose(html_element_close) => html_element_close.span(),
            HtmlCommand::Comment(html_comment) => html_comment.span(),
            HtmlCommand::Text(string_literal_or_variable) => string_literal_or_variable.span(),
            HtmlCommand::Use(expr) => expr.span(),
            HtmlCommand::Extern(block) => block.span(),
            HtmlCommand::Let(html_let) => html_let.span(),
        }
    }
}

impl Parse for HtmlCommand {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![let]) {
            input.parse().map(Self::Let)
        } else if lookahead.peek(Token![use]) {
            input.parse::<Token![use]>()?;
            let res = input.parse().map(Self::Use)?;
            input.parse::<Token![;]>()?;
            Ok(res)
        } else if lookahead.peek(Token![extern]) {
            input.parse::<Token![extern]>()?;
            let res = input.parse().map(Self::Extern)?;
            Ok(res)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Text)
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::Text)
        } else if lookahead.peek(Token![_]) {
            input.parse().map(Self::Whitespace)
        } else if lookahead.peek(Ident::peek_any) {
            input.parse().map(Self::Text)
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

#[derive(Debug)]
struct HtmlLet {
    let_: Token![let],
    variable: Ident,
    eq: Token![=],
    inner: HtmlLetInner,
    semi: Token![;],
}

impl HtmlLet {
    pub fn span(&self) -> Span {
        self.let_.span()
    }
}

impl Parse for HtmlLet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let let_ = input.parse::<Token![let]>()?;
        let variable = input.parse()?;
        let eq = input.parse::<Token![=]>()?;
        let inner = input.parse()?;
        let semi = input.parse::<Token![;]>()?;
        Ok(Self { let_, variable, eq, inner, semi })
    }
}

#[derive(Debug)]
enum HtmlLetInner {
    Expr(Expr),
    If(HtmlIf),
    While(HtmlWhile),
}

impl HtmlLetInner {
    pub fn span(&self) -> Span {
        match self {
            HtmlLetInner::Expr(expr) => expr.span(),
            HtmlLetInner::If(html_if) => html_if.span(),
            HtmlLetInner::While(html_while) => html_while.span(),
        }
    }
}

impl Parse for HtmlLetInner {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![if]) {
            input.parse().map(Self::If)
        } else if input.peek(Token![while]) {
            input.parse().map(Self::While)
        } else {
            input.parse().map(Self::Expr)
        }
    }
}

#[derive(Debug)]
struct HtmlWhitespace {
    underscore: Token![_],
}

impl HtmlWhitespace {
    pub fn span(&self) -> Span {
        self.underscore.span()
    }
}

impl Parse for HtmlWhitespace {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { underscore: input.parse()? })
    }
}

#[derive(Debug)]
enum DashOrColon {
    Dash,
    Colon,
}

impl Parse for DashOrColon {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![-]) {
            input.parse().map(|_: Token![-]| Self::Dash)
        } else if lookahead.peek(Token![:]) {
            input.parse().map(|_: Token![:]| Self::Colon)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
enum StringLiteralOrVariable {
    Literal(LitStr),
    Variable(Ident),
    Expression(Expr),
}

impl StringLiteralOrVariable {
    pub fn span(&self) -> Span {
        match self {
            StringLiteralOrVariable::Literal(lit_str) => lit_str.span(),
            StringLiteralOrVariable::Variable(ident) => ident.span(),
            StringLiteralOrVariable::Expression(expr) => expr.span(),
        }
    }
}

impl Parse for StringLiteralOrVariable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let content;
            let _brace_token = braced!(content in input);

            content.parse().map(Self::Expression)
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Ident::peek_any) {
            input.call(Ident::parse_any).map(Self::Variable)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
struct HtmlAttribute {
    ident: Punctuated<Ident, DashOrColon>,
    value: StringLiteralOrVariable,
}

impl Parse for HtmlAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident: Punctuated<Ident, DashOrColon> = Punctuated::new();
        ident.push_value(input.call(Ident::parse_any)?);
        while input.peek(Token![-]) || input.peek(Token![:]) {
            ident.push_punct(input.parse()?);
            ident.push_value(input.parse()?);
        }
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { ident, value })
    }
}

#[derive(Debug)]
struct HtmlElement {
    element: Ident,
    attributes: Vec<HtmlAttribute>,
    open_end: Token![>],
}

impl HtmlElement {
    pub fn span(&self) -> Span {
        self.element.span()
    }
}

impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        let element = input.call(Ident::parse_any)?;
        let mut attributes = Vec::new();
        while !input.peek(Token!(>)) {
            attributes.push(input.parse()?);
        }
        let open_end = input.parse()?;
        Ok(Self { element, attributes, open_end })
    }
}

#[derive(Debug)]
struct HtmlElementClose {
    element: Ident,
}

impl HtmlElementClose {
    pub fn span(&self) -> Span {
        self.element.span()
    }
}

impl Parse for HtmlElementClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let element = input.call(Ident::parse_any)?;
        input.parse::<Token![>]>()?;
        Ok(Self { element })
    }
}

#[derive(Debug)]
struct HtmlComment {
    comment: LitStr,
}

impl HtmlComment {
    pub fn span(&self) -> Span {
        self.comment.span()
    }
}

impl Parse for HtmlComment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        input.parse::<Token![!]>()?;
        input.parse::<Token![-]>()?;
        input.parse::<Token![-]>()?;
        let comment: LitStr = input.parse()?;
        input.parse::<Token![-]>()?;
        input.parse::<Token![-]>()?;
        input.parse::<Token![>]>()?;
        Ok(Self { comment })
    }
}

// TODO FIXME implement arbitrary children matcher
// TODO FIXME allow calling subtemplates inside of a html_extractor?
// TODO FIXME implement else
// TODO FIXME  => a = statement_evaluating_to_unit
// html_handler = html_handler.skip_any_comment(); probably a special case. maybe allow <!-- variable -->
#[derive(Debug)]
struct HtmlIf {
    conditional: Expr,
    body: HtmlCommands,
    result_expr: Expr,
    else_: Option<(HtmlCommands, Expr)>,
}

impl HtmlIf {
    pub fn span(&self) -> Span {
        self.conditional.span()
    }
}

impl Parse for HtmlIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        let conditional = input.parse()?;
        let body_parse_buffer;
        let _brace_token = braced!(body_parse_buffer in input);
        let body = body_parse_buffer.parse()?;
        input.parse::<Token![=]>()?;
        input.parse::<Token![>]>()?;
        let result_expr = input.parse()?;
        let else_ = if input.peek2(Token![else]) {
            input.parse::<Token![;]>()?;
            input.parse::<Token![else]>()?;
            let body_parse_buffer;
            let _brace_token = braced!(body_parse_buffer in input);
            let body = body_parse_buffer.parse()?;
            input.parse::<Token![=]>()?;
            input.parse::<Token![>]>()?;
            let result_expr = input.parse()?;
            Some((body, result_expr))
        } else {
            None
        };
        Ok(Self { conditional, body, result_expr, else_ })
    }
}

#[derive(Debug)]
struct HtmlWhile {
    conditional: Expr,
    body: HtmlCommands,
    result_expr: Expr,
}

impl HtmlWhile {
    pub fn span(&self) -> Span {
        self.conditional.span()
    }
}

impl Parse for HtmlWhile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![while]>()?;
        let conditional = input.parse()?;
        let body_parse_buffer;
        let _brace_token = braced!(body_parse_buffer in input);
        let body = body_parse_buffer.parse()?;
        input.parse::<Token![=]>()?;
        input.parse::<Token![>]>()?;
        let result_expr = input.parse()?;
        Ok(Self { conditional, body, result_expr })
    }
}

fn convert_commands(commands: &HtmlCommands) -> Vec<TokenStream> {
    commands
        .commands
        .iter()
        .map(|command| match command {
            HtmlCommand::ElementOpen(input) => {
                let tag = input.element.to_string();

                let attributes = input.attributes.iter().map(|iter| {
                    let name = iter
                        .ident
                        .pairs()
                        .map(|p| {
                            p.value().to_string()
                                + match p.punct() {
                                    Some(DashOrColon::Colon) => ":",
                                    Some(DashOrColon::Dash) => "-",
                                    None => "",
                                }
                        })
                        .join("");
                    let value = &iter.value;
                    match value {
                        StringLiteralOrVariable::Literal(lit_str) => {
                            quote_spanned! {lit_str.span()=>
                                #[allow(unused_mut)]
                                let mut html_handler = html_handler.attribute(#name, #lit_str);
                            }
                        }
                        StringLiteralOrVariable::Expression(expr) => {
                            quote_spanned! {expr.span()=>
                                let tmp_internal_html_extractor_proc_macro: &str = #expr;
                                #[allow(unused_mut)]
                                let mut html_handler = html_handler.attribute(#name, tmp_internal_html_extractor_proc_macro);
                            }
                        }
                        StringLiteralOrVariable::Variable(ident) => {
                            quote_spanned! {ident.span()=>
                                #[allow(unused_mut)]
                                let (mut html_handler, #ident) = html_handler.attribute_value(#name);
                            }
                        }
                    }
                });

                let open = quote_spanned! {input.element.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.next_child_tag_open_start(#tag);
                };

                let close = quote_spanned! {input.open_end.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.tag_open_end();
                };

                quote_spanned! {open.span()=>
                    #open
                    #(
                        #attributes
                    )*
                    #close
                }
            }
            HtmlCommand::Whitespace(html_whitespace) => {
                quote_spanned! {html_whitespace.underscore.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.skip_whitespace();
                }
            }
            HtmlCommand::ElementClose(html_element_close) => {
                let name = html_element_close.element.to_string();
                quote_spanned! {html_element_close.element.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.close_element(#name);
                }
            }
            HtmlCommand::Comment(html_comment) => {
                let comment = &html_comment.comment;
                quote_spanned! {html_comment.comment.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.skip_comment(#comment);
                }
            }
            HtmlCommand::Text(html_text) => match html_text {
                StringLiteralOrVariable::Literal(lit_str) => {
                    quote_spanned! {lit_str.span()=>
                        #[allow(unused_mut)]
                        let mut html_handler = html_handler.skip_text(#lit_str);
                    }
                }
                StringLiteralOrVariable::Expression(expr) => {
                    quote_spanned! {expr.span()=>
                        #[allow(unused_mut)]
                        let mut html_handler = html_handler.skip_text(#expr);
                    }
                }
                StringLiteralOrVariable::Variable(ident) => {
                    quote_spanned! {ident.span()=>
                        #[allow(unused_mut)]
                        let (mut html_handler, #ident) = html_handler.text();
                    }
                }
            },
            HtmlCommand::Use(use_expr) => {
                quote_spanned! {use_expr.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = #use_expr;
                }
            }
            HtmlCommand::Extern(extern_body) => {
                let body = &extern_body.stmts;
                quote_spanned! {extern_body.span()=>
                    #(#body)*
                }
            }
            HtmlCommand::Let(html_let) => {
                let variable = &html_let.variable;
                match &html_let.inner {
                    HtmlLetInner::Expr(expr) => {
                        quote_spanned! {expr.span()=>
                            let (html_handler, #variable) = #expr;
                        }
                    }
                    HtmlLetInner::If(html_if) => {
                        let conditional = &html_if.conditional;
                        let body = convert_commands(&html_if.body);
                        let result_expr = &html_if.result_expr;
                        let temp_var = Ident::new("temp_var", Span::mixed_site());
                        if let Some(else_) = &html_if.else_ {
                            let else_body = convert_commands(&else_.0);
                            let else_result_expr = &else_.1;
                            quote_spanned! {else_.0.span()=>
                                let (mut html_handler, #temp_var) = if #conditional {
                                    #(#body)*
                                    (html_handler, ::itertools::Either::Left(#result_expr))
                                } else {
                                    #(#else_body)*
                                    (html_handler, ::itertools::Either::Right(#else_result_expr))
                                };
                                let #variable = #temp_var;
                            }
                        } else {
                            quote_spanned! {html_if.body.span()=>
                                let #temp_var;
                                (html_handler, #temp_var) = if #conditional {
                                    #(#body)*
                                    (html_handler, Some(#result_expr))
                                } else {
                                    (html_handler, None)
                                };
                                let #variable = #temp_var;
                            }
                        }
                    }
                    HtmlLetInner::While(html_while) => {
                        let conditional = &html_while.conditional;
                        let body = convert_commands(&html_while.body);
                        let result_expr = &html_while.result_expr;
                        let temp_vec = Ident::new("temp_vec", Span::mixed_site());
                        quote_spanned! {html_while.body.span()=>
                            let mut #temp_vec = Vec::new();
                            while (#conditional) {
                                html_handler = {
                                    let (html_handler, tmp) = {
                                        #(#body)*

                                        (html_handler, #result_expr)
                                    };
                                    #temp_vec.push(tmp);
                                    html_handler
                                };
                            }
                            let mut #variable = #temp_vec;
                        }
                    }
                }
            }
        })
        .collect()
}

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as HtmlCommands);

    let expanded = convert_commands(&input);
    let result = quote! {
        #(#expanded)*
    };

    proc_macro::TokenStream::from(result)
}
