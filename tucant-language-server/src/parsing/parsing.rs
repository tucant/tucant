use std::{marker::PhantomData, ops::RangeFrom, iter::{Enumerate, Copied}, slice::Iter};

use nom::{IResult, error::ParseError, Slice, InputIter, Needed, AsChar, combinator::{map_res, recognize}, sequence::{preceded, terminated}, multi::{many1, many0}, character::{complete::{one_of, self}, is_digit}, bytes::complete::tag, InputTake, InputLength, Compare};

#[derive(Clone, Copy)]
pub struct Span<T> {
    line: u32,
    column: u32,
    inner: T,
}

pub struct MyInput<'a> {
    input: &'a [u8],
}

impl<'a> Slice<RangeFrom<usize>> for MyInput<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        Self {
            input: &self.input[range]
        }
    }
}

impl<'a> InputIter for MyInput<'a> {
    type Item = Span<u8>;

    type Iter = Enumerate<Self::IterElem>;

    type IterElem = Copied<Iter<'a, Span<u8>>>;

    fn iter_indices(&self) -> Self::Iter {
        self.input.iter_elements().enumerate();
        todo!()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.input.iter().copied();
        todo!()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool {
        //self.input.iter().position(|b| predicate(*b));
        todo!()
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.input.len() >= count {
            Ok(count)
          } else {
            Err(Needed::new(count - self.input.len()))
          }
    }
}


impl<'a> InputTake for MyInput<'a> {
    fn take(&self, count: usize) -> Self {
        todo!()
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        todo!()
    }
}

impl<'a> InputLength for MyInput<'a> {
    fn input_len(&self) -> usize {
        todo!()
    }
}

impl<'a> Compare<&'a str> for MyInput<'a> {
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

pub struct MyError<I> {
    data: PhantomData<I>
}

impl<I> ParseError<I> for MyError<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Self {
            data: PhantomData,
        }
    }

    fn append(input: I, kind: nom::error::ErrorKind, other: Self) -> Self {
        Self {
            data: PhantomData,
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
fn parse_number(input: MyInput) -> IResult<MyInput, MyInput> {
    tag("test")(input)
}

pub struct Parser {

}

impl Parser {

}