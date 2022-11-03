use std::{borrow::Cow, fmt::Debug, iter::Peekable};

use itertools::Itertools;
use tucant_language_server_derive_output::{FoldingRange, Position, Range};

// write idiomatic code first, optimize later

#[derive(Debug, Clone)]
pub struct Span {
    pub filename: String,
    pub range: Range
}

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

pub struct LineColumnIterator<I: Iterator<Item=char>> {
    iterator: I,
    position: Position,
}

impl<I: Iterator<Item=char>> LineColumnIterator<I> {
    pub fn new(iterator: I) -> Self {
        Self {
            iterator,
            position: Position { line: 0, character: 0 },
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for LineColumnIterator<I> {
    type Item = (char, Position);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(character) => {
                match character {
                    '\n' => {
                        self.position.line += 1;
                        self.position.character = 0;
                    }
                    '\r' => {},
                    _ => {
                        self.position.character += 1;
                    }
                }
                Some((character, self.position.clone()))
            },
            None => None,
        }
    }
}

pub struct Tokenizer<I: Iterator<Item=char>> {
    iterator: Peekable<LineColumnIterator<I>>,
}

impl<I: Iterator<Item=char>> Tokenizer<I> {
    pub fn new(iterator: I) -> Self {
        Self {
            iterator: LineColumnIterator::new(iterator).peekable(),
        }
    }
}

pub struct TokenizerBuilder;

impl TokenizerBuilder {
    pub fn from_string(string: String) -> Tokenizer<std::vec::IntoIter<char>> {
        Tokenizer::new(string.chars().collect::<Vec<_>>().into_iter())
    }
}

impl<I: Iterator<Item=char>> Iterator for Tokenizer<I> {
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.peek() {
            Some(('(', position)) => {
                self.iterator.next();
                Some((Token::ParenOpen, Span {
                    filename: "<stdin>".to_string(),
                    range: Range {
                        start: position.clone(),
                        end: Position {
                            line: position.line,
                            character: position.character + 1,
                        },
                    }
                }))
            },
            Some((')', position)) => {
                self.iterator.next();
                Some((Token::ParenClose, Span {
                    filename: "<stdin>".to_string(),
                    range: Range {
                        start: position.clone(),
                        end: Position {
                            line: position.line,
                            character: position.character + 1,
                        },
                    }
                }))
            },
            Some(('"', start_pos)) => {
                self.iterator.next();
                let end: String = self.iterator.peeking_take_while(|(char, pos)| *char != '"').map(|(char, pos)| char).collect();
                if let Some(('"', end_pos)) = self.iterator.next() {
                    Some((Token::String(end), Span {
                        filename: "<stdin>".to_string(),
                        range: Range {
                            start: start_pos.clone(),
                            end: end_pos,
                        },
                    }))
                } else {
                    // unterminated string literal
                    None // TODO FIXME error
                }
            },
            Some(('0' ..= '9', _)) => {
                let end: String = self.iterator.peeking_take_while(|(char, pos)| char.is_ascii_digit()).map(|(char, pos)| char).collect();
                Some(Token::Number(end.parse().unwrap()))
            },
            Some(('a' ..= 'z' | 'A' ..= 'Z' | '_', _)) => {
                let end: String = self.iterator.peeking_take_while(|(char, pos)| !char.is_whitespace() && *char != ')').map(|(char, pos)| char).collect();
                Some(Token::Identifier(end))
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

// cargo test --target x86_64-unknown-linux-gnu parser -- --show-output
#[test]
pub fn test_tokenize() {
    println!("{:?}", TokenizerBuilder::from_string(r#"(this is "awesome" 1337 lisp)"#.to_string()).collect_vec());
}

/*
pub fn visitor<'a>(
    element: (Ast, Span),
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
    element: (Ast, Span),
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
    element: (Ast, Span),
    position: &Position,
) -> Option<(Ast, Span)> {
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
                    .filter_map(|l| hover_visitor(l, position))
                    .next()
            }
        }
    }
}*/

#[derive(Debug, Clone)]
pub enum Ast {
    Number(i64),
    String(String),
    Identifier(String),
    List(Vec<(Ast, Span)>),
}

#[cfg(test)]
fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

/* 
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

*/