use std::{borrow::Cow, fmt::Debug};

use tucant_language_server_derive_output::{FoldingRange, Position};

// TODO FIXME tokenization in extra stage

#[derive(Clone, Copy)]
pub struct Span<'a, T: Debug> {
    pub inner: T,
    pub full_string: &'a str, // TODO this could become a ref to a struct for the whole file with name info etc
    pub string: &'a str,
}

impl<'a> From<Ast<'a>> for Span<'a, Ast<'a>> {
    fn from(ast: Ast<'a>) -> Self {
        let fake: &'static str = "fake";
        Self {
            full_string: fake,
            string: fake,
            inner: ast,
        }
    }
}

#[derive(Debug)]
pub struct Error<'a, T: Debug> {
    pub location: Span<'a, ()>,
    pub reason: Cow<'static, str>,
    pub partial_parse: T,
}

fn offset_to_line_column<'a, T: Debug>(span: &Span<'a, T>, string: &str) -> (usize, usize) {
    span.full_string[..(string.as_ptr() as usize - span.full_string.as_ptr() as usize)]
        .lines()
        .enumerate()
        .last()
        .map_or((0, 0), |(index, last_line)| (index, last_line.len()))
}

pub fn line_column_to_offset(string: &str, line: usize, column: usize) -> usize {
    let the_line = string.lines().nth(line).unwrap();
    let line_offset = the_line
        .char_indices()
        .nth(column)
        .map(|(offset, _)| offset)
        .unwrap_or(the_line.len());
    the_line.as_ptr() as usize - string.as_ptr() as usize + line_offset
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
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: (),
            full_string: input,
            string: input,
        }
    }
}

impl<'a, T: Debug> Span<'a, T> {
    pub fn start_line_column(&self) -> (usize, usize) {
        let start_pos = offset_to_line_column(self, self.string);
        (start_pos.0, start_pos.1)
    }

    pub fn end_line_column(&self) -> (usize, usize) {
        let end_pos = offset_to_line_column(self, &self.string[self.string.len()..]);
        (end_pos.0, end_pos.1)
    }
}

// TODO FIXME remove this as it just makes it less transparent
impl<'a> From<Span<'a, ()>> for &'a str {
    fn from(value: Span<'a, ()>) -> Self {
        value.string
    }
}

fn my_char_indices(input: &str) -> impl Iterator<Item = (usize, char, usize)> + '_ {
    input
        .char_indices()
        .map(|(offset, character)| (offset, character, offset + character.len_utf8()))
}

fn parse_string<'a>(
    input: Span<'a, ()>,
) -> Result<(Span<'a, &'a str>, Span<'a, ()>), Error<'a, Span<'_, Ast<'_>>>> {
    let input_str = Into::<&'a str>::into(input);
    let mut it = my_char_indices(input_str);
    match it.next() {
        Some((_, '"', _)) => {}
        Some((_, character, _)) => Err(Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input_str[0..character.len_utf8()],
            },
            reason: Cow::from(r#"Expected a `"`"#),
            partial_parse: Span {
                inner: Ast::String(""),
                full_string: input.full_string,
                string: &input_str[0..character.len_utf8()],
            },
        })?,
        None => Err(Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input_str[0..0],
            },
            reason: Cow::from(r#"Unexpected end of code. Expected a `"`"#),
            partial_parse: Span {
                inner: Ast::String(""),
                full_string: input.full_string,
                string: &input_str[0..0],
            },
        })?,
    };
    match it
        .skip_while(|(_, character, _)| *character != '"')
        .map(|(_, _, end)| end)
        .next()
    {
        Some(offset) => {
            let (str_str, rest_str) = input_str.split_at(offset);
            Ok((
                Span {
                    inner: str_str.trim_matches('"'),
                    full_string: input.full_string,
                    string: str_str,
                },
                Span {
                    inner: (),
                    full_string: input.full_string,
                    string: rest_str,
                },
            ))
        }
        None => Err(Error {
            location: input,
            reason: Cow::from(r#"Unterminated string literal"#),
            partial_parse: Span {
                inner: Ast::String(&input.string[1..]),
                full_string: input.full_string,
                string: input.string,
            },
        })?,
    }
}

// https://doc.rust-lang.org/book/ch08-02-strings.html
fn parse_number<'a>(
    input: Span<'a, ()>,
) -> Result<(Span<'a, i64>, Span<'a, ()>), Error<'a, Span<'_, Ast<'_>>>> {
    let input_str: &'a str = input.into();
    let end_of_numbers = input_str
        .char_indices()
        .skip_while(|(_, character)| character.is_ascii_digit())
        .map(|(offset, _)| offset)
        .next()
        .unwrap_or(input_str.len()); // TODO FIXME different error message
    let (number_str, rest_str) = input_str.split_at(end_of_numbers);
    Ok((
        Span {
            inner: number_str.parse::<i64>().map_err(|_| Error {
                location: Span {
                    inner: (),
                    full_string: input.full_string,
                    string: number_str,
                },
                reason: Cow::from(r#"Failed to parse number"#),
                partial_parse: Span {
                    inner: Ast::Number(1337),
                    full_string: input.full_string,
                    string: number_str,
                },
            })?,
            full_string: input.full_string,
            string: number_str,
        },
        Span {
            inner: (),
            full_string: input.full_string,
            string: rest_str,
        },
    ))
}

fn parse_identifier<'a>(
    input: Span<'a, ()>,
) -> Result<(Span<'a, &'a str>, Span<'a, ()>), Error<'a, Span<'_, Ast<'_>>>> {
    let input_str = Into::<&'a str>::into(input);
    let end = my_char_indices(input_str)
        .take_while(|(_, character, _)| character.is_ascii_alphabetic() || *character == '-')
        .map(|(_, _, end)| end)
        .last()
        .ok_or_else(|| Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input.string[0..0],
            },
            reason: Cow::from("Expected an identifier"),
            partial_parse: Span {
                inner: Ast::Identifier(""),
                full_string: input.full_string,
                string: &input.string[0..0],
            },
        })?;
    let (str_str, rest_str) = input_str.split_at(end);
    Ok((
        Span {
            inner: str_str,
            full_string: input.full_string,
            string: str_str,
        },
        Span {
            inner: (),
            full_string: input.full_string,
            string: rest_str,
        },
    ))
}

fn parse_whitespace<'a>(
    input: Span<'a, ()>,
) -> Result<(Span<'a, ()>, Span<'a, ()>), Error<'a, Span<'_, Ast<'_>>>> {
    let input_str = Into::<&'a str>::into(input);
    let pos = my_char_indices(input_str)
        .find(|(_offset, character, _)| !character.is_whitespace())
        .map(|(offset, _, _)| offset)
        .unwrap_or(input_str.len());
    let (whitespace_str, rest_str) = input_str.split_at(pos);
    Ok((
        Span {
            inner: (),
            full_string: input.full_string,
            string: whitespace_str,
        },
        Span {
            inner: (),
            full_string: input.full_string,
            string: rest_str,
        },
    ))
}

fn parse_list<'a>(
    full_input: Span<'a, ()>,
) -> Result<(Span<'a, Vec<Span<'a, Ast<'a>>>>, Span<'a, ()>), Error<'a, Span<'_, Ast<'_>>>> {
    let mut input = full_input;
    let input_str = Into::<&'a str>::into(input);
    if !input_str.starts_with('(') {
        return Err(Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input_str[0..0],
            },
            reason: Cow::from(r#"Expected a `(`"#),
            partial_parse: Span {
                inner: Ast::List(vec![]),
                full_string: full_input.full_string,
                string: &input_str[0..0],
            },
        });
    }
    input = Span {
        inner: (),
        full_string: input.full_string,
        string: &input_str[1..],
    };
    let mut result = Vec::new();
    loop {
        let result_ref = &result;
        input = parse_whitespace(input)
            .map_err(|err| Error {
                location: err.location,
                reason: err.reason,
                partial_parse: Span {
                    inner: Ast::List(result_ref.clone()),
                    full_string: full_input.full_string,
                    string: &input_str[0..0], // TODO FIXME
                },
            })?
            .1;
        let input_str = Into::<&'a str>::into(input);
        if let Some(rest) = input_str.strip_prefix(')') {
            let offset = input_str.as_ptr() as usize + 1 - full_input.string.as_ptr() as usize;
            break Ok((
                Span {
                    inner: result,
                    full_string: input.full_string,
                    string: &full_input.string[..offset],
                },
                Span {
                    inner: (),
                    full_string: input.full_string,
                    string: rest,
                },
            ));
        }
        let element;
        (element, input) = match parse_ast(input) {
            Ok(value) => value,
            Err(value) => Err(Error {
                location: value.location,
                reason: value.reason,
                partial_parse: Span {
                    inner: Ast::List(result_ref.clone()),
                    full_string: full_input.full_string,
                    string: &input_str[0..0], // TODO FIXME
                },
            })?,
        };
        result.push(element);
    }
}

