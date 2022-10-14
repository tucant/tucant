use std::{marker::PhantomData, ops::{RangeFrom, RangeBounds, Range}, iter::{Enumerate, Copied}, slice::Iter};

use nom::{IResult, error::ParseError, Slice, InputIter, Needed, AsChar, combinator::{map_res, recognize}, sequence::{preceded, terminated}, multi::{many1, many0}, character::{complete::{one_of, self}, is_digit}, bytes::complete::tag, InputTake, InputLength, Compare};

#[derive(Clone, Copy)]
pub struct Span<T> {
    inner: T,
    start: usize,
    line_start: usize,
    column_start: usize,
    end: usize,
    line_end: usize,
    column_end: usize,
}

impl<'a> Slice<RangeFrom<usize>> for Span<&'a str> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        todo!()
    }
}

impl<'a> InputIter for Span<&'a str> {
    type Item = Span<char>;

    type Iter = Enumerate<Self::IterElem>;

    type IterElem = Copied<Iter<'a, Span<char>>>;

    fn iter_indices(&self) -> Self::Iter {
        todo!()
    }

    fn iter_elements(&self) -> Self::IterElem {
        todo!()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool {
        todo!()
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        todo!()
    }
}


impl<'a> InputTake for Span<&'a str> {
    fn take(&self, count: usize) -> Self {
        todo!()
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        todo!()
    }
}

impl<'a> InputLength for Span<&'a str> {
    fn input_len(&self) -> usize {
        todo!()
    }
}

impl<'a> Compare<&'a str> for Span<&'a str> {
    fn compare(&self, t: &'a str) -> nom::CompareResult {
        todo!()
    }

    fn compare_no_case(&self, t: &'a str) -> nom::CompareResult {
        todo!()
    }
}

impl AsChar for Span<u8> {
    fn as_char(self) -> char {
        self.inner.as_char()
    }

    fn is_alpha(self) -> bool {
        self.inner.is_alpha()
    }

    fn is_alphanum(self) -> bool {
        self.inner.is_alphanum()
    }

    fn is_dec_digit(self) -> bool {
        self.inner.is_dec_digit()
    }

    fn is_hex_digit(self) -> bool {
        self.inner.is_hex_digit()
    }

    fn is_oct_digit(self) -> bool {
        self.inner.is_oct_digit()
    }

    fn len(self) -> usize {
        self.inner.len()
    }
}

pub struct MyError {

}

impl ParseError<Span<&str>> for MyError {
    fn from_error_kind(input: Span<&str>, kind: nom::error::ErrorKind) -> Self {
        todo!()
    }

    fn append(input: Span<&str>, kind: nom::error::ErrorKind, other: Self) -> Self {
        todo!()
    }
}

// https://github.com/Geal/nom/blob/main/examples/string.rs
// https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md
// https://iximiuz.com/en/posts/rust-writing-parsers-with-nom/
// https://docs.rs/nom/latest/nom/recipes/index.html
// https://github.com/Geal/nom/blob/main/doc/error_management.md
// https://github.com/Geal/nom/blob/main/doc/custom_input_types.md
// https://github.com/Geal/nom/blob/main/examples/s_expression.rs
fn parse_number(input: Span<&str>) -> IResult<Span<&str>, Span<&str>> {
    tag("test")(input)
}

pub struct Parser {

}

impl Parser {

}