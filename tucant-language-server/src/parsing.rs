use std::{marker::PhantomData, ops::RangeFrom, iter::{Enumerate, Copied}, slice::Iter};

use nom::{IResult, error::ParseError, Slice, InputIter, Needed};

pub struct Span {
    line: u32,
    column: u32,
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
    type Item = u8;

    type Iter = Enumerate<Self::IterElem>;

    type IterElem = Copied<Iter<'a, u8>>;

    fn iter_indices(&self) -> Self::Iter {
        self.input.iter_elements().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.input.iter().copied()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool {
        self.input.iter().position(|b| predicate(*b))
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.input.len() >= count {
            Ok(count)
          } else {
            Err(Needed::new(count - self.input.len()))
          }
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
fn parse_string(input: MyInput) -> IResult<MyInput, char, MyError<MyInput>> {
    nom::character::complete::char('"')(input)
}

pub struct Parser {

}

impl Parser {

}