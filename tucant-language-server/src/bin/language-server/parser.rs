use std::{fmt::Debug, iter::Peekable};

use itertools::Itertools;
use once_cell::sync::Lazy;
use tucant_language_server_derive_output::{FoldingRange, Position, Range};

// write idiomatic code first, optimize later

#[derive(Debug, Clone)]
pub struct Span {
    pub filename: String,
    pub range: Range,
}

pub static FAKE_SPAN: Lazy<Span> = Lazy::new(|| Span {
    filename: String::from("<fake>"),
    range: Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 0,
            character: 0,
        },
    },
});

#[derive(Debug)]
pub struct Error<T: Debug> {
    pub location: Span,
    pub reason: String,
    pub partial_parse: T,
}

#[derive(Debug)]
pub enum Token {
    ParenOpen,
    ParenClose,
    String(String),
    Identifier(String),
    Number(i64),
}

#[derive(Debug, Clone)]
pub enum Ast {
    Number(i64),
    String(String),
    Identifier(String),
    List(Vec<(Ast, Span)>),
}

#[derive(Clone)]
pub struct LineColumnIterator<I: Iterator<Item = char> + Clone> {
    iterator: I,
    position: Position,
}

impl<I: Iterator<Item = char> + Clone> LineColumnIterator<I> {
    pub const fn new(iterator: I) -> Self {
        Self {
            iterator,
            position: Position {
                line: 0,
                character: 0,
            },
        }
    }
}