pub fn visitor<'a>(
    element: &'a Span<'a, Ast<'a>>,
) -> Box<dyn Iterator<Item = (u64, u64, u64, u64, u64)> + 'a> {
    match &element.inner {
        Ast::Identifier(_) => {
            let pos = element.start_line_column();
            Box::new(std::iter::once((
                pos.0.try_into().unwrap(),
                pos.1.try_into().unwrap(),
                element.string.len().try_into().unwrap(),
                2,
                0,
            )))
        }
        Ast::Number(_) => {
            let pos = element.start_line_column();
            Box::new(std::iter::once((
                pos.0.try_into().unwrap(),
                pos.1.try_into().unwrap(),
                element.string.len().try_into().unwrap(),
                1,
                0,
            )))
        }
        Ast::String(_) => {
            let pos = element.start_line_column();
            Box::new(std::iter::once((
                pos.0.try_into().unwrap(),
                pos.1.try_into().unwrap(),
                element.string.len().try_into().unwrap(),
                0,
                0,
            )))
        }
        Ast::List(list) => Box::new(list.iter().flat_map(visitor)),
    }
}

pub fn list_visitor<'a>(
    element: &'a Span<'a, Ast<'a>>,
) -> Box<dyn Iterator<Item = FoldingRange> + 'a> {
    match &element.inner {
        Ast::Identifier(_) => Box::new(std::iter::empty()),
        Ast::Number(_) => Box::new(std::iter::empty()),
        Ast::String(_) => Box::new(std::iter::empty()),
        Ast::List(list) => Box::new(
            std::iter::once(FoldingRange {
                start_line: element.start_line_column().0.try_into().unwrap(),
                start_character: Some(element.start_line_column().1.try_into().unwrap()),
                end_line: element.end_line_column().0.try_into().unwrap(),
                end_character: Some(element.end_line_column().1.try_into().unwrap()),
                kind: Some(tucant_language_server_derive_output::FoldingRangeKind::Region),
                collapsed_text: None,
            })
            .chain(list.iter().flat_map(list_visitor)),
        ),
    }
}

pub fn hover_visitor<'a>(
    element: &'a Span<'a, Ast<'a>>,
    position: &Position,
) -> Option<&'a Span<'a, Ast<'a>>> {
    match &element.inner {
        Ast::Identifier(_) | Ast::Number(_) | Ast::String(_) => {
            if element.start_line_column()
                <= (
                    position.line.try_into().unwrap(),
                    position.character.try_into().unwrap(),
                )
                && (
                    position.line.try_into().unwrap(),
                    position.character.try_into().unwrap(),
                ) <= element.end_line_column()
            {
                Some(element)
            } else {
                None
            }
        }
        Ast::List(list) => {
            if element.start_line_column()
                == (
                    position.line.try_into().unwrap(),
                    position.character.try_into().unwrap(),
                )
                || (
                    position.line.try_into().unwrap(),
                    position.character.try_into().unwrap(),
                ) == element.end_line_column()
            {
                Some(element)
            } else {
                list.iter()
                    .map(|l| hover_visitor(l, position))
                    .filter_map(|x| x)
                    .next()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ast<'a> {
    Number(i64),
    String(&'a str),
    Identifier(&'a str),
    List(Vec<Span<'a, Ast<'a>>>),
}

pub fn parse_ast<'a>(
    mut input: Span<'a, ()>,
) -> Result<(Span<'a, Ast<'a>>, Span<'a, ()>), Error<'a, Span<'_, Ast<'_>>>> {
    input = parse_whitespace(input)?.1;
    let input_str = Into::<&'a str>::into(input);
    let mut it = my_char_indices(input_str);
    match it.next() {
        Some((_, '"', _)) => parse_string(input).map(|v| {
            (
                Span {
                    inner: Ast::String(v.0.inner),
                    full_string: v.0.full_string,
                    string: v.0.string,
                },
                v.1,
            )
        }),
        Some((_, '0'..='9', _)) => parse_number(input).map(|v| {
            (
                Span {
                    inner: Ast::Number(v.0.inner),
                    full_string: v.0.full_string,
                    string: v.0.string,
                },
                v.1,
            )
        }),
        Some((_, 'a'..='z' | 'A'..='Z', _)) => parse_identifier(input).map(|v| {
            (
                Span {
                    //
                    inner: Ast::Identifier(v.0.inner),
                    full_string: v.0.full_string,
                    string: v.0.string,
                },
                v.1,
            )
        }),
        Some((_, '(', _)) => parse_list(input).map(|v| {
            (
                Span {
                    inner: Ast::List(v.0.inner),
                    full_string: v.0.full_string,
                    string: v.0.string,
                },
                v.1,
            )
        }),
        Some((start, character, end)) => Err(Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input.string[start..end],
            },
            reason: Cow::from(format!(
                r#"Unexpected character `{}`. Expected `"`, 0-9, a-z, A-Z or `(`."#,
                character
            )),
            partial_parse: Span {
                inner: Ast::List(vec![]),
                full_string: input.full_string,
                string: &input.string[start..end],
            },
        }),
        None => Err(Error {
            location: Span {
                inner: (),
                full_string: input.full_string,
                string: &input.string[0..0],
            },
            reason: Cow::from("Unexpected end of input"),
            partial_parse: Span {
                inner: Ast::List(vec![]),
                full_string: input.full_string,
                string: &input.string[0..0],
            },
        }),
    }
}

pub fn parse_root(input: Span<()>) -> Result<(Span<Ast>, Span<()>), Error<Span<Ast>>> {
    let (ast, mut rest) = parse_ast(input)?;
    rest = parse_whitespace(rest)?.1;
    if !rest.string.is_empty() {
        Err(Error {
            location: rest,
            reason: Cow::from("Expected end of file."),
            partial_parse: ast,
        })
    } else {
        Ok((ast, rest))
    }
}

#[cfg(test)]
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
    assert_eq!(
        number.location.string,
        "70708777897986976707598759785978698752"
    );
}

// RUST_LOG=trace cargo watch -x 'test -- --nocapture test_parse_string'
#[test]
fn test_parse_string() {
    init();

    let span = Span::new(r#"notastring"#);
    let string = parse_string(span).unwrap_err();
    println!("{:?}", string);
    assert_eq!(string.reason, r#"Expected a `"`"#);
    assert_eq!(string.location.string, "n");

    let span = Span::new(r#""unterminated"#);
    let string = parse_string(span).unwrap_err();
    println!("{:?}", string);
    assert_eq!(string.reason, r#"Unterminated string literal"#);
    assert_eq!(string.location.string, r#""unterminated"#);

    let span = Span::new(r#""astring"jojo"#);
    let string = parse_string(span).unwrap();
    println!("{:?}", string);
    assert_eq!(string.0.inner, "astring");
    assert_eq!(string.0.string, r#""astring""#);
    assert_eq!(string.1.string, "jojo");

    let span = Span::new(r#""astring""#);
    let string = parse_string(span).unwrap();
    println!("{:?}", string);
    assert_eq!(string.0.inner, "astring");
    assert_eq!(string.0.string, r#""astring""#);
    assert_eq!(string.1.string, "");
}

// RUST_LOG=trace cargo watch -x 'test -- --nocapture test_parse_identifier'
#[test]
fn test_parse_identifier() {
    init();

    let span = Span::new(r#"7notanidentifier"#);
    let string = parse_identifier(span).unwrap_err();
    println!("{:?}", string);
    assert_eq!(string.reason, r#"Expected an identifier"#);
    assert_eq!(string.location.string, "");

    let span = Span::new(r#""notanidentifier"#);
    let string = parse_identifier(span).unwrap_err();
    println!("{:?}", string);
    assert_eq!(string.reason, r#"Expected an identifier"#);
    assert_eq!(string.location.string, "");

    let span = Span::new(r#"anidentifier"#);
    let string = parse_identifier(span).unwrap();
    println!("{:?}", string);
    assert_eq!(string.0.inner, "anidentifier");
    assert_eq!(string.0.string, "anidentifier");
    assert_eq!(string.1.string, "");

    let span = Span::new(r#"anidentifier    jlih"#);
    let string = parse_identifier(span).unwrap();
    println!("{:?}", string);
    assert_eq!(string.0.inner, "anidentifier");
    assert_eq!(string.0.string, "anidentifier");
    assert_eq!(string.1.string, "    jlih");
}

#[test]
fn test_parse_whitespace() {
    init();

    let span = Span::new(r#""#);
    let string = parse_whitespace(span).unwrap();
    println!("{:?}", string);
    assert_eq!(string.0.string, "");
    assert_eq!(string.1.string, "");

    let span = Span::new(r#"  f  fwwe wef"#);
    let string = parse_whitespace(span).unwrap();
    println!("{:?}", string);
    assert_eq!(string.0.string, "  ");
    assert_eq!(string.1.string, "f  fwwe wef");

    let span = Span::new(r#"dsfsdf dsf  "#);
    let string = parse_whitespace(span).unwrap();
    println!("{:?}", string);
    assert_eq!(string.0.string, "");
    assert_eq!(string.1.string, "dsfsdf dsf  ");
}

#[test]
fn test_parse_list() {
    init();

    let span = Span::new(r#"()"#);
    let value = parse_list(span).unwrap();
    println!("{:?}", value);
    assert_eq!(value.0.string, "()");
    assert_eq!(value.1.string, "");
    assert!(value.0.inner.is_empty());

    let span = Span::new(r#"(  1    2   3    )"#);
    let value = parse_list(span).unwrap();
    println!("{:?}", value);
    assert_eq!(value.0.string, "(  1    2   3    )");
    assert_eq!(value.1.string, "");
    assert_eq!(value.0.inner.len(), 3);
    assert!(matches!(value.0.inner[0].inner, Ast::Number(1)));
    assert_eq!(value.0.inner[0].string, "1");
    assert!(matches!(value.0.inner[1].inner, Ast::Number(2)));
    assert_eq!(value.0.inner[1].string, "2");
    assert!(matches!(value.0.inner[2].inner, Ast::Number(3)));
    assert_eq!(value.0.inner[2].string, "3");
}

#[test]
fn test_parse_ast() {
    init();

    let span = Span::new(r#"   ()"#);
    let value = parse_ast(span).unwrap();
    println!("{:?}", value);
    assert_eq!(value.0.string, "()");
    assert_eq!(value.1.string, "");
    let value = match value {
        (
            Span {
                inner: Ast::List(list),
                ..
            },
            _,
        ) => list,
        _ => panic!("Expected AST list"),
    };
    assert!(value.is_empty());

    let span = Span::new(r#"  (  1    2   3    )"#);
    let value = parse_ast(span).unwrap();
    println!("{:?}", value);
    assert_eq!(value.0.string, "(  1    2   3    )");
    assert_eq!(value.1.string, "");
    let value = match value {
        (
            Span {
                inner: Ast::List(list),
                ..
            },
            _,
        ) => list,
        _ => panic!("Expected AST list"),
    };
    assert_eq!(value.len(), 3);
    assert!(matches!(value[0].inner, Ast::Number(1)));
    assert_eq!(value[0].string, "1");
    assert!(matches!(value[1].inner, Ast::Number(2)));
    assert_eq!(value[1].string, "2");
    assert!(matches!(value[2].inner, Ast::Number(3)));
    assert_eq!(value[2].string, "3");
}
