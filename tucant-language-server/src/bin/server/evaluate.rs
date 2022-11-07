use tucant_language_server_derive_output::{Position, Range};

use crate::parser::{Ast, Span};

use std::any::Any;

use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct EvaluateError {
    pub location: Span,
    pub reason: String,
}

pub type RcValue = Rc<dyn Value>;
pub type RcType = Rc<dyn Type>;

pub type GenericCall<T> = Result<
    (
        (T, Span),
        Box<dyn Iterator<Item = Result<(T, Span), EvaluateError>>>,
    ),
    Box<dyn Iterator<Item = Result<(T, Span), EvaluateError>>>,
>;

pub type EvaluateCall = GenericCall<RcValue>;

pub type TypecheckCall = GenericCall<RcType>;

pub trait Value: Debug + Any {
    fn evaluate_call(
        self: Rc<Self>,
        span: Span,
        _context: &mut Vec<(String, (RcValue, Span))>,
        _args: (&[(Ast, Span)], Span),
    ) -> EvaluateCall {
        /*Err(EvaluateError {
            location: span,
            reason: "not yet implemented".to_string(),
        });*/
        todo!()
    }
}

pub trait Type: Debug + Any {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        _context: &mut Vec<(String, (RcType, Span))>,
        _args: (&[(Ast, Span)], Span),
    ) -> TypecheckCall {
        let val = EvaluateError {
            location: span,
            reason: "not yet implemented".to_string(),
        };
        Err(Box::new(std::iter::once(Err(val))))
    }
}

#[derive(Debug)]
pub struct IntegerValue(i64);

impl Value for IntegerValue {}

#[derive(Debug, Clone)]
pub struct IntegerType(Option<i64>);

impl Type for IntegerType {}

#[derive(Debug, Clone)]
pub struct WidenInteger;

fn expect_n<T: 'static, const N: usize>(
    args: (&[(Ast, Span)], Span),
) -> Result<&[(Ast, Span); N], Box<dyn Iterator<Item = Result<(T, Span), EvaluateError>>>> {
    match TryInto::<&[(Ast, Span); N]>::try_into(args.0) {
        Ok(v) => Ok(v),
        Err(err) => Err(Box::new(std::iter::once(Err(EvaluateError {
            location: args.1,
            reason: format!("expected exactly {} arguments", N).to_string(),
        })))),
    }
}

impl Type for WidenInteger {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
    ) -> TypecheckCall {
        let [value]: &[(Ast, Span); 1] = expect_n(args.clone())?;
        let (value, value_trace) = typecheck_with_context(context, value.clone())?;
        match Rc::downcast::<IntegerType>(value.0.clone()) {
            Ok(_) => {
                let return_value: (RcType, Span) = (Rc::new(IntegerType(None)), args.1);
                Ok((
                    return_value.clone(),
                    Box::new(value_trace.chain(std::iter::once(Ok(return_value)))),
                ))
            }
            Err(_err) => {
                let vall = Err(EvaluateError {
                    location: value.1.clone(),
                    reason: format!("expected integer type, got {:?}", value.0),
                });
                Err(Box::new(vec![vall].into_iter()))
            }
        }
    }
}

#[derive(Debug)]
pub struct StringValue(String);

impl Value for StringValue {}

#[derive(Debug, Clone)]
pub struct StringType(Option<String>);

impl Type for StringType {}

#[derive(Debug)]
pub struct AddLambdaValue; // if this doesn't work maybe just add a span to every one of them and add a methdod that returns the span?

impl Value for AddLambdaValue {
    fn evaluate_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, (RcValue, Span))>,
        args: (&[(Ast, Span)], Span),
    ) -> EvaluateCall {
        /*
        let [left, right]: &[(Ast, Span); 2] = args.0.try_into().map_err(|_| EvaluateError {
            location: span.clone(),
            reason: "expected exactly two arguments".to_string(),
        })?;
        let left_value = evaluate_with_context(context, left.clone())?;
        let right_value = evaluate_with_context(context, right.clone())?;
        let left_value = (&left_value.0 as &dyn Any)
            .downcast_ref::<IntegerValue>()
            .ok_or(EvaluateError {
                location: left_value.1,
                reason: format!("expected integer type, got {:?}", left_value.0),
            })?;
        let right_value = (&right_value.0 as &dyn Any)
            .downcast_ref::<IntegerValue>()
            .ok_or(EvaluateError {
                location: right_value.1.clone(),
                reason: format!("expected integer type, got {:?}", right_value),
            })?;
        Ok((
            Rc::new(IntegerValue(
                left_value
                    .0
                    .checked_add(right_value.0)
                    .ok_or(EvaluateError {
                        location: span.clone(),
                        reason: format!(
                            "integer overflow, adding {:?} and {:?}",
                            left_value, right_value
                        ),
                    })?,
            )),
            args.1,
        ));
        */
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct AddLambdaType;

impl Type for AddLambdaType {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
    ) -> TypecheckCall {
        let [left, right]: &[(Ast, Span); 2] = expect_n(args)?;
        let (left_value, left_value_trace) = typecheck_with_context(context, left.clone())?;
        let (right_value, right_value_trace) = typecheck_with_context(context, right.clone())?;
        let left_value = Rc::downcast::<IntegerType>(left_value.0.clone()).map_err(|err| {
            let val: Box<(dyn Iterator<Item = Result<(Rc<(dyn Type)>, Span), EvaluateError>>)> =
                Box::new(std::iter::once(Err(EvaluateError {
                    location: left_value.1.clone(),
                    reason: format!("expected integer type, got {:?}", left_value.0),
                })));
            val
        })?;
        let right_value = Rc::downcast::<IntegerType>(right_value.0.clone()).map_err(|err| {
            Box::new(std::iter::once(Err(EvaluateError {
                location: right_value.1.clone(),
                reason: format!("expected integer type, got {:?}", right_value.0),
            }))) as Box<(dyn Iterator<Item = _>)>
        })?;
        let val = left_value
            .0
            .and_then(|l| {
                right_value.0.map(|r| {
                    l.checked_add(r).ok_or_else(|| {
                        Box::new(std::iter::once(Err(EvaluateError {
                            location: span.clone(),
                            reason: format!(
                                "integer overflow, adding {:?} and {:?}",
                                left_value, right_value
                            ),
                        }))) as Box<(dyn Iterator<Item = _>)>
                    })
                })
            })
            .transpose()?;
        let res = (Rc::new(IntegerType(val)) as RcType, span);
        Ok((res.clone(), Box::new(std::iter::once(Ok(res)))))
    }
}

#[derive(Debug)]
pub struct LambdaValue {
    variable: String,
    body: (Ast, Span),
}

impl Value for LambdaValue {
    fn evaluate_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, (RcValue, Span))>,
        args: (&[(Ast, Span)], Span),
    ) -> EvaluateCall {
        /*
        let [variable_value]: &[(Ast, Span); 1] = args.0.try_into().map_err(|_| EvaluateError {
            location: span,
            reason: "expected exactly one argument".to_string(),
        })?;
        let arg_value = evaluate_with_context(context, variable_value.clone())?;
        context.push((self.variable.clone(), arg_value));
        let return_value = evaluate_with_context(context, self.body.clone());
        context.pop();
        return_value
        */
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct LambdaType {
    variable: String,
    body: (Ast, Span),
}

impl Type for LambdaType {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
    ) -> TypecheckCall {
        let [variable_value]: &[(Ast, Span); 1] = expect_n(args)?;
        let (arg_value, arg_value_trace) = typecheck_with_context(context, variable_value.clone())?;
        context.push((self.variable.clone(), arg_value)); // TODO FIXME make this in some way you can't forget popping on drop? (like try syntax?)
        let return_value = typecheck_with_context(context, self.body.clone());
        context.pop();
        return_value
    }
}

#[derive(Debug)]
pub struct DefineLambdaValue;

impl Value for DefineLambdaValue {
    fn evaluate_call(
        self: Rc<Self>,
        span: Span,
        _context: &mut Vec<(String, (RcValue, Span))>,
        args: (&[(Ast, Span)], Span),
    ) -> EvaluateCall {
        /*
        let [variable, _type, body]: &[(Ast, Span); 3] =
            args.0.try_into().map_err(|_| EvaluateError {
                location: span.clone(),
                reason: "expected exactly three arguments".to_string(),
            })?;
        let variable = match &variable.0 {
            Ast::Identifier(identifier) => identifier,
            _ => Err(EvaluateError {
                location: variable.1.clone(),
                reason: "expected argument identifier".to_string(),
            })?,
        };
        let _type = match &_type.0 {
            Ast::Identifier(identifier) => identifier,
            _ => Err(EvaluateError {
                location: _type.1.clone(),
                reason: "expected argument type".to_string(),
            })?,
        };
        Ok((
            Rc::new(LambdaValue {
                variable: variable.to_string(),
                body: body.clone(),
            }),
            span,
        ))
        */
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct DefineLambdaType;

impl Type for DefineLambdaType {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
    ) -> TypecheckCall {
        let [variable, _type, body]: &[(Ast, Span); 3] = expect_n(args)?;
        let variable = match &variable.0 {
            Ast::Identifier(identifier) => identifier,
            _ => {
                let err = Err(EvaluateError {
                    location: variable.1.clone(),
                    reason: "expected argument identifier".to_string(),
                });
                return Err(Box::new(std::iter::once(err)));
            }
        };
        let type_identifier = match &_type.0 {
            Ast::Identifier(identifier) => identifier,
            _ => {
                let err = Err(EvaluateError {
                    location: _type.1.clone(),
                    reason: "expected argument type".to_string(),
                });
                return Err(Box::new(std::iter::once(err)));
            }
        };
        let (param_type, trace) =
            resolve_identifier_type(context, (type_identifier.clone(), _type.1.clone()))?;
        context.push((variable.clone(), param_type));
        let return_value = typecheck_with_context(context, body.clone());
        context.pop();
        let return_value = return_value?;

        let val = (
            Rc::new(LambdaType {
                variable: variable.to_string(),
                body: body.clone(),
            }) as RcType,
            span,
        );
        Ok((val.clone(), Box::new(std::iter::once(Ok(val)))))
    }
}

pub fn evaluate(value: (Ast, Span)) -> EvaluateCall {
    let mut context: Vec<(String, (RcValue, Span))> = vec![
        (
            "lambda".to_string(),
            (
                Rc::new(DefineLambdaValue),
                Span {
                    filename: "<builtin>".to_string(),
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
                },
            ),
        ),
        (
            "add".to_string(),
            (
                Rc::new(AddLambdaValue),
                Span {
                    filename: "<builtin>".to_string(),
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
                },
            ),
        ),
    ];
    evaluate_with_context(&mut context, value)
}

pub fn typecheck(value: (Ast, Span)) -> TypecheckCall {
    let mut context: Vec<(String, (RcType, Span))> = vec![
        (
            "lambda".to_string(),
            (
                Rc::new(DefineLambdaType),
                Span {
                    filename: "<builtin>".to_string(),
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
                },
            ),
        ),
        (
            "add".to_string(),
            (
                Rc::new(AddLambdaType),
                Span {
                    filename: "<builtin>".to_string(),
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
                },
            ),
        ),
        (
            "widen-integer".to_string(),
            (
                Rc::new(WidenInteger),
                Span {
                    filename: "<builtin>".to_string(),
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
                },
            ),
        ),
        (
            "integer-type".to_string(),
            (
                Rc::new(IntegerType(None)),
                Span {
                    filename: "<builtin>".to_string(),
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
                },
            ),
        ),
    ];
    typecheck_with_context(&mut context, value)
}

// TODO FIXME probably return an IdentiferType that also contains the location of the definition
fn resolve_identifier_type(
    context: &mut [(String, (RcType, Span))],
    identifier: (String, Span),
) -> TypecheckCall {
    match context
        .iter()
        .rev()
        .find(|(ident, _)| &identifier.0 == ident)
        .map(|(_ident, value)| value)
    {
        Some(value) => Ok((value.clone(), Box::new(std::iter::once(Ok(value.clone()))))),
        None => Err(Box::new(std::iter::once(Err(EvaluateError {
            location: identifier.1,
            reason: format!("could not find identifier {}", identifier.0),
        })))),
    }
}

pub fn typecheck_with_context(
    context: &mut Vec<(String, (RcType, Span))>,
    _type: (Ast, Span),
) -> TypecheckCall {
    match &_type.0 {
        Ast::Number(number) => {
            let rc = (Rc::new(IntegerType(Some(*number))) as RcType, _type.1);
            Ok((rc.clone(), Box::new(std::iter::once(Ok(rc)))))
        }
        Ast::String(string) => {
            let rc = (
                Rc::new(StringType(Some(string.to_string()))) as RcType,
                _type.1,
            );
            Ok((rc.clone(), Box::new(std::iter::once(Ok(rc)))))
        }
        Ast::Identifier(identifier) => {
            resolve_identifier_type(context, (identifier.to_string(), _type.1))
        }
        Ast::List(elements) => {
            let (callable, args) = match elements.split_first() {
                Some(v) => v,
                None => {
                    let err = Err(EvaluateError {
                        location: _type.1,
                        reason: "can't call an empty list".to_string(),
                    });
                    return Err(Box::new(std::iter::once(err)));
                }
            };
            let (callable, callable_trace) = match &callable.0 {
                Ast::Identifier(identifier) => {
                    resolve_identifier_type(context, (identifier.clone(), callable.1.clone()))?
                }
                Ast::List(_) => typecheck_with_context(context, callable.clone())?,
                _ => {
                    let val = Err(EvaluateError {
                        location: _type.1,
                        reason: "can't call a string or number".to_string(),
                    });
                    return Err(Box::new(std::iter::once(val)));
                }
            };
            callable
                .0
                .typecheck_call(callable.1, context, (args, _type.1))
        }
    }
}

#[allow(clippy::all)]
pub fn evaluate_with_context(
    _context: &mut Vec<(String, (RcValue, Span))>,
    _value: (Ast, Span),
) -> EvaluateCall {
    todo!()
    /*
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
    */
}

// cargo test -- --show-output evaluate
#[test]
#[ignore = "not yet implemented"]
fn test_primitives() {
    /*
    use crate::parser::parse_from_str;

    let fake_span = Span {
        filename: "<fake>".to_string(),
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
    };
    let span = Ast::Number(5);
    println!("{:?}", evaluate((span, fake_span.clone())));

    let span = Ast::String("Hallo".to_string());
    println!("{:?}", evaluate((span, fake_span.clone())));

    let span = Ast::Identifier("notexisting".to_string());
    println!("{:?}", evaluate((span, fake_span.clone())));

    let span = Ast::Identifier("lambda".to_string());
    println!("{:?}", evaluate((span, fake_span.clone())));

    let span = Ast::List(vec![]);
    println!("{:?}", evaluate((span, fake_span.clone())));

    let span = Ast::List(vec![(Ast::Number(42), fake_span.clone())]);
    println!("{:?}", evaluate((span, fake_span)));

    let result = evaluate(
        parse_from_str(
            r#"
        (lambda v v)
    "#,
        )
        .unwrap(),
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_from_str(
            r#"
        (lambda 1 v)
    "#,
        )
        .unwrap(),
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_from_str(
            r#"
        ((lambda v v) 1)
    "#,
        )
        .unwrap(),
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_from_str(
            r#"
        (add 1 (add 1 1))
    "#,
        )
        .unwrap(),
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_from_str(
            r#"
        (add 1 (add 1 ""))
    "#,
        )
        .unwrap(),
    );
    println!("{:?}", result);

    // TODO FIXME
    // (lambda x (add 1 ""))
    // the underlying problem is that lambdas are not typechecked when not called
    */
}
