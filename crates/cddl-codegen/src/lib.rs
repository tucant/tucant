use winnow::{
    ascii::{alpha1, dec_uint, multispace0 as s},
    combinator::{alt, cut_err, dispatch, fail, opt, preceded, repeat, terminated, trace},
    error::{StrContext, StrContextValue},
    prelude::*,
    token::{take, take_until, take_while},
};

// https://www.rfc-editor.org/rfc/rfc8610

// grammar: https://www.rfc-editor.org/rfc/rfc8610#appendix-B

fn ccdl(input: &mut &str) -> ModalResult<usize> {
    trace("ccdl", preceded(s, repeat(1.., (terminated(rule, s)))).map(|v: Vec<_>| 1)).parse_next(input)
}

fn rule(input: &mut &str) -> ModalResult<usize> {
    trace("rule", alt(((typename, s, assignt, s, r#type), (groupname, s, assigng, s, grpent))).map(|v| 1)).parse_next(input)
}

fn typename(input: &mut &str) -> ModalResult<usize> {
    trace("typename", id.map(|v| 1)).parse_next(input)
}

fn groupname(input: &mut &str) -> ModalResult<usize> {
    trace("groupname", id.map(|v| 1)).parse_next(input)
}

fn assignt(input: &mut &str) -> ModalResult<usize> {
    trace("assignt", alt(("=", "/=")).map(|v| 1)).parse_next(input)
}

fn assigng(input: &mut &str) -> ModalResult<usize> {
    trace("assigng", alt(("=", "//=")).map(|v| 1)).parse_next(input)
}

fn r#type(input: &mut &str) -> ModalResult<usize> {
    trace("type", (type1, repeat(0.., (s, "/", s, type1))).map(|v: (_, Vec<_>)| 1)).parse_next(input)
}

fn type1(input: &mut &str) -> ModalResult<usize> {
    trace("type1", (type2, opt((s, alt((rangeop, ctlop)), s, type2)))).map(|v| 1).parse_next(input)
}

fn type2(input: &mut &str) -> ModalResult<usize> {
    // TODO not complete
    trace("type2", alt((value, typename, ("(", s, r#type, s, ")").map(|v| 1), ("{", s, group, s, "}").map(|v| 1), ("[", s, group, s, "]").map(|v| 1)))).parse_next(input)
}

fn rangeop(input: &mut &str) -> ModalResult<usize> {
    alt(("...", "..")).map(|v| 1).parse_next(input)
}

fn ctlop(input: &mut &str) -> ModalResult<usize> {
    preceded(".", id).map(|v| 1).parse_next(input)
}

fn group(input: &mut &str) -> ModalResult<usize> {
    trace("group", (grpchoice, repeat(0.., (s, "//", s, grpchoice))).map(|v: (_, Vec<_>)| 1)).parse_next(input)
}

fn grpchoice(input: &mut &str) -> ModalResult<usize> {
    trace("grpchoice", repeat(0.., (grpent, optcom))).parse_next(input)
}

fn grpent(input: &mut &str) -> ModalResult<usize> {
    let mut a = (opt(terminated(memberkey, s)), r#type).map(|v| 1usize);
    let mut b = groupname.map(|v| 1usize); // not complete
    let mut c = ("(", s, group, s, ")").map(|v| 1usize);
    trace("grpent", (opt(terminated(occur, s)), alt((a, b, c))).map(|v| 1)).parse_next(input)
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

fn occur(input: &mut &str) -> ModalResult<usize> {
    // TODO dec_uint not fully correct
    trace("occur", alt(((opt(dec_uint), "*", opt(dec_uint)).map(|v: (Option<u64>, _, Option<u64>)| 1), "+".map(|v| 1), "?".map(|v| 1)))).parse_next(input)
}

fn value(input: &mut &str) -> ModalResult<usize> {
    trace("value", alt((dec_uint.map(|v: u64| 1), ("\"", take_until(0.., "\""), "\"").map(|v| 1)))).parse_next(input)
}

fn id<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
    trace("id", take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '-', '.'))).parse_next(s)
}

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
    fn test1() {
        let input = r#"SessionCommand = (
  session.End //
  session.New //
  session.Status //
  session.Subscribe //
  session.Unsubscribe
)"#;
        let parsed = input.parse::<Test>().unwrap();
    }

    #[test]
    fn it_works() {
        let input = read_to_string("../../webdriver-bidi/all.cddl").unwrap();
        let parsed = input.parse::<Test>().unwrap();
        panic!("{parsed:?}");
    }
}
