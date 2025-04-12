use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive};

use winnow::{
    ascii::{alpha1, dec_uint, float, multispace0, multispace1, till_line_ending},
    combinator::{alt, cut_err, dispatch, fail, opt, preceded, repeat, seq, terminated, trace},
    error::{StrContext, StrContextValue},
    prelude::*,
    token::{take, take_until, take_while},
};

// https://www.rfc-editor.org/rfc/rfc8610

// grammar: https://www.rfc-editor.org/rfc/rfc8610#appendix-B

#[derive(Debug)]
pub enum Rule {
    Group { name: String, group: Group },
    Type { name: String, r#type: Type },
}

#[derive(Debug)]
pub struct Group {}

#[derive(Debug)]
pub enum Type {
    Value(Value),
    Typename(String),
    Combined {
        operator: Operator,
        first: Box<Type>,
        second: Box<Type>,
    },
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
        (type1, repeat(0.., (s, "/", s, type1))).map(|v: (_, Vec<_>)| todo!()),
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
            value.map(|v| Type::Value(v)),
            typename.map(|v| Type::Typename(v.to_owned())),
            ("(", s, r#type, s, ")").map(|v| todo!()),
            ("{", s, group, s, "}").map(|v| todo!()),
            ("[", s, group, s, "]").map(|v| todo!()),
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

fn group(input: &mut &str) -> ModalResult<usize> {
    trace(
        "group",
        (grpchoice, repeat(0.., (s, "//", s, grpchoice))).map(|v: (_, Vec<_>)| 1),
    )
    .parse_next(input)
}

fn grpchoice(input: &mut &str) -> ModalResult<usize> {
    trace("grpchoice", repeat(0.., (grpent, optcom))).parse_next(input)
}

fn grpent(input: &mut &str) -> ModalResult<Group> {
    let mut a = (opt(terminated(memberkey, s)), r#type).map(|v| 1usize);
    let mut b = groupname.map(|v| 1usize); // not complete
    let mut c = ("(", s, group, s, ")").map(|v| 1usize);
    trace(
        "grpent",
        (opt(terminated(occur, s)), alt((a, b, c))).map(|v| Group {}),
    )
    .parse_next(input)
}

fn memberkey(input: &mut &str) -> ModalResult<usize> {
    let mut a = (terminated(type1, s), "=>").map(|v| 1usize); // not complete
    let mut b = (bareword, s, ":").map(|v| 1usize);
    let mut c = (value, s, ":").map(|v| 1usize);
    trace("memberkey", alt((a, b, c))).parse_next(input)
}

fn bareword(input: &mut &str) -> ModalResult<usize> {
    trace("bareword", id.map(|v| 1)).parse_next(input)
}

fn optcom(input: &mut &str) -> ModalResult<usize> {
    trace("optcom", (s, opt((",", s))).map(|v| 1)).parse_next(input)
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use winnow::Parser;

    use crate::ccdl;

    #[test]
    fn it_works() {
        let input = read_to_string("../../webdriver-bidi/all.cddl").unwrap();
        let parsed = ccdl.parse(&input).unwrap();
        panic!("{parsed:#?}");
    }
}
