use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive};

use heck::{ToPascalCase, ToSnakeCase, ToUpperCamelCase};
use syn::File;
use winnow::{
    ascii::{alpha1, dec_uint, float, multispace0, multispace1, till_line_ending},
    combinator::{alt, cut_err, dispatch, fail, opt, preceded, repeat, seq, terminated, trace},
    error::{StrContext, StrContextValue},
    prelude::*,
    token::{take, take_until, take_while},
};

use quote::{format_ident, quote};

// https://www.rfc-editor.org/rfc/rfc8610

// grammar: https://www.rfc-editor.org/rfc/rfc8610#appendix-B

#[derive(Debug)]
pub enum Rule {
    Group {
        name: String,
        group: (Option<Occur>, Group),
    },
    Type {
        name: String,
        r#type: Type,
    },
}

#[derive(Debug)]
pub enum Group {
    And(Vec<(Option<Occur>, Group)>),
    Or(Vec<Group>),
    KeyValue(Option<Key>, Type),
    Name(String),
}

#[derive(Debug)]
pub enum Type {
    Value(Value),
    Typename(String),
    Combined {
        operator: Operator,
        first: Box<Type>,
        second: Box<Type>,
    },
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
    trace("assignt", alt(("=", "/=")).map(|v| 1)).parse_next(input)
}

fn assigng(input: &mut &str) -> ModalResult<usize> {
    trace("assigng", alt(("=", "//=")).map(|v| 1)).parse_next(input)
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
    trace("type1", (type2, opt((s, alt((rangeop, ctlop)), s, type2))))
        .map(|(first, second)| {
            if let Some(((), operator, (), second)) = second {
                Type::Combined {
                    operator,
                    first: Box::new(first),
                    second: Box::new(second),
                }
            } else {
                first
            }
        })
        .parse_next(input)
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

#[derive(Debug)]
pub enum Operator {
    Range,
    Control(String),
}

fn rangeop(input: &mut &str) -> ModalResult<Operator> {
    alt(("...", ".."))
        .map(|v| Operator::Range)
        .parse_next(input)
}

fn ctlop(input: &mut &str) -> ModalResult<Operator> {
    preceded(".", id)
        .map(|v| Operator::Control(v.to_owned()))
        .parse_next(input)
}

fn group(input: &mut &str) -> ModalResult<Group> {
    trace(
        "group",
        (grpchoice, repeat(0.., preceded((s, "//", s), grpchoice))).map(
            |(first, mut rest): (_, Vec<_>)| {
                if rest.is_empty() {
                    first
                } else {
                    rest.insert(0, first);
                    Group::Or(rest)
                }
            },
        ),
    )
    .parse_next(input)
}

fn grpchoice(input: &mut &str) -> ModalResult<Group> {
    trace("grpchoice", repeat(0.., terminated(grpent, optcom)))
        .map(|v| Group::And(v))
        .parse_next(input)
}

fn grpent(input: &mut &str) -> ModalResult<(Option<Occur>, Group)> {
    let mut a =
        (opt(terminated(memberkey, s)), r#type).map(|(key, value)| Group::KeyValue(key, value));
    let mut b = groupname.map(|v| Group::Name(v.to_owned())); // not complete
    let mut c = terminated(preceded(("(", s), group), (s, ")"));
    trace("grpent", (opt(terminated(occur, s)), alt((a, b, c)))).parse_next(input)
}

#[derive(Debug)]
pub enum Key {
    Type(Type),
    Value(Value),
    Literal(String),
}

fn memberkey(input: &mut &str) -> ModalResult<Key> {
    let mut a = terminated(type1, (s, "=>")).map(|v| Key::Type(v)); // not complete
    let mut b = terminated(bareword, (s, ":")).map(|v| Key::Literal(v.to_owned()));
    let mut c = terminated(value, (s, ":")).map(|v| Key::Value(v));
    trace("memberkey", alt((a, b, c))).parse_next(input)
}

fn bareword<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    trace("bareword", id).parse_next(input)
}

fn optcom(input: &mut &str) -> ModalResult<()> {
    trace("optcom", (s, opt((",", s))).map(|v| ())).parse_next(input)
}

#[derive(Debug, PartialEq)]
pub enum Occur {
    RangeInclusive(RangeInclusive<usize>),
    RangeFrom(RangeFrom<usize>),
}

fn occur(input: &mut &str) -> ModalResult<Occur> {
    // TODO dec_uint not fully correct
    trace(
        "occur",
        alt((
            (opt(dec_uint), "*", opt(dec_uint)).map(
                |(l, _, u): (Option<usize>, _, Option<usize>)| match (l, u) {
                    (Some(l), Some(u)) => Occur::RangeInclusive(l..=u),
                    (Some(l), None) => Occur::RangeFrom(l..),
                    (None, Some(u)) => Occur::RangeInclusive(0..=u),
                    (None, None) => Occur::RangeFrom(0..),
                },
            ),
            "+".map(|v| Occur::RangeFrom(1..)),
            "?".map(|v| Occur::RangeInclusive(0..=1)),
        )),
    )
    .parse_next(input)
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(String),
}

fn value(input: &mut &str) -> ModalResult<Value> {
    trace(
        "value",
        alt((
            take_while(1.., ('0'..='9', 'e', '.', '-', '+'))
                .map(|v: &str| Value::Number(v.to_owned())),
            seq!(_: "\"", take_until(0.., "\""), _: "\"")
                .map(|(v,): (&str,)| Value::String(v.to_owned())),
        )),
    )
    .parse_next(input)
}

fn id<'a>(s: &mut &'a str) -> ModalResult<&'a str> {
    trace(
        "id",
        take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '-', '.')),
    )
    .parse_next(s)
}