impl<I: Iterator<Item = char> + Clone> Iterator for LineColumnIterator<I> {
    type Item = (char, Position);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(character) => {
                let old_position = self.position.clone();
                match character {
                    '\n' => {
                        self.position.line += 1;
                        self.position.character = 0;
                    }
                    '\r' => {}
                    _ => {
                        self.position.character += 1;
                    }
                }
                Some((character, old_position))
            }
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct Tokenizer<I: Iterator<Item = char> + Clone> {
    iterator: Peekable<LineColumnIterator<I>>,
}

impl<I: Iterator<Item = char> + Clone> Tokenizer<I> {
    pub fn new(iterator: I) -> Self {
        Self {
            iterator: LineColumnIterator::new(iterator).peekable(),
        }
    }
}

pub struct TokenizerBuilder;

impl TokenizerBuilder {
    #[must_use]
    pub fn from_string(string: &str) -> Tokenizer<std::vec::IntoIter<char>> {
        Tokenizer::new(string.chars().collect::<Vec<_>>().into_iter())
    }
}

fn parse_paren_open<I: Iterator<Item = char> + Clone>(
    iterator: &mut Peekable<LineColumnIterator<I>>,
) -> Result<(Token, Span), Error<()>> {
    match iterator.next().unwrap() {
        ('(', position) => Ok((
            Token::ParenOpen,
            Span {
                filename: "<stdin>".to_string(),
                range: Range {
                    start: position.clone(),
                    end: Position {
                        line: position.line,
                        character: position.character + 1,
                    },
                },
            },
        )),
        (_, position) => Err(Error {
            location: Span {
                filename: "<stdin>".to_string(),
                range: Range {
                    start: position.clone(),
                    end: position,
                },
            },
            reason: String::new(),
            partial_parse: (),
        }),
    }
}

fn parse_paren_close<I: Iterator<Item = char> + Clone>(
    iterator: &mut Peekable<LineColumnIterator<I>>,
) -> Result<(Token, Span), Error<()>> {
    // TODO FIXME duplication
    match iterator.next().unwrap() {
        (')', position) => Ok((
            // TODO FIXME this is already checked in the caller, maybe clone iterators and just try parsing?
            Token::ParenClose,
            Span {
                filename: "<stdin>".to_string(),
                range: Range {
                    start: position.clone(),
                    end: Position {
                        line: position.line,
                        character: position.character + 1,
                    },
                },
            },
        )),
        (_, position) => Err(Error {
            location: Span {
                filename: "<stdin>".to_string(),
                range: Range {
                    start: position.clone(),
                    end: position,
                },
            },
            reason: String::new(),
            partial_parse: (),
        }),
    }
}

pub fn parse_string<I: Iterator<Item = char> + Clone>(
    iterator: &mut Peekable<LineColumnIterator<I>>,
) -> Result<(Token, Span), Error<()>> {
    match iterator.next().unwrap() {
        ('"', start_pos) => {
            let end: String = iterator
                .peeking_take_while(|(char, _pos)| *char != '"')
                .map(|(char, _pos)| char)
                .collect();
            match iterator.next() {
                Some(('"', end_pos)) => Ok((
                    Token::String(end),
                    Span {
                        filename: "<stdin>".to_string(),
                        range: Range {
                            start: start_pos,
                            end: Position {
                                line: end_pos.line,
                                character: end_pos.character + 1,
                            },
                        },
                    },
                )),
                Some((_, end_pos)) => Err(Error {
                    location: Span {
                        filename: "<stdin>".to_string(),
                        range: Range {
                            start: end_pos.clone(),
                            end: end_pos,
                        },
                    },
                    reason: r#"Expected a `"`"#.to_string(),
                    partial_parse: (),
                }),
                None => Err(Error {
                    location: Span {
                        filename: "<stdin>".to_string(),
                        range: Range {
                            start: start_pos.clone(),
                            end: start_pos,
                        },
                    },
                    reason: r#"Unterminated string literal"#.to_string(),
                    partial_parse: (),
                }),
            }
        }
        (_, position) => Err(Error {
            location: Span {
                filename: "<stdin>".to_string(),
                range: Range {
                    start: position.clone(),
                    end: position,
                },
            },
            reason: r#"Expected a `"`"#.to_string(),
            partial_parse: (),
        }),
    }
}

pub fn parse_number<I: Iterator<Item = char> + Clone>(
    iterator: &mut Peekable<LineColumnIterator<I>>,
) -> Result<(Token, Span), Error<()>> {
    match iterator.peek().unwrap() {
        ('0'..='9', start_pos) => {
            let start_pos = start_pos.clone();
            let end_pos = iterator
                .clone()
                .peeking_take_while(|(char, _pos)| char.is_ascii_digit())
                .map(|(_char, pos)| pos)
                .last()
                .unwrap();
            let number: String = iterator
                .peeking_take_while(|(char, _pos)| char.is_ascii_digit())
                .map(|(char, _pos)| char)
                .collect();

            let span = Span {
                filename: "<stdin>".to_string(),
                range: Range {
                    start: start_pos,
                    end: Position {
                        line: end_pos.line,
                        character: end_pos.character + 1,
                    },
                },
            };
            match number.parse() {
                Ok(n) => Ok((Token::Number(n), span)),
                Err(_err) => Err(Error {
                    location: span,
                    reason: "Failed to parse number".to_string(),
                    partial_parse: (),
                }),
            }
        }
        (_, position) => Err(Error {
            location: Span {
                filename: "<stdin>".to_string(),
                range: Range {
                    start: position.clone(),
                    end: position.clone(),
                },
            },
            reason: "Failed to parse number".to_string(),
            partial_parse: (),
        }),
    }
}

fn parse_identifier<I: Iterator<Item = char> + Clone>(
    iterator: &mut Peekable<LineColumnIterator<I>>,
) -> Result<(Token, Span), Error<()>> {
    match iterator.peek().unwrap() {
        ('a'..='z' | 'A'..='Z' | '_', start_pos) => {
            let start_pos = start_pos.clone();
            let end_pos = iterator
                .clone()
                .peeking_take_while(|(char, _pos)| !char.is_whitespace() && *char != ')')
                .map(|(_char, pos)| pos)
                .last()
                .unwrap();
            let number: String = iterator
                .peeking_take_while(|(char, _pos)| !char.is_whitespace() && *char != ')')
                .map(|(char, _pos)| char)
                .collect();

            Ok((
                Token::Identifier(number),
                Span {
                    filename: "<stdin>".to_string(),
                    range: Range {
                        start: start_pos,
                        end: Position {
                            line: end_pos.line,
                            character: end_pos.character + 1,
                        },
                    },
                },
            ))
        }
        (_, position) => Err(Error {
            location: Span {
                filename: "<stdin>".to_string(),
                range: Range {
                    start: position.clone(),
                    end: position.clone(),
                },
            },
            reason: "Expected an identifier".to_string(),
            partial_parse: (),
        }),
    }
}

impl<I: Iterator<Item = char> + Clone> Iterator for Tokenizer<I> {
    type Item = Result<(Token, Span), Error<()>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.peek() {
            Some(('(', _position)) => Some(parse_paren_open(&mut self.iterator)),
            Some((')', _position)) => Some(parse_paren_close(&mut self.iterator)),
            Some(('"', _start_pos)) => Some(parse_string(&mut self.iterator)),
            Some(('0'..='9', _start_pos)) => Some(parse_number(&mut self.iterator)),
            Some(('a'..='z' | 'A'..='Z' | '_', _start_pos)) => {
                Some(parse_identifier(&mut self.iterator))
            }
            Some((' ' | '\t' | '\n' | '\r', _)) => {
                self.iterator.next();
                // whitespace
                self.next()
            }
            Some(_) => {
                self.iterator.next();
                // unexpected character
                // TODO FIXME error
                None
            }
            None => None,
        }
    }
}

pub fn parse<I: Iterator<Item = char> + Clone>(
    tokenizer: &mut Peekable<Tokenizer<I>>,
) -> Result<(Ast, Span), Error<()>> {
    match tokenizer.next().transpose()? {
        Some((Token::Identifier(ident), span)) => Ok((Ast::Identifier(ident), span)),
        Some((Token::Number(ident), span)) => Ok((Ast::Number(ident), span)),
        Some((Token::String(ident), span)) => Ok((Ast::String(ident), span)),
        Some((Token::ParenOpen, open_span)) => {
            let mut list = Vec::new();
            let close_span = loop {
                match tokenizer.peek() {
                    Some(Ok((Token::ParenClose, close_span))) => break close_span.clone(),
                    _ => list.push(parse(tokenizer)?),
                }
            };
            tokenizer.next();
            Ok((
                Ast::List(list),
                Span {
                    filename: open_span.filename,
                    range: Range {
                        start: open_span.range.start,
                        end: close_span.range.end,
                    },
                },
            ))
        }
        Some((Token::ParenClose, span)) => Err(Error {
            location: span,
            reason: "unmatched closing paren at".to_string(),
            partial_parse: (),
        }),
        None => Err(Error {
            location: FAKE_SPAN.clone(),
            reason: "unexpected end of input".to_string(),
            partial_parse: (),
        }),
    }
}

pub fn parse_from_str(string: &str) -> Result<(Ast, Span), Error<()>> {
    let span = TokenizerBuilder::from_string(string);
    parse(&mut span.peekable())
}

// cargo test --target x86_64-unknown-linux-gnu parser -- --show-output
#[test]
pub fn test_tokenize() {
    println!(
        "{:#?}",
        TokenizerBuilder::from_string(r#"(this is "awesome" 1337 lisp)"#).collect::<Vec<_>>()
    );
}

pub fn visitor(element: (Ast, Span)) -> Box<dyn Iterator<Item = (u64, u64, u64, u64, u64)>> {
    match element.0 {
        Ast::Identifier(_identifier) => {
            let start_pos = element.1.range.start;
            let end_pos = element.1.range.end;
            Box::new(std::iter::once((
                start_pos.line,
                start_pos.character,
                end_pos.character - start_pos.character,
                2,
                0,
            )))
        }
        Ast::Number(_) => {
            let start_pos = element.1.range.start;
            let end_pos = element.1.range.end;
            Box::new(std::iter::once((
                start_pos.line,
                start_pos.character,
                end_pos.character - start_pos.character,
                1,
                0,
            )))
        }
        Ast::String(_) => {
            let start_pos = element.1.range.start;
            let end_pos = element.1.range.end;
            Box::new(std::iter::once((
                start_pos.line,
                start_pos.character,
                end_pos.character - start_pos.character,
                0,
                0,
            )))
        }
        Ast::List(list) => {
            let start_pos = element.1.range.start;
            let end_pos = element.1.range.end;
            Box::new(
                std::iter::once((start_pos.line, start_pos.character, 1, 3, 0))
                    .chain(list.into_iter().flat_map(visitor))
                    .chain(std::iter::once((
                        end_pos.line,
                        end_pos.character.saturating_sub(1),
                        1,
                        3,
                        0,
                    ))),
            )
        }
    }
}

pub fn list_visitor(element: (Ast, Span)) -> Box<dyn Iterator<Item = FoldingRange>> {
    match element.0 {
        Ast::Identifier(_) | Ast::Number(_) | Ast::String(_) => Box::new(std::iter::empty()),
        Ast::List(list) => Box::new(
            std::iter::once(FoldingRange {
                start_line: element.1.range.start.line,
                start_character: Some(element.1.range.start.character),
                end_line: element.1.range.end.line,
                end_character: Some(element.1.range.end.character),
                kind: Some(tucant_language_server_derive_output::FoldingRangeKind::Region),
                collapsed_text: None,
            })
            .chain(list.into_iter().flat_map(list_visitor)),
        ),
    }
}

#[must_use]
pub fn hover_visitor(element: (Ast, Span), position: &Position) -> Option<(Ast, Span)> {
    match element.0 {
        Ast::Identifier(_) | Ast::Number(_) | Ast::String(_) => {
            if (element.1.range.start.line, element.1.range.start.character)
                <= (position.line, position.character)
                && (position.line, position.character)
                    <= (element.1.range.end.line, element.1.range.end.character)
            {
                Some(element)
            } else {
                None
            }
        }
        Ast::List(ref list) => {
            if &element.1.range.start == position || position == &element.1.range.end {
                Some(element)
            } else {
                list.iter().find_map(|l| hover_visitor(l.clone(), position))
            }
        }
    }
}

#[must_use]
pub fn highlight_visitor(element: (Ast, Span), position: &Position) -> Vec<Span> {
    match element.0 {
        Ast::Identifier(_) | Ast::Number(_) | Ast::String(_) => {
            if (element.1.range.start.line, element.1.range.start.character)
                <= (position.line, position.character)
                && (position.line, position.character)
                    <= (element.1.range.end.line, element.1.range.end.character)
            {
                vec![element.1]
            } else {
                vec![]
            }
        }
        Ast::List(ref list) => {
            if &element.1.range.start == position || position == &element.1.range.end {
                vec![
                    Span {
                        filename: element.1.filename.clone(),
                        range: Range {
                            start: element.1.range.start.clone(),
                            end: Position {
                                line: element.1.range.start.line,
                                character: element.1.range.start.character + 1,
                            },
                        },
                    },
                    Span {
                        filename: element.1.filename,
                        range: Range {
                            start: Position {
                                line: element.1.range.end.line,
                                character: element.1.range.end.character - 1,
                            },
                            end: element.1.range.end,
                        },
                    },
                ]
            } else {
                list.iter()
                    .flat_map(|l| highlight_visitor(l.clone(), position))
                    .collect()
            }
        }
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

    let mut span = TokenizerBuilder::from_string(r#"notanumber"#);
    let number = parse_number(&mut span.iterator).unwrap_err();
    println!("{number:?}");
    assert_eq!(number.reason, "Failed to parse number");
    //assert_eq!(number.location.string, "");

    let mut span = TokenizerBuilder::from_string(r#"3notendingwithanumber"#);
    let number = parse_number(&mut span.iterator).unwrap();
    println!("{number:?}");
    assert!(matches!(number.0, Token::Number(3)));
    //assert_eq!(number.0.string, "3");
    //assert_eq!(number.1.string, "notendingwithanumber");

    let mut span = TokenizerBuilder::from_string(r#"3"#);
    let number = parse_number(&mut span.iterator).unwrap();
    println!("{number:?}");
    assert!(matches!(number.0, Token::Number(3)));
    //assert_eq!(number.0.string, "3");
    //assert_eq!(number.1.string, "");

    let mut span = TokenizerBuilder::from_string(r#"3z9"#);
    let number = parse_number(&mut span.iterator).unwrap();
    println!("{number:?}");
    assert!(matches!(number.0, Token::Number(3)));
    //assert_eq!(number.0.string, "3");
    //assert_eq!(number.1.string, "z9");

    let mut span = TokenizerBuilder::from_string(r#"3546z945"#);
    let number = parse_number(&mut span.iterator).unwrap();
    println!("{number:?}");
    assert!(matches!(number.0, Token::Number(3546)));
    //assert_eq!(number.0.string, "3546");
    //assert_eq!(number.1.string, "z945");

    let mut span = TokenizerBuilder::from_string(r#"345345"#);
    let number = parse_number(&mut span.iterator).unwrap();
    println!("{number:?}");
    assert!(matches!(number.0, Token::Number(345_345)));
    //assert_eq!(number.0.string, "345345");
    //assert_eq!(number.1.string, "");

    let mut span = TokenizerBuilder::from_string(r#"345345sdfasd"#);
    let number = parse_number(&mut span.iterator).unwrap();
    println!("{number:?}");
    assert!(matches!(number.0, Token::Number(345_345)));
    //assert_eq!(number.0.string, "345345");
    //assert_eq!(number.1.string, "sdfasd");

    let mut span = TokenizerBuilder::from_string(r#"n32otanumber"#);
    let number = parse_number(&mut span.iterator).unwrap_err();
    println!("{number:?}");
    assert_eq!(number.reason, "Failed to parse number");
    //assert_eq!(number.location.string, "");

    let mut span =
        TokenizerBuilder::from_string(r#"70708777897986976707598759785978698752otanumber"#);
    let number = parse_number(&mut span.iterator).unwrap_err();
    println!("{number:?}");
    assert_eq!(number.reason, "Failed to parse number");
    /*assert_eq!(
        number.location.string,
        "70708777897986976707598759785978698752"
    );*/
}

// RUST_LOG=trace cargo watch -x 'test -- --nocapture test_parse_string'
#[test]
fn test_parse_string() {
    init();

    let mut span = TokenizerBuilder::from_string(r#"notastring"#);
    let string = parse_string(&mut span.iterator).unwrap_err();
    println!("{string:?}");
    assert_eq!(string.reason, r#"Expected a `"`"#);
    //assert_eq!(string.location.string, "n");

    let mut span = TokenizerBuilder::from_string(r#""unterminated"#);
    let string = parse_string(&mut span.iterator).unwrap_err();
    println!("{string:?}");
    assert_eq!(string.reason, r#"Unterminated string literal"#);
    //assert_eq!(string.location.string, r#""unterminated"#);

    let mut span = TokenizerBuilder::from_string(r#""astring"jojo"#);
    let string = parse_string(&mut span.iterator).unwrap();
    println!("{string:?}");
    assert!(matches!(string.0, Token::String(v) if v == "astring"));
    //assert_eq!(string.0.string, r#""astring""#);
    //assert_eq!(string.1.string, "jojo");

    let mut span = TokenizerBuilder::from_string(r#""astring""#);
    let string = parse_string(&mut span.iterator).unwrap();
    println!("{string:?}");
    assert!(matches!(string.0, Token::String(v) if v == "astring"));
    //assert_eq!(string.0.string, r#""astring""#);
    //assert_eq!(string.1.string, "");
}

// RUST_LOG=trace cargo watch -x 'test -- --nocapture test_parse_identifier'
#[test]
fn test_parse_identifier() {
    init();

    let mut span = TokenizerBuilder::from_string(r#"7notanidentifier"#);
    let string = parse_identifier(&mut span.iterator).unwrap_err();
    println!("{string:?}");
    assert_eq!(string.reason, r#"Expected an identifier"#);
    //assert_eq!(string.location.string, "");

    let mut span = TokenizerBuilder::from_string(r#""notanidentifier"#);
    let string = parse_identifier(&mut span.iterator).unwrap_err();
    println!("{string:?}");
    assert_eq!(string.reason, r#"Expected an identifier"#);
    //assert_eq!(string.location.string, "");

    let mut span = TokenizerBuilder::from_string(r#"anidentifier"#);
    let string = parse_identifier(&mut span.iterator).unwrap();
    println!("{string:?}");
    assert!(matches!(string.0, Token::Identifier(v) if v == "anidentifier"));
    //assert_eq!(string.0.string, "anidentifier");
    //assert_eq!(string.1.string, "");

    let mut span = TokenizerBuilder::from_string(r#"anidentifier    jlih"#);
    let string = parse_identifier(&mut span.iterator).unwrap();
    println!("{string:?}");
    assert!(matches!(string.0, Token::Identifier(v) if v == "anidentifier"));
    //assert_eq!(string.0.string, "anidentifier");
    //assert_eq!(string.1.string, "    jlih");
}

#[test]
fn test_parse_whitespace() {
    init();

    let mut span = TokenizerBuilder::from_string("");
    assert_eq!(span.iterator.next(), None);
    //assert_eq!(string.0.string, "");
    //assert_eq!(string.1.string, "");

    let span = TokenizerBuilder::from_string(r#"  f  fwwe wef"#);
    let string = parse(&mut span.peekable()).unwrap();
    println!("{string:?}");
    //assert_eq!(string.0.string, "  ");
    //assert_eq!(string.1.string, "f  fwwe wef");

    let span = TokenizerBuilder::from_string(r#"dsfsdf dsf  "#);
    let string = parse(&mut span.peekable()).unwrap();
    println!("{string:?}");
    //assert_eq!(string.0.string, "");
    //assert_eq!(string.1.string, "dsfsdf dsf  ");
}

#[test]
fn test_parse_list() {
    init();

    println!(
        "{:?}",
        TokenizerBuilder::from_string(r#"()"#).collect::<Vec<_>>()
    );

    let span = TokenizerBuilder::from_string(r#"()"#);
    let value = parse(&mut span.peekable()).unwrap();
    println!("{value:?}");
    //assert_eq!(value.0.string, "()");
    //assert_eq!(value.1.string, "");
    assert!(matches!(value.0, Ast::List(list) if matches!(list.as_slice(), [])));

    let span = TokenizerBuilder::from_string(r#"(  1    2   3    )"#);
    let value = parse(&mut span.peekable()).unwrap();
    println!("{value:?}");
    //assert_eq!(value.0.string, "(  1    2   3    )");
    //assert_eq!(value.1.string, "");
    assert!(
        matches!(value.0, Ast::List(list) if matches!(list.as_slice(), [(Ast::Number(1),_), (Ast::Number(2),_), (Ast::Number(3),_)]))
    );
}

#[test]
fn test_parse_ast() {
    init();

    let span = TokenizerBuilder::from_string(r#"   ()"#);
    let value = parse(&mut span.peekable()).unwrap();
    println!("{value:?}");
    //assert_eq!(value.0.string, "()");
    //assert_eq!(value.1.string, "");
    assert!(matches!(value.0, Ast::List(list) if matches!(list.as_slice(), [])));

    let span = TokenizerBuilder::from_string(r#"  (  1    2   3    )"#);
    let value = parse(&mut span.peekable()).unwrap();
    println!("{value:?}");
    //assert_eq!(value.0.string, "(  1    2   3    )");
    //assert_eq!(value.1.string, "");
    assert!(
        matches!(value.0, Ast::List(list) if matches!(list.as_slice(), [(Ast::Number(1),_), (Ast::Number(2),_), (Ast::Number(3),_)]))
    );
}
