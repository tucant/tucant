use std::ops::{RangeFrom, RangeInclusive};

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::Span;
use syn::Ident;
use winnow::{
    ascii::{dec_uint, multispace1, till_line_ending},
    combinator::{alt, opt, preceded, repeat, seq, terminated, trace},
    prelude::*,
    token::{take_until, take_while},
};

use quote::quote;

// https://www.rfc-editor.org/rfc/rfc8610

// grammar: https://www.rfc-editor.org/rfc/rfc8610#appendix-B

#[derive(Debug)]
pub enum Rule {
    Group { name: String, group: (Option<Occur>, Group) },
    Type { name: String, r#type: Type },
}

#[derive(Debug, Clone)]
pub enum Group {
    And(Vec<(Option<Occur>, Group)>),
    Or(Vec<Group>),
    KeyValue(Option<Key>, Type),
    Name(String),
}

#[derive(Debug, Clone)]
pub enum Type {
    Value(Value),
    Typename(String),
    Combined { operator: Operator, first: Box<Type>, second: Box<Type> },
    Or(Vec<Type>),
    Group(Box<Group>),
}

pub fn ccdl(input: &mut &str) -> ModalResult<Vec<Rule>> {
    trace("ccdl", preceded(s, repeat(1.., terminated(rule, s)))).parse_next(input)
}

fn rule(input: &mut &str) -> ModalResult<Rule> {
    trace(
        "rule",
        alt((
            seq! {Rule::Type {
                name: typename.map(ToOwned::to_owned),
                _: (s, assignt, s),
                r#type: r#type
            }},
            seq! {Rule::Group {
                name: groupname.map(ToOwned::to_owned),
                _: (s, assigng, s),
                group: grpent
            }},
        )),
    )
    .parse_next(input)
}

fn typename<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    trace("typename", id).parse_next(input)
}

fn groupname<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    trace("groupname", id).parse_next(input)
}

fn assignt(input: &mut &str) -> ModalResult<usize> {
    trace("assignt", alt(("=", "/=")).map(|_| 1)).parse_next(input)
}

fn assigng(input: &mut &str) -> ModalResult<usize> {
    trace("assigng", alt(("=", "//=")).map(|_| 1)).parse_next(input)
}

fn r#type(input: &mut &str) -> ModalResult<Type> {
    trace(
        "type",
        (type1, repeat(0.., preceded((s, "/", s), type1))).map(|(first, mut rest): (_, Vec<_>)| {
            if rest.is_empty() {
                first
            } else {
                rest.insert(0, first);
                Type::Or(rest)
            }
        }),
    )
    .parse_next(input)
}

fn type1(input: &mut &str) -> ModalResult<Type> {
    trace("type1", (type2, opt((s, alt((rangeop, ctlop)), s, type2)))).map(|(first, second)| if let Some(((), operator, (), second)) = second { Type::Combined { operator, first: Box::new(first), second: Box::new(second) } } else { first }).parse_next(input)
}

