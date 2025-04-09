// https://www.rfc-editor.org/rfc/rfc8610

use winnow::ModalResult;
use winnow::Parser;
use winnow::Result;
use winnow::ascii::alpha1;
use winnow::ascii::digit1;
use winnow::ascii::multispace0;
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::separated;
use winnow::combinator::trace;
use winnow::error::ParserError;
use winnow::error::{StrContext, StrContextValue};
use winnow::stream::AsChar;
use winnow::stream::Stream;
use winnow::stream::StreamIsPartial;
use winnow::token::take_while;

fn parse_name(input: &mut &str) -> ModalResult<usize> {
    (alpha1, " = {\n").map(|v| 1).context(StrContext::Label("name")).parse_next(input)
}

fn ident<'i>(s: &mut &'i str) -> ModalResult<&'i str> {
    take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '-')).context(StrContext::Label("ident")).parse_next(s)
}

fn parse_entry(input: &mut &str) -> ModalResult<usize> {
    (multispace0, alpha1, ": ", ident).map(|v| 1).context(StrContext::Label("entry")).parse_next(input)
}

fn parse_group(input: &mut &str) -> ModalResult<usize> {
    (parse_name, separated(0.., cut_err(parse_entry), ",")).context(StrContext::Label("group")).map(|(v, a): (usize, Vec<_>)| 1).parse_next(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Test(usize);

impl std::str::FromStr for Test {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_group.map(|_| Test(1)).parse(input).map_err(|e| anyhow::format_err!("{e}"))
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
