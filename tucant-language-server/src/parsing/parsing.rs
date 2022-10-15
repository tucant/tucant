use std::{
    iter::{Copied, Enumerate},
    marker::PhantomData,
    ops::{Range, RangeBounds, RangeFrom},
    slice::Iter,
};

use log::trace;
use nom::{
    character::{
        complete::{self, one_of},
        is_digit,
    },
    combinator::{map_res, recognize},
    error::{ContextError, ParseError, context},
    multi::{many0, many1},
    sequence::{preceded, terminated},
    AsChar, Compare, IResult, InputIter, InputLength, InputTake, Needed, Slice, bytes::complete::is_a, InputTakeAtPosition,
};
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Span<T: Debug> {
    inner: T,
    start: usize,
    line_start: usize,
    column_start: usize,
    end: usize,
    line_end: usize,
    column_end: usize,
}

impl<'a> Debug for Span<&'a str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "location:{}:{} - location:{}:{}\n{}",
            self.line_start,
            self.column_start,
            self.line_end,
            self.column_end,
            &self.inner[self.start..self.end]
        )
    }
}

impl<'a> Span<&'a str> {
    fn new(input: &'a str) -> Self {
        let last_line_pos = input
            .lines()
            .enumerate()
            .last()
            .map_or((0, 0), |(index, last_line)| (index, last_line.len()));
        Self {
            inner: input,
            start: 0,
            line_start: 0,
            column_start: 0,
            end: input.len(),
            line_end: last_line_pos.0,
            column_end: last_line_pos.1,
        }
    }
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
        P: Fn(Self::Item) -> bool,
    {
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
        (
            Self {
                inner: self.inner,
                start: self.start,
                line_start: self.line_start,
                column_start: self.column_start,
                end: self.start + count,
                line_end: usize::MAX,
                column_end: usize::MAX,
            },
            Self {
                inner: self.inner,
                start: self.start + count,
                line_start: usize::MAX,
                column_start: usize::MAX,
                end: self.end,
                line_end: self.line_end,
                column_end: self.column_end,
            },
        )
    }
}

impl<'a> InputLength for Span<&'a str> {
    fn input_len(&self) -> usize {
        todo!()
    }
}

impl<'a> Compare<&'a str> for Span<&'a str> {
    fn compare(&self, t: &'a str) -> nom::CompareResult {
        trace!("compare {:?} {:?}", self, t);
        if self.inner[self.start..self.end].starts_with(t) {
            nom::CompareResult::Ok
        } else {
            nom::CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: &'a str) -> nom::CompareResult {
        todo!()
    }
}

impl<'a> InputTakeAtPosition for Span<&'a str> {
    type Item = char;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool {
        todo!()
    }

    fn split_at_position1<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: nom::error::ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool {
        todo!()
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool {
        todo!()
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: nom::error::ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool {
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

#[derive(Debug)]
pub struct MyError<'a> {
    location: Span<&'a str>,
    kind: nom::error::ErrorKind,
    context: &'a str,
}

impl<'a> ParseError<Span<&'a str>> for MyError<'a> {
    fn from_error_kind(input: Span<&'a str>, kind: nom::error::ErrorKind) -> Self {
        Self {
            location: input,
            kind,
            context: ""
        }
    }

    fn append(input: Span<&str>, kind: nom::error::ErrorKind, other: Self) -> Self {
        todo!()
    }

    fn from_char(input: Span<&'a str>, _: char) -> Self {
        todo!()
    }

    fn or(self, other: Self) -> Self {
        todo!()
    }
}

impl<'a> ContextError<Span<&'a str>> for MyError<'a> {
    fn add_context(input: Span<&'a str>, ctx: &'static str, other: Self) -> Self {
        Self {
            location: input, kind: other.kind, context: ctx
        }
    }
}

// https://github.com/Geal/nom/blob/main/examples/string.rs
// https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md
// https://iximiuz.com/en/posts/rust-writing-parsers-with-nom/
// https://docs.rs/nom/latest/nom/recipes/index.html
// https://github.com/Geal/nom/blob/main/doc/error_management.md
// https://github.com/Geal/nom/blob/main/doc/custom_input_types.md
// https://github.com/Geal/nom/blob/main/examples/s_expression.rs
fn parse_number(input: Span<&str>) -> IResult<Span<&str>, Span<&str>, MyError> {
    context(r#"expected "test""#, tag("test"))(input)
}

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// RUST_LOG=trace cargo watch -x 'test -- --nocapture test_parse_number'
#[test]
fn test_parse_number() {
    init();
    /*let span = Span::new(r#"
    (this is an (epic awesome great) "test" 5)
    "#);*/
    let span = Span::new(r#"test fdsf"#);

    let result = parse_number(span);

    println!("{:?}", result);

    let span = Span::new(r#"notest fdsf"#);

    let result = parse_number(span);

    println!("{:?}", result);
}