fn type2(input: &mut &str) -> ModalResult<Type> {
    // TODO not complete
    trace(
        "type2",
        alt((
            value.map(Type::Value),
            typename.map(|v| Type::Typename(v.to_owned())),
            terminated(preceded(("(", s), r#type), (s, ")")),
            terminated(preceded(("{", s), group), (s, "}")).map(|v| Type::Group(Box::new(v))),
            terminated(preceded(("[", s), group), (s, "]")).map(|v| Type::Group(Box::new(v))), // TODO
        )),
    )
    .parse_next(input)
}

#[derive(Debug, Clone)]
pub enum Operator {
    Range,
    Control(String),
}

fn rangeop(input: &mut &str) -> ModalResult<Operator> {
    alt(("...", "..")).map(|_| Operator::Range).parse_next(input)
}

fn ctlop(input: &mut &str) -> ModalResult<Operator> {
    preceded(".", id).map(|v| Operator::Control(v.to_owned())).parse_next(input)
}

fn group(input: &mut &str) -> ModalResult<Group> {
    trace(
        "group",
        (grpchoice, repeat(0.., preceded((s, "//", s), grpchoice))).map(|(first, mut rest): (_, Vec<_>)| {
            if rest.is_empty() {
                first
            } else {
                rest.insert(0, first);
                Group::Or(rest)
            }
        }),
    )
    .parse_next(input)
}

fn grpchoice(input: &mut &str) -> ModalResult<Group> {
    trace("grpchoice", repeat(0.., terminated(grpent, optcom))).map(Group::And).parse_next(input)
}

fn grpent(input: &mut &str) -> ModalResult<(Option<Occur>, Group)> {
    let a = (opt(terminated(memberkey, s)), r#type).map(|(key, value)| Group::KeyValue(key, value));
    let b = groupname.map(|v| Group::Name(v.to_owned())); // not complete
    let c = terminated(preceded(("(", s), group), (s, ")"));
    trace("grpent", (opt(terminated(occur, s)), alt((a, b, c)))).parse_next(input)
}

#[derive(Debug, Clone)]
pub enum Key {
    Type(Type),
    Value(Value),
    Literal(String),
}

fn memberkey(input: &mut &str) -> ModalResult<Key> {
    let a = terminated(type1, (s, "=>")).map(Key::Type); // not complete
    let b = terminated(bareword, (s, ":")).map(|v| Key::Literal(v.to_owned()));
    let c = terminated(value, (s, ":")).map(Key::Value);
    trace("memberkey", alt((a, b, c))).parse_next(input)
}

fn bareword<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    trace("bareword", id).parse_next(input)
}

fn optcom(input: &mut &str) -> ModalResult<()> {
    trace("optcom", (s, opt((",", s))).map(|_| ())).parse_next(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Occur {
    RangeInclusive(RangeInclusive<usize>),
    RangeFrom(RangeFrom<usize>),
}

fn occur(input: &mut &str) -> ModalResult<Occur> {
    // TODO dec_uint not fully correct
    trace(
        "occur",
        alt((
            (opt(dec_uint), "*", opt(dec_uint)).map(|(l, _, u): (Option<usize>, _, Option<usize>)| match (l, u) {
                (Some(l), Some(u)) => Occur::RangeInclusive(l..=u),
                (Some(l), None) => Occur::RangeFrom(l..),
                (None, Some(u)) => Occur::RangeInclusive(0..=u),
                (None, None) => Occur::RangeFrom(0..),
            }),
            "+".map(|_| Occur::RangeFrom(1..)),
            "?".map(|_| Occur::RangeInclusive(0..=1)),
        )),
    )
    .parse_next(input)
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(String),
}

fn value(input: &mut &str) -> ModalResult<Value> {
    trace("value", alt((take_while(1.., ('0'..='9', 'e', '.', '-', '+')).map(|v: &str| Value::Number(v.to_owned())), seq!(_: "\"", take_until(0.., "\""), _: "\"").map(|(v,): (&str,)| Value::String(v.to_owned()))))).parse_next(input)
}

fn id<'a>(s: &mut &'a str) -> ModalResult<&'a str> {
    trace("id", take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '-', '.'))).parse_next(s)
}

fn s(input: &mut &str) -> ModalResult<()> {
    repeat(0.., alt((multispace1.map(|_| ()), (";", till_line_ending).map(|_| ())))).parse_next(input)
}

pub fn codegen(rules: &[Rule]) {
    std::fs::write("../tucant-tests/src/cddl.txt", format!("{rules:#?}")).unwrap();
    let rules = rules.iter().map(codegen_rule);
    let rules = quote! {
        /// https://www.rfc-editor.org/rfc/rfc8610#appendix-D
        pub struct TODO;
        pub type Text = String;
        pub type Any = serde_json::Value;


        #(#rules)*
    };
    std::fs::write("../tucant-tests/src/cddl.rs", format!("{rules}")).unwrap();
    let syntax_tree = syn::parse2(rules).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    std::fs::write("../tucant-tests/src/cddl.rs", &code).unwrap();
}

