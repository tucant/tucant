use std::{fmt::Debug, str::CharIndices};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Clone, Copy)]
pub struct Span<'a, T: Debug> {
    inner: T,
    full_string: &'a str, // TODO this could become a ref to a struct for the whole file with name info etc
    string: &'a str,
}

#[derive(Debug)]
pub struct Error<'a> {
    location: Span<'a, ()>,
    reason: &'static str,
}

fn offset_to_line_column<'a, T: Debug>(span: &Span<'a, T>, string: &str) -> (usize, usize) {
    span.full_string[..(string.as_ptr() as usize - span.full_string.as_ptr() as usize)]
        .lines()
        .enumerate()
        .last()
        .map_or((0, 0), |(index, last_line)| (index, last_line.len()))
}

impl<'a, T: Debug> Debug for Span<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_pos = offset_to_line_column(self, self.string);
        let end_pos = offset_to_line_column(self, &self.string[self.string.len()..]);
        write!(
            f,
            "location:{}:{} - location:{}:{}\n{}\nvalue:{:?}",
            start_pos.0, start_pos.1, end_pos.0, end_pos.1, &self.string, self.inner
        )
    }
}

impl<'a> Span<'a, ()> {
    fn new(input: &'a str) -> Self {
        Self {
            inner: (),
            full_string: input,
            string: input,
        }
    }
}

impl<'a> Into<&'a str> for Span<'a, ()> {
    fn into(self) -> &'a str {
        &self.string
    }
}

fn parse_string<'a>(input: Span<'a, ()>) -> Result<(Span<&'a str>, Span<'a, ()>), Error> {
    let input_str = Into::<&'a str>::into(input);
    let mut it = input_str.char_indices();
    match it.next() {
        Some((_, '"')) => {}
        Some((_, character)) => Err(Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input_str[0..character.len_utf8()],
            },
            reason: r#"Expected a `"`"#,
        })?,
        None => Err(Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input_str[0..0],
            },
            reason: r#"Unexpected end of code. Expected a `"`"#,
        })?,
    };
    match it
        .skip_while(|(_, character)| *character != '"')
        .skip(1)
        .map(|(offset, _)| offset)
        .next()
    {
        Some(offset) => {
            let (str_str, rest_str) = input_str.split_at(offset);
            Ok((Span {
                inner: str_str.trim_matches('"'),
                full_string: input.full_string,
                string: str_str,
            }, Span {
                inner: (),
                full_string: input.full_string,
                string: rest_str,
            }))
        },
        None => Err(Error {
            location: input,
            reason: r#"Unterminated string literal"#,
        })?,
    }
}

// https://doc.rust-lang.org/book/ch08-02-strings.html
fn parse_number<'a>(input: Span<'a, ()>) -> Result<(Span<i64>, Span<'a, ()>), Error> {
    let input_str: &'a str = input.into();
    let end_of_numbers = input_str
        .char_indices()
        .skip_while(|(_, character)| character.is_ascii_digit())
        .map(|(offset, _)| offset)
        .next()
        .unwrap_or(input_str.len()); // TODO FIXME different error message
    let (number_str, rest_str) = input_str.split_at(end_of_numbers);
    Ok((Span {
        inner: number_str.parse::<i64>().map_err(|_| Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: number_str,
            },
            reason: r#"Failed to parse number"#,
        })?,
        full_string: input.full_string,
        string: number_str,
    }, Span {
        inner: (),
        full_string: input.full_string,
        string: rest_str,
    }))
}

fn parse_atom<'a>(input: Span<&'a str>) {}

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
    let span = Span::new(r#"notanumber"#);
    let number = parse_number(span);
    println!("{:?}", number);

    let span = Span::new(r#"3notendingwithanumber"#);
    let number = parse_number(span);
    println!("{:?}", number);

    let span = Span::new(r#"3"#);
    let number = parse_number(span);
    println!("{:?}", number);

    let span = Span::new(r#"3z9"#);
    let number = parse_number(span);
    println!("{:?}", number);

    let span = Span::new(r#"3546z945"#);
    let number = parse_number(span);
    println!("{:?}", number);

    let span = Span::new(r#"345345"#);
    let number = parse_number(span);
    println!("{:?}", number);

    let span = Span::new(r#"345345sdfasd"#);
    let number = parse_number(span);
    println!("{:?}", number);

    let span = Span::new(r#"n32otanumber"#);
    let number = parse_number(span);
    println!("{:?}", number);

    let span = Span::new(r#"70708777897986976707598759785978698752otanumber"#);
    let number = parse_number(span);
    println!("{:?}", number);
}

// RUST_LOG=trace cargo watch -x 'test -- --nocapture test_parse_string'
#[test]
fn test_parse_string() {
    init();

    let span = Span::new(r#"notastring"#);
    let string = parse_string(span);
    println!("{:?}", string);

    let span = Span::new(r#""unterminated"#);
    let string = parse_string(span);
    println!("{:?}", string);

    let span = Span::new(r#""astring"jojo"#);
    let string = parse_string(span);
    println!("{:?}", string);

    let span = Span::new(r#""astring""#);
    let string = parse_string(span);
    println!("{:?}", string);
}