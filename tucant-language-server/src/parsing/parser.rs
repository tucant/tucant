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

// alternative: yield start and end of character?
fn old_my_char_indices<'a>(input: &'a str) -> impl Iterator<Item = (usize, Option<char>)> + 'a {
    input.char_indices().map(|(offset, character)| (offset, Some(character))).chain(std::iter::once((input.len(), None)))
}

fn my_char_indices<'a>(input: &'a str) -> impl Iterator<Item = (usize, char, usize)> + 'a {
    input.char_indices().map(|(offset, character)| (offset, character, offset + character.len_utf8()))
}

fn parse_string<'a>(input: Span<'a, ()>) -> Result<(Span<'a, &'a str>, Span<'a, ()>), Error> {
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
fn parse_number<'a>(input: Span<'a, ()>) -> Result<(Span<'a, i64>, Span<'a, ()>), Error> {
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

fn parse_identifier<'a>(input: Span<'a, ()>) -> Result<(Span<'a, &'a str>, Span<'a, ()>), Error> {
    // https://github.com/rust-lang/rust/issues/62208
    let input_str: &'a str = input.into();
    let end = my_char_indices(input_str)
        .take_while(|(_, character, _)| character.is_ascii_alphabetic())
        .map(|(_, _, end)| end)
        .last().ok_or_else(|| Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input.string[0..0],
            },
            reason: "Expected an identifier",
        })?;
    let (str_str, rest_str) = input_str.split_at(end);
    Ok((Span {
        inner: str_str,
        full_string: input.full_string,
        string: str_str,
    }, Span {
        inner: (),
        full_string: input.full_string,
        string: rest_str,
    }))
}

/*/
fn parse_list<'a>(input: Span<'a, ()>) -> Result<(Span<'a, i64>, Span<'a, ()>), Error> {

}
*/

#[derive(Debug)]
pub enum AST<'a> {
    Number(i64),
    String(&'a str),
    Identifier(&'a str),
    List(Vec<Span<'a, AST<'a>>>),
}

fn parse_root<'a>(input: Span<'a, ()>) -> Result<(Span<'a, AST<'a>>, Span<'a, ()>), Error> {
    let input_str = Into::<&'a str>::into(input);
    let mut it = my_char_indices(input_str);
    match it.next() {
        Some((_, '"', _)) => parse_string(input).and_then(|v| {
            Ok((Span {
                inner: AST::String(v.0.inner),
                full_string: v.0.full_string,
                string: v.0.string,
            }, v.1))
        }),
        Some((_, '0' ..= '9', _)) => parse_number(input).and_then(|v| {
            Ok((Span {
                inner: AST::Number(v.0.inner),
                full_string: v.0.full_string,
                string: v.0.string,
            }, v.1))
        }),
        Some((_, 'a' ..= 'z' | 'A' ..= 'Z', _)) => parse_identifier(input).and_then(|v| {
            Ok((Span {
                inner: AST::String(v.0.inner),
                full_string: v.0.full_string,
                string: v.0.string,
            }, v.1))
        }),
        Some((_, '(', _)) => todo!(),
        Some((start, _, end)) => Err(Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input.string[start..end],
            },
            reason: "Expected an identifier",
        }),
        None => todo!(),
    }
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
    let span = Span::new(r#"notanumber"#);
    let number = parse_number(span).unwrap_err();
    println!("{:?}", number);
    assert_eq!(number.reason, "Failed to parse number");
    assert_eq!(number.location.string, "");

    let span = Span::new(r#"3notendingwithanumber"#);
    let number = parse_number(span).unwrap();
    println!("{:?}", number);
    assert_eq!(number.0.inner, 3);
    assert_eq!(number.0.string, "3");
    assert_eq!(number.1.string, "notendingwithanumber");

    let span = Span::new(r#"3"#);
    let number = parse_number(span).unwrap();
    println!("{:?}", number);
    assert_eq!(number.0.inner, 3);
    assert_eq!(number.0.string, "3");
    assert_eq!(number.1.string, "");

    let span = Span::new(r#"3z9"#);
    let number = parse_number(span).unwrap();
    println!("{:?}", number);
    assert_eq!(number.0.inner, 3);
    assert_eq!(number.0.string, "3");
    assert_eq!(number.1.string, "z9");

    let span = Span::new(r#"3546z945"#);
    let number = parse_number(span).unwrap();
    println!("{:?}", number);
    assert_eq!(number.0.inner, 3546);
    assert_eq!(number.0.string, "3546");
    assert_eq!(number.1.string, "z945");

    let span = Span::new(r#"345345"#);
    let number = parse_number(span).unwrap();
    println!("{:?}", number);
    assert_eq!(number.0.inner, 345345);
    assert_eq!(number.0.string, "345345");
    assert_eq!(number.1.string, "");

    let span = Span::new(r#"345345sdfasd"#);
    let number = parse_number(span).unwrap();
    println!("{:?}", number);
    assert_eq!(number.0.inner, 345345);
    assert_eq!(number.0.string, "345345");
    assert_eq!(number.1.string, "sdfasd");

    let span = Span::new(r#"n32otanumber"#);
    let number = parse_number(span).unwrap_err();
    println!("{:?}", number);
    assert_eq!(number.reason, "Failed to parse number");
    assert_eq!(number.location.string, "");

    let span = Span::new(r#"70708777897986976707598759785978698752otanumber"#);
    let number = parse_number(span).unwrap_err();
    println!("{:?}", number);
    assert_eq!(number.reason, "Failed to parse number");
    assert_eq!(number.location.string, "70708777897986976707598759785978698752");
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

    // TODO FIXME
    let span = Span::new(r#""astring""#);
    let string = parse_string(span);
    println!("{:?}", string);
}

// RUST_LOG=trace cargo watch -x 'test -- --nocapture test_parse_identifier'
#[test]
fn test_parse_identifier() {
    init();

    let span = Span::new(r#"7notanidentifier"#);
    let string = parse_identifier(span);
    println!("{:?}", string);

    let span = Span::new(r#""notanidentifier"#);
    let string = parse_identifier(span);
    println!("{:?}", string);

    let span = Span::new(r#"anidentifier"#);
    let string = parse_identifier(span);
    println!("{:?}", string);

    let span = Span::new(r#"anidentifier    jlih"#);
    let string = parse_identifier(span);
    println!("{:?}", string);
}