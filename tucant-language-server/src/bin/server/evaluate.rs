use crate::parser::{parse_root, Ast, Span};

use std::borrow::Cow;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct EvaluateError<'a> {
    pub location: Option<Span<'a, ()>>,
    pub reason: Cow<'static, str>,
}

pub type EvaluateResult<'a, V> = Result<V, EvaluateError<'a>>;

pub trait Value<'a>: Debug {
    fn evaluate_call(
        self: Rc<Self>,
        _context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>,
        _args: &[Span<'a, Ast<'a>>],
    ) -> EvaluateResult<'a, Rc<dyn Value<'a> + 'a>> {
        Err(EvaluateError {
            location: None,
            reason: "not yet implemented".to_string().into(),
        }) // TODO FIXME add span information
    }

    fn span(&self) -> Span<'a, ()>;

    fn downcast_integer_value(&self) -> Option<&IntegerValue> {
        None
    }
}

pub trait Type<'a>: Debug {
    fn typecheck_call(
        self: Rc<Self>,
        _context: &mut Vec<(String, Rc<dyn Type<'a> + 'a>)>,
        _args: &[Span<'a, Ast<'a>>],
    ) -> EvaluateResult<'a, Rc<dyn Type<'a> + 'a>> {
        Err(EvaluateError {
            location: None,
            reason: "not yet implemented".to_string().into(),
        })
    }

    fn span(&self) -> Span<'a, ()>;

    fn downcast_integer_type(&self) -> Option<&IntegerType> {
        None
    }
}

#[derive(Debug)]
pub struct IntegerValue(i64);

impl<'a> Value<'a> for Span<'a, IntegerValue> {
    fn downcast_integer_value(&self) -> Option<&IntegerValue> {
        Some(&self.inner)
    }

    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

#[derive(Debug)]
pub struct IntegerType(Option<i64>);

impl<'a> Type<'a> for Span<'a, IntegerType> {
    fn downcast_integer_type(&self) -> Option<&IntegerType> {
        Some(&self.inner)
    }

    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

#[derive(Debug)]
pub struct StringValue(String);

impl<'a> Value<'a> for Span<'a, StringValue> {
    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

#[derive(Debug)]
pub struct StringType(Option<String>);

impl<'a> Type<'a> for Span<'a, StringType> {
    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

#[derive(Debug)]
pub struct AddLambdaValue; // if this doesn't work maybe just add a span to every one of them and add a methdod that returns the span?

impl<'a> Value<'a> for Span<'a, AddLambdaValue> {
    fn evaluate_call(
        self: Rc<Self>,
        context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>,
        args: &[Span<'a, Ast<'a>>],
    ) -> EvaluateResult<'a, Rc<dyn Value<'a> + 'a>> {
        let [left, right]: &[Span<'a, Ast<'a>>; 2] = args.try_into().or_else(|_| {
            Err(EvaluateError {
                location: None,
                reason: "expected exactly two arguments".to_string().into(),
            })
        })?;
        let left_value = evaluate_with_context(context, left.clone())?;
        let right_value = evaluate_with_context(context, right.clone())?;
        let left_value = left_value.downcast_integer_value().ok_or(EvaluateError {
            location: Some(left_value.span()),
            reason: format!("expected integer type, got {:?}", left_value).into(),
        })?;
        let right_value = right_value.downcast_integer_value().ok_or(EvaluateError {
            location: Some(right_value.span()),
            reason: format!("expected integer type, got {:?}", right_value).into(),
        })?;
        Ok(Rc::new(Span {
            inner: IntegerValue(
                left_value
                    .0
                    .checked_add(right_value.0)
                    .ok_or(EvaluateError {
                        location: None,
                        reason: format!(
                            "integer overflow, adding {:?} and {:?}",
                            left_value, right_value
                        )
                        .into(),
                    })?,
            ),
            full_string: "", // TODO FIXME join two spans
            string: "",
        }))
    }

    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

#[derive(Debug)]
pub struct AddLambdaType;

impl<'a> Type<'a> for Span<'a, AddLambdaType> {
    fn typecheck_call(
        self: Rc<Self>,
        context: &mut Vec<(String, Rc<dyn Type<'a> + 'a>)>,
        args: &[Span<'a, Ast<'a>>],
    ) -> EvaluateResult<'a, Rc<dyn Type<'a> + 'a>> {
        let [left, right]: &[Span<'a, Ast<'a>>; 2] = args.try_into().or_else(|_| {
            Err(EvaluateError {
                location: None,
                reason: "expected exactly two arguments".to_string().into(),
            })
        })?;
        let left_value = typecheck_with_context(context, left.clone())?;
        let right_value = typecheck_with_context(context, right.clone())?;
        let left_value = left_value.downcast_integer_type().ok_or(EvaluateError {
            location: Some(left_value.span()),
            reason: format!("expected integer type, got {:?}", left_value).into(),
        })?;
        let right_value = right_value.downcast_integer_type().ok_or(EvaluateError {
            location: Some(right_value.span()),
            reason: format!("expected integer type, got {:?}", right_value).into(),
        })?;
        Ok(Rc::new(Span {
            inner: IntegerType(
                left_value
                    .0
                    .and_then(|l| {
                        right_value.0.map(|r| {
                            l.checked_add(r).ok_or(EvaluateError {
                                location: None,
                                reason: format!(
                                    "integer overflow, adding {:?} and {:?}",
                                    left_value, right_value
                                )
                                .into(),
                            })
                        })
                    })
                    .transpose()?,
            ),
            full_string: "",
            string: "",
        }))
    }

    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

#[derive(Debug)]
pub struct LambdaValue<'a> {
    variable: String,
    body: Span<'a, Ast<'a>>,
}

impl<'a> Value<'a> for Span<'a, LambdaValue<'a>> {
    fn evaluate_call(
        self: Rc<Self>,
        context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>,
        args: &[Span<'a, Ast<'a>>],
    ) -> EvaluateResult<'a, Rc<dyn Value<'a> + 'a>> {
        let [variable_value]: &[Span<'a, Ast<'a>>; 1] = args.try_into().or_else(|_| {
            Err(EvaluateError {
                location: None,
                reason: "expected exactly one argument".to_string().into(),
            })
        })?;
        let arg_value = evaluate_with_context(context, variable_value.clone())?;
        context.push((self.inner.variable.clone(), arg_value));
        let return_value = evaluate_with_context(context, self.inner.body.clone());
        context.pop();
        return_value
    }

    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

#[derive(Debug)]
pub struct LambdaType<'a> {
    variable: String,
    body: Span<'a, Ast<'a>>,
}

impl<'a> Type<'a> for Span<'a, LambdaType<'a>> {
    fn typecheck_call(
        self: Rc<Self>,
        context: &mut Vec<(String, Rc<dyn Type<'a> + 'a>)>,
        args: &[Span<'a, Ast<'a>>],
    ) -> EvaluateResult<'a, Rc<dyn Type<'a> + 'a>> {
        let [variable_value]: &[Span<'a, Ast<'a>>; 1] = args.try_into().or_else(|_| {
            Err(EvaluateError {
                location: None,
                reason: "expected exactly one argument".to_string().into(),
            })
        })?;
        let arg_value = typecheck_with_context(context, variable_value.clone())?;
        context.push((self.inner.variable.clone(), arg_value));
        let return_value = typecheck_with_context(context, self.inner.body.clone());
        context.pop();
        return_value
    }

    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

#[derive(Debug)]
pub struct DefineLambdaValue;

impl<'a> Value<'a> for Span<'a, DefineLambdaValue> {
    fn evaluate_call(
        self: Rc<Self>,
        _context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>,
        args: &[Span<'a, Ast<'a>>],
    ) -> EvaluateResult<'a, Rc<dyn Value<'a> + 'a>> {
        let [variable, body]: &[Span<'a, Ast<'a>>; 2] = args.try_into().or_else(|_| {
            Err(EvaluateError {
                location: None,
                reason: "expected exactly two arguments".to_string().into(),
            })
        })?;
        let variable = match variable.inner {
            Ast::Identifier(identifier) => identifier,
            _ => Err(EvaluateError {
                location: None,
                reason: "expected argument identifier".to_string().into(),
            })?,
        };
        Ok(Rc::new(Span {
            inner: LambdaValue::<'_> {
                variable: variable.to_string(),
                body: body.clone(),
            },
            full_string: "",
            string: "",
        }))
    }

    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

#[derive(Debug)]
pub struct DefineLambdaType;

impl<'a> Type<'a> for Span<'a, DefineLambdaType> {
    fn typecheck_call(
        self: Rc<Self>,
        _context: &mut Vec<(String, Rc<dyn Type<'a> + 'a>)>,
        args: &[Span<'a, Ast<'a>>],
    ) -> EvaluateResult<'a, Rc<dyn Type<'a> + 'a>> {
        let [variable, body]: &[Span<'a, Ast<'a>>; 2] = args.try_into().or_else(|_| {
            Err(EvaluateError {
                location: None,
                reason: "expected exactly two arguments".to_string().into(),
            })
        })?;
        let variable = match variable.inner {
            Ast::Identifier(identifier) => identifier,
            _ => Err(EvaluateError {
                location: None,
                reason: "expected argument identifier".to_string().into(),
            })?,
        };
        Ok(Rc::new(Span {
            inner: LambdaType::<'_> {
                variable: variable.to_string(),
                body: body.clone(),
            },
            full_string: "lambda", // TODO FIXME fix span info to whole list?
            string: "lambda",
        }))
    }

    fn span(&self) -> Span<'a, ()> {
        Span {
            inner: (),
            full_string: self.full_string,
            string: self.string,
        }
    }
}

pub fn evaluate<'a>(value: Span<'a, Ast<'a>>) -> EvaluateResult<'a, Rc<dyn Value<'a> + 'a>> {
    let mut context: Vec<(String, Rc<dyn Value>)> = vec![
        (
            "lambda".to_string(),
            Rc::new(Span {
                inner: DefineLambdaValue,
                full_string: "lambda",
                string: "lambda",
            }),
        ),
        (
            "add".to_string(),
            Rc::new(Span {
                inner: AddLambdaValue,
                full_string: "add",
                string: "add",
            }),
        ),
    ];
    evaluate_with_context(&mut context, value)
}

pub fn typecheck<'a>(value: Span<'a, Ast<'a>>) -> EvaluateResult<'a, Rc<dyn Type<'a> + 'a>> {
    let mut context: Vec<(String, Rc<dyn Type>)> = vec![
        (
            "lambda".to_string(),
            Rc::new(Span {
                inner: DefineLambdaType,
                full_string: "lambda",
                string: "lambda",
            }),
        ),
        (
            "add".to_string(),
            Rc::new(Span {
                inner: AddLambdaType,
                full_string: "add",
                string: "add",
            }),
        ),
    ];
    typecheck_with_context(&mut context, value)
}

fn resolve_identifier<'a, T: Clone>(
    context: &mut [(String, T)],
    identifier: Span<'a, &'a str>,
) -> EvaluateResult<'a, T> {
    context
        .iter()
        .rev()
        .find(|(ident, _)| identifier.inner == ident)
        .map(|(_ident, value)| value)
        .ok_or(EvaluateError {
            location: Some(Span {
                full_string: identifier.full_string,
                string: identifier.string,
                inner: (),
            }),
            reason: format!("could not find identfier {}", identifier.string).into(),
        })
        .cloned()
}

pub fn typecheck_with_context<'a>(
    context: &mut Vec<(String, Rc<dyn Type<'a> + 'a>)>,
    _type: Span<'a, Ast<'a>>,
) -> EvaluateResult<'a, Rc<dyn Type<'a> + 'a>> {
    match _type.inner {
        Ast::Number(number) => Ok(Rc::new(Span {
            inner: IntegerType(Some(number)),
            full_string: _type.full_string,
            string: _type.string,
        })),
        Ast::String(string) => Ok(Rc::new(Span {
            inner: StringType(Some(string.to_string())),
            full_string: _type.full_string,
            string: _type.string,
        })),
        Ast::Identifier(identifier) => resolve_identifier(
            context,
            Span {
                full_string: _type.full_string,
                string: _type.string,
                inner: identifier,
            },
        ),
        Ast::List(elements) => {
            let (callable, args) = elements.split_first().ok_or(EvaluateError {
                location: None,
                reason: "can't call an empty list".to_string().into(),
            })?;
            let callable = match callable.inner {
                Ast::Identifier(identifier) => resolve_identifier(
                    context,
                    Span {
                        full_string: callable.full_string,
                        string: callable.string,
                        inner: identifier,
                    },
                ),
                Ast::List(_) => typecheck_with_context(context, callable.clone()),
                _ => Err(EvaluateError {
                    location: None,
                    reason: "can't call a string or number".to_string().into(),
                })?,
            };
            // TODO FIXME pass the whole list to get proper span information / pass an outer span (rewrap list)
            callable?.typecheck_call(context, args)
        }
    }
}

pub fn evaluate_with_context<'a>(
    context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>,
    value: Span<'a, Ast<'a>>,
) -> EvaluateResult<'a, Rc<dyn Value<'a> + 'a>> {
    match value.inner {
        Ast::Number(number) => Ok(Rc::new(Span {
            inner: IntegerValue(number),
            full_string: value.full_string,
            string: value.string,
        })),
        Ast::String(string) => Ok(Rc::new(Span {
            inner: StringValue(string.to_string()),
            full_string: value.full_string,
            string: value.string,
        })),
        Ast::Identifier(identifier) => resolve_identifier(
            context,
            Span {
                full_string: value.full_string,
                string: value.string,
                inner: identifier,
            },
        ),
        Ast::List(elements) => {
            let (callable, args) = elements.split_first().ok_or(EvaluateError {
                location: None,
                reason: "can't call an empty list".to_string().into(),
            })?;
            let callable = match callable.inner {
                Ast::Identifier(identifier) => resolve_identifier(
                    context,
                    Span {
                        full_string: callable.full_string,
                        string: callable.string,
                        inner: identifier,
                    },
                ),
                Ast::List(_) => evaluate_with_context(context, callable.clone()),
                _ => Err(EvaluateError {
                    location: None,
                    reason: "can't call a string or number".to_string().into(),
                })?,
            };
            callable?.evaluate_call(context, args)
        }
    }
}

// cargo test -- --show-output evaluate
#[test]
fn test_primitives() {
    let span = Ast::Number(5);
    println!("{:?}", evaluate(span.into()));

    let span = Ast::String("Hallo");
    println!("{:?}", evaluate(span.into()));

    let span = Ast::Identifier("notexisting");
    println!("{:?}", evaluate(span.into()));

    let span = Ast::Identifier("lambda");
    println!("{:?}", evaluate(span.into()));

    let span = Ast::List(vec![]);
    println!("{:?}", evaluate(span.into()));

    let span = Ast::List(vec![Ast::Number(42).into()]).into();
    println!("{:?}", evaluate(span));

    let result = evaluate(
        parse_root(Span::new(
            r#"
        (lambda v v)
    "#,
        ))
        .unwrap()
        .0,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_root(Span::new(
            r#"
        (lambda 1 v)
    "#,
        ))
        .unwrap()
        .0,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_root(Span::new(
            r#"
        ((lambda v v) 1)
    "#,
        ))
        .unwrap()
        .0,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_root(Span::new(
            r#"
        (add 1 (add 1 1))
    "#,
        ))
        .unwrap()
        .0,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_root(Span::new(
            r#"
        (add 1 (add 1 ""))
    "#,
        ))
        .unwrap()
        .0,
    );
    println!("{:?}", result);

    // TODO FIXME
    // (lambda x (add 1 ""))
    // the underlying problem is that lambdas are not typechecked when not called
}