fn s(input: &mut &str) -> ModalResult<()> {
    repeat(
        0..,
        alt((multispace1.map(|v| ()), (";", till_line_ending).map(|v| ()))),
    )
    .parse_next(input)
}

pub fn codegen(rules: &[Rule]) -> String {
    let rules = rules.iter().map(codegen_rule);
    let rules = quote! {
        #(#rules)*
    };
    let syntax_tree = syn::parse2(rules).unwrap();
    prettyplease::unparse(&syntax_tree)
}

fn codegen_rule(rule: &Rule) -> proc_macro2::TokenStream {
    match rule {
        Rule::Group { name, group } => {
            let name = format_ident!("{}", name.to_upper_camel_case());
            assert_eq!(group.0, None);
            let group = &group.1;
            let inner = match group {
                Group::And(items) => quote! { TODO },
                Group::Or(groups) => quote! { TODO },
                Group::KeyValue(key, r#type) => {
                    let key = key.as_ref().unwrap();
                    let key = match key {
                        Key::Type(_) => quote! { TODO },
                        Key::Value(value) => quote! { TODO },
                        Key::Literal(literal) => {
                            let key = format_ident!("{}", literal);
                            quote! {
                                #key
                            }   
                        },
                    };
                    let r#type = quote! { TODO };
                    quote! {
                        pub #key: #r#type
                    }
                },
                Group::Name(name) => {
                    let name_snake = format_ident!("{}", name.to_snake_case());
                    let name = format_ident!("{}", name.to_upper_camel_case());
                    quote! {
                        #[serde(flatten)]
                        pub #name_snake: #name
                    }
                },
            };
            quote! {
                pub struct #name {
                    #inner
                }
            }
        }
        Rule::Type { name, r#type } => {
            let name = format_ident!("{}", name.to_upper_camel_case());
            quote! {
                pub struct #name {}
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
    fn it_works() {
        let input = read_to_string("../../webdriver-bidi/all.cddl").unwrap();
        let parsed = ccdl.parse(&input).unwrap();
        println!("{parsed:#?}");
        let code = codegen(&parsed);
        panic!("{code}");
    }
}
