use tucant_language_server_derive_output::{Position, Range};

use crate::parser::{Ast, Span};

use std::any::{Any};

use std::fmt::Debug;
use std::rc::Rc;

// remove the lifetimes everywhere and do more

#[derive(Debug, Clone)]
pub struct EvaluateError {
    pub location: Span,
    pub reason: String,
}

pub type EvaluateResult<V> = Result<V, EvaluateError>; // TODO FIXME maybe put the span to the outside?
pub type RcValue = Rc<dyn Value>;
pub type RcType = Rc<dyn Type>;

pub trait Value: Debug + Any {
    fn evaluate_call(
        self: Rc<Self>,
        span: Span,
        _context: &mut Vec<(String, (RcValue, Span))>,
        _args: (&[(Ast, Span)], Span),
    ) -> EvaluateResult<(RcValue, Span)> {
        Err(EvaluateError {
            location: span,
            reason: "not yet implemented".to_string(),
        })
    }
}

pub type TypecheckCall = (
    EvaluateResult<(RcType, Span)>,
    Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
);

pub trait Type: Debug + Any {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        _context: &mut Vec<(String, (RcType, Span))>,
        _args: (&[(Ast, Span)], Span),
    ) -> TypecheckCall {
        let val = Err(EvaluateError {
            location: span,
            reason: "not yet implemented".to_string(),
        });
        (val.clone(), Box::new(std::iter::once(val)))
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

impl Type for WidenInteger {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
    ) -> TypecheckCall {
        let [value]: &[(Ast, Span); 1] = match args.0.try_into() {
            Ok(v) => v,
            Err(_e) => {
                let val = Err(EvaluateError {
                    location: span,
                    reason: "expected exactly one argument".to_string(),
                });
                return (val.clone(), Box::new(std::iter::once(val)));
            }
        };
        let (value, value_trace) = typecheck_with_context(context, value.clone());
        match value {
            Ok(value) => match Rc::downcast::<IntegerType>(value.0.clone()) {
                Ok(_) => {
                    let return_value: EvaluateResult<(RcType, Span)> =
                        Ok((Rc::new(IntegerType(None)), args.1));
                    (
                        return_value.clone(),
                        Box::new(value_trace.chain(std::iter::once(return_value))),
                    )
                }
                Err(_err) => {
                    let vall = Err(EvaluateError {
                        location: value.1.clone(),
                        reason: format!("expected integer type, got {:?}", value.0),
                    });
                    let val = Err(EvaluateError {
                        location: args.1,
                        reason: "some parameters are not integers".to_string(),
                    });
                    (val.clone(), Box::new(vec![vall].into_iter()))
                }
            },
            Err(_) => (value, value_trace),
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
    ) -> EvaluateResult<(RcValue, Span)> {
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
        ))
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
        let [left, right]: &[(Ast, Span); 2] = match args.0.try_into() {
            Ok(v) => v,
            Err(_e) => {
                let val = Err(EvaluateError {
                    location: args.1,
                    reason: "expected exactly two arguments".to_string(),
                });
                return (val.clone(), Box::new(std::iter::once(val)));
            }
        };
        let (left_value, left_value_trace) = typecheck_with_context(context, left.clone());
        let (right_value, right_value_trace) = typecheck_with_context(context, right.clone());
        match (left_value.clone(), right_value.clone()) {
            (Ok(vl), Ok(vr)) => {
                let (left_value_i, right_value_i) = match (
                    Rc::downcast::<IntegerType>(vl.0.clone()).ok(),
                    Rc::downcast::<IntegerType>(vr.0.clone()).ok(),
                ) {
                    (Some(vl), Some(vr)) => (vl, vr),
                    (None, None) => {
                        let vall = Err(EvaluateError {
                            location: vl.1.clone(),
                            reason: format!("expected integer type, got {:?}", vl.0),
                        });
                        let valr = Err(EvaluateError {
                            location: vr.1.clone(),
                            reason: format!("expected integer type, got {:?}", vr.0),
                        });
                        let val = Err(EvaluateError {
                            location: args.1,
                            reason: "some parameters are not integers".to_string(),
                        });
                        return (val.clone(), Box::new(vec![vall, valr].into_iter()));
                    }
                    (Some(_vl), None) => {
                        let valr = Err(EvaluateError {
                            location: vr.1.clone(),
                            reason: format!("expected integer type, got {:?}", vr.0),
                        });
                        let val = Err(EvaluateError {
                            location: args.1,
                            reason: "some parameters are not integers".to_string(),
                        });
                        return (val.clone(), Box::new(vec![valr].into_iter()));
                    }
                    (None, Some(_vr)) => {
                        let vall = Err(EvaluateError {
                            location: vl.1.clone(),
                            reason: format!("expected integer type, got {:?}", vl.0),
                        });
                        let val = Err(EvaluateError {
                            location: args.1,
                            reason: "some parameters are not integers".to_string(),
                        });
                        return (val.clone(), Box::new(vec![vall].into_iter()));
                    }
                };
                let val = left_value_i
                    .0
                    .and_then(|l| {
                        right_value_i.0.map(|r| {
                            l.checked_add(r).ok_or(EvaluateError {
                                location: span.clone(),
                                reason: format!(
                                    "integer overflow, adding {:?} and {:?}",
                                    left_value, right_value
                                ),
                            })
                        })
                    })
                    .transpose();
                match val {
                    Ok(val) => {
                        let return_value: EvaluateResult<(RcType, Span)> =
                            Ok((Rc::new(IntegerType(val)), args.1));
                        (
                            return_value.clone(),
                            Box::new(
                                left_value_trace
                                    .chain(right_value_trace)
                                    .chain(std::iter::once(return_value)),
                            ),
                        )
                    }
                    Err(err) => {
                        let return_value: EvaluateResult<(RcType, Span)> = Err(err);
                        (
                            return_value.clone(),
                            Box::new(
                                left_value_trace
                                    .chain(right_value_trace)
                                    .chain(std::iter::once(return_value)),
                            ),
                        )
                    }
                }
            }
            (Err(ref _e), _) => (
                left_value,
                Box::new(left_value_trace.chain(right_value_trace)),
            ),
            (_, Err(ref _e)) => (
                right_value,
                Box::new(left_value_trace.chain(right_value_trace)),
            ),
        }
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
    ) -> EvaluateResult<(RcValue, Span)> {
        let [variable_value]: &[(Ast, Span); 1] = args.0.try_into().map_err(|_| EvaluateError {
            location: span,
            reason: "expected exactly one argument".to_string(),
        })?;
        let arg_value = evaluate_with_context(context, variable_value.clone())?;
        context.push((self.variable.clone(), arg_value));
        let return_value = evaluate_with_context(context, self.body.clone());
        context.pop();
        return_value
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
        let [variable_value]: &[(Ast, Span); 1] = match args.0.try_into() {
            Ok(v) => v,
            Err(_) => {
                let err = Err(EvaluateError {
                    location: span,
                    reason: "expected exactly one argument".to_string(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };
        let (arg_value, arg_value_trace) = typecheck_with_context(context, variable_value.clone());
        if let Ok(arg_value) = arg_value {
            context.push((self.variable.clone(), arg_value));
            let return_value = typecheck_with_context(context, self.body.clone());
            context.pop();
            return_value
        } else {
            (arg_value, arg_value_trace)
        }
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
    ) -> EvaluateResult<(RcValue, Span)> {
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
        let [variable, _type, body]: &[(Ast, Span); 3] = match args.0.try_into() {
            Ok(val) => val,
            Err(_) => {
                let err = Err(EvaluateError {
                    location: span,
                    reason: "expected exactly three arguments".to_string(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };
        let variable = match &variable.0 {
            Ast::Identifier(identifier) => identifier,
            _ => {
                let err = Err(EvaluateError {
                    location: variable.1.clone(),
                    reason: "expected argument identifier".to_string(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };
        let type_identifier = match &_type.0 {
            Ast::Identifier(identifier) => identifier,
            _ => {
                let err = Err(EvaluateError {
                    location: _type.1.clone(),
                    reason: "expected argument type".to_string(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };

        let param_type =
            resolve_identifier_type(context, (type_identifier.clone(), _type.1.clone()));
        let Ok(param_type) = param_type else {
            let err = Err(EvaluateError {
                location: _type.1.clone(),
                reason: "unknown argument type".to_string(),
            });
            return (err.clone(), Box::new(std::iter::once(err)));
        };
        context.push((variable.clone(), param_type));
        let (return_value, trace) = typecheck_with_context(context, body.clone());
        context.pop();

        if let Err(_err) = &return_value {
            return (return_value, trace);
        }

        let val: EvaluateResult<(RcType, Span)> = Ok((
            Rc::new(LambdaType {
                variable: variable.to_string(),
                body: body.clone(),
            }),
            span,
        ));
        (val.clone(), Box::new(std::iter::once(val)))
    }
}

pub fn evaluate(value: (Ast, Span)) -> EvaluateResult<(RcValue, Span)> {
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
) -> EvaluateResult<(RcType, Span)> {
    match context
        .iter()
        .rev()
        .find(|(ident, _)| &identifier.0 == ident)
        .map(|(_ident, value)| value)
    {
        Some(value) => Ok((value.0.clone(), identifier.1)),
        None => Err(EvaluateError {
            location: identifier.1,
            reason: format!("could not find identifier {}", identifier.0),
        }),
    }
}

pub fn typecheck_with_context(
    context: &mut Vec<(String, (RcType, Span))>,
    _type: (Ast, Span),
) -> TypecheckCall {
    match &_type.0 {
        Ast::Number(number) => {
            let rc: EvaluateResult<(RcType, Span)> =
                Ok((Rc::new(IntegerType(Some(*number))), _type.1));
            (rc.clone(), Box::new(std::iter::once(rc)))
        }
        Ast::String(string) => {
            let rc: EvaluateResult<(RcType, Span)> =
                Ok((Rc::new(StringType(Some(string.to_string()))), _type.1));
            (rc.clone(), Box::new(std::iter::once(rc)))
        }
        Ast::Identifier(identifier) => {
            let rc = resolve_identifier_type(context, (identifier.to_string(), _type.1));
            (rc.clone(), Box::new(std::iter::once(rc)))
        }
        Ast::List(elements) => {
            let (callable, args) = match elements.split_first() {
                Some(v) => v,
                None => {
                    let err = Err(EvaluateError {
                        location: _type.1,
                        reason: "can't call an empty list".to_string(),
                    });
                    return (err.clone(), Box::new(std::iter::once(err)));
                }
            };
            let (callable, callable_trace) = match &callable.0 {
                Ast::Identifier(identifier) => {
                    let val =
                        resolve_identifier_type(context, (identifier.clone(), callable.1.clone()));
                    let val: TypecheckCall = (val.clone(), Box::new(std::iter::once(val)));
                    val
                }
                Ast::List(_) => typecheck_with_context(context, callable.clone()),
                _ => {
                    let val = Err(EvaluateError {
                        location: _type.1,
                        reason: "can't call a string or number".to_string(),
                    });
                    return (val.clone(), Box::new(std::iter::once(val)));
                }
            };
            // TODO FIXME pass the whole list to get proper span information / pass an outer span (rewrap list)
            match callable {
                Ok(v) => {
                    let (res, res_trace) = v.0.typecheck_call(v.1, context, (args, _type.1));
                    (res, Box::new(callable_trace.chain(res_trace)))
                }
                e => (
                    e.clone(),
                    Box::new(callable_trace.chain(std::iter::once(e))),
                ),
            }
        }
    }
}

#[allow(clippy::all)]
pub fn evaluate_with_context(
    _context: &mut Vec<(String, (RcValue, Span))>,
    _value: (Ast, Span),
) -> EvaluateResult<(RcValue, Span)> {
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
}
