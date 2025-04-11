use winnow::{
    ascii::{alpha1, dec_uint, multispace0 as s},
    combinator::{alt, cut_err, dispatch, fail, opt, preceded, repeat, terminated},
    error::{StrContext, StrContextValue},
    prelude::*,
    token::{take, take_until, take_while},
};

// https://www.rfc-editor.org/rfc/rfc8610

// grammar: https://www.rfc-editor.org/rfc/rfc8610#appendix-B

fn ccdl(input: &mut &str) -> ModalResult<usize> {
    preceded(s, repeat(1.., (terminated(rule, s)))).map(|v: Vec<_>| 1).parse_next(input)
}

fn rule(input: &mut &str) -> ModalResult<usize> {
    alt(((typename, s, assignt, s, r#type), (groupname, s, assigng, s, grpent))).map(|v| 1).parse_next(input)
}

fn typename(input: &mut &str) -> ModalResult<usize> {
    id.map(|v| 1).parse_next(input)
}

fn groupname(input: &mut &str) -> ModalResult<usize> {
    id.map(|v| 1).parse_next(input)
}

fn assignt(input: &mut &str) -> ModalResult<usize> {
    alt(("=", "/=")).map(|v| 1).parse_next(input)
}

fn assigng(input: &mut &str) -> ModalResult<usize> {
    alt(("=", "//=")).map(|v| 1).parse_next(input)
}

fn r#type(input: &mut &str) -> ModalResult<usize> {
    (type1, repeat(0.., (s, "/", s, type1))).map(|v: (_, Vec<_>)| 1).parse_next(input)
}

fn type1(input: &mut &str) -> ModalResult<usize> {
    // TODO not complete
    type2.parse_next(input)
}

fn type2(input: &mut &str) -> ModalResult<usize> {
    // TODO not complete
    alt((value, typename, ("(", s, r#type, s, ")").map(|v| 1), ("{", s, group, s, "}").map(|v| 1), ("[", s, group, s, "]").map(|v| 1))).parse_next(input)
}

fn group(input: &mut &str) -> ModalResult<usize> {
    (grpchoice, repeat(0.., (s, "//", s, grpchoice))).map(|v: (_, Vec<_>)| 1).parse_next(input)
}

fn grpchoice(input: &mut &str) -> ModalResult<usize> {
    repeat(0.., (grpent, optcom)).parse_next(input)
}

fn grpent(input: &mut &str) -> ModalResult<usize> {
    let mut a = (opt(terminated(occur, s)), opt(terminated(memberkey, s)), r#type).map(|v| 1usize);
    let mut b = (opt(terminated(occur, s)), groupname).map(|v| 1usize); // not complete
    let mut c = (opt(terminated(occur, s)), "(", s, group, s, ")").map(|v| 1usize);
    alt((a, b, c)).parse_next(input)
}

fn memberkey(input: &mut &str) -> ModalResult<usize> {
    let mut a = terminated(type1, s).map(|v| 1usize); // not complete
    let mut b = (bareword, s, ":").map(|v| 1usize);
    let mut c = (value, s, ":").map(|v| 1usize);
    alt((a, b, c)).parse_next(input)
}

fn bareword(input: &mut &str) -> ModalResult<usize> {
    id.map(|v| 1).parse_next(input)
}

fn optcom(input: &mut &str) -> ModalResult<usize> {
    (s, opt((",", s))).map(|v| 1).parse_next(input)
}

fn occur(input: &mut &str) -> ModalResult<usize> {
    // TODO dec_uint not fully correct
    alt(((dec_uint, "*", dec_uint).map(|v: (u64, _, u64)| 1), "+".map(|v| 1), "?".map(|v| 1))).parse_next(input)
}

fn value(input: &mut &str) -> ModalResult<usize> {
    alt((dec_uint.map(|v: u64| 1), ("\"", take_until(0.., "\""), "\"").map(|v| 1))).parse_next(input)
}

fn id<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
    take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '-')).parse_next(s)
}

/*
fn parse_name(input: &mut &str) -> ModalResult<usize> {
    (alpha1, multispace0, "=", multispace0).map(|v| 1).context(StrContext::Label("name")).parse_next(input)
}

fn ident<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
    take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '-')).context(StrContext::Label("ident")).parse_next(s)
}

fn parse_entry(input: &mut &str) -> ModalResult<usize> {
    (multispace0, alpha1, opt((": ", ident)), opt(","), multispace0).map(|v| 1).context(StrContext::Label("entry")).parse_next(input)
}

fn parse_group(input: &mut &str) -> ModalResult<usize> {
    (
        multispace0,
        parse_name,
        dispatch! {
            take(1usize);
            "{" => (multispace0, repeat(0.., parse_entry), multispace0, "}").map(|_: (_, Vec<_>, _, _)| 1),
            "(" => (multispace0, repeat(0.., (ident, multispace0, opt(alt(("//", "/"))), multispace0)), ")").map(|_: (_, Vec<_>, _)| 1),
            _ => (fail::<_, usize, _>).context(StrContext::Label("group open"))
            .context(StrContext::Expected(StrContextValue::StringLiteral("{")))
            .context(StrContext::Expected(StrContextValue::StringLiteral("("))),
        },
    )
        // .context(StrContext::Label("group"))
        .map(|_| 1)
        .parse_next(input)
}*/

#[derive(Debug, PartialEq, Eq)]
pub struct Test(usize);

impl std::str::FromStr for Test {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        ccdl.map(|_| Test(1)).parse(input).map_err(|e| anyhow::format_err!("{e}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::Test;
    use std::fs::read_to_string;

    #[test]
    fn it_works() {
        let input = read_to_string("../../webdriver-bidi/all.cddl").unwrap();
        let parsed = input.parse::<Test>().unwrap();
        panic!("{parsed:?}");
    }
}
