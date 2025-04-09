// https://www.rfc-editor.org/rfc/rfc8610

use winnow::Parser;
use winnow::Result;
use winnow::ascii::digit1;
use winnow::combinator::alt;
use winnow::error::{StrContext, StrContextValue};

fn parse_digits(input: &mut &str) -> Result<usize> {
    digit1.parse_to().parse_next(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Test(usize);

impl std::str::FromStr for Test {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_digits.map(|_| Test(1)).parse(input).map_err(|e| anyhow::format_err!("{e}"))
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