fn codegen_type(r#type: &Type) -> proc_macro2::TokenStream {
    match r#type {
        Type::Value(value) => match value {
            Value::String(_) => quote! { String },
            Value::Number(_) => quote! { TODO },
        },
        Type::Typename(name) => {
            let name = Ident::new_raw(&name.to_upper_camel_case(), Span::call_site());
            quote! {
                #name
            }
        }
        Type::Combined { operator: _, first: _, second: _ } => quote! { TODO },
        Type::Or(_items) => quote! { TODO },
        Type::Group(_group) => quote! { TODO },
    }
}

fn codegen_group(group: &(Option<Occur>, Group)) -> proc_macro2::TokenStream {
    // TODO occur
    let group = &group.1;
    
    match group {
        Group::And(items) => {
            let items = items.iter().map(codegen_group);
            quote! {
                #(#items)*
            }
        }
        Group::Or(_groups) => quote! { pub todo: TODO, },
        Group::KeyValue(key, r#type) => {
            let type_tokens = codegen_type(r#type);
            match key {
                Some(Key::Type(_)) => quote! {
                    pub TODO: #type_tokens,
                },
                Some(Key::Value(_value)) => quote! {
                    pub TODO: #type_tokens,
                },
                Some(Key::Literal(literal)) => {
                    let key = Ident::new_raw(literal, Span::call_site());
                    quote! {
                        pub #key: #type_tokens,
                    }
                }
                None => {
                    if let Type::Typename(name) = r#type {
                        let key = Ident::new_raw(&name.to_snake_case(), Span::call_site());
                        quote! {
                            #[serde(flatten)]
                            pub #key: #type_tokens,
                        }
                    } else {
                        quote! {
                            pub NO_KEY: #type_tokens,
                        }
                    }
                }
            }
        }
        Group::Name(name) => {
            let name_snake = Ident::new_raw(&name.to_snake_case(), Span::call_site());
            let name = Ident::new_raw(&name.to_upper_camel_case(), Span::call_site());
            quote! {
                #[serde(flatten)]
                pub #name_snake: #name,
            }
        }
    }
}

fn codegen_rule(rule: &Rule) -> proc_macro2::TokenStream {
    match rule {
        Rule::Group { name, group } => {
            let name = Ident::new_raw(&name.to_upper_camel_case(), Span::call_site());
            match &group.1 {
                Group::And(_items) => {
                    let inner = codegen_group(group);
                    quote! {
                        pub struct #name {
                            #inner
                        }
                    }
                }
                Group::Or(groups) => {
                    let groups = groups.iter().map(|group| match group {
                        Group::And(_items) => quote! { Todo, },
                        Group::Or(_groups) => todo!(),
                        Group::KeyValue(_key, _) => todo!(),
                        Group::Name(name) => {
                            let name = Ident::new_raw(&name.to_upper_camel_case(), Span::call_site());
                            quote! { #name(#name), }
                        }
                    });
                    quote! {
                        #[derive(Serialize, Deserialize)]
                        pub enum #name {
                            #(#groups)*
                        }
                    }
                }
                Group::KeyValue(_key, _) => todo!(),
                Group::Name(_) => todo!(),
            }
        }
        Rule::Type { name, r#type } => {
            let name_ident = Ident::new_raw(&name.to_upper_camel_case(), Span::call_site());
            match r#type {
                Type::Value(_value) => quote! { pub type #name_ident = TODO; },
                Type::Typename(_name) => quote! { pub type #name_ident = TODO; },
                Type::Combined { operator: _, first: _, second: _ } => quote! { pub type #name_ident = TODO; },
                Type::Or(_items) => quote! { pub type #name_ident = TODO; },
                Type::Group(group) => codegen_rule(&Rule::Group { name: name.to_owned(), group: (None, *group.clone()) }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use winnow::Parser;

    use crate::{ccdl, codegen};

    #[test]
    #[ignore]
    fn it_works() {
        let input = read_to_string("../../webdriver-bidi/all.cddl").unwrap();
        let parsed = ccdl.parse(&input).unwrap();
        codegen(&parsed);
    }
}
