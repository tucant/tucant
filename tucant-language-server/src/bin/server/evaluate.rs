use tucant_language_server_derive_output::{Position, Range};

use crate::parser::{Ast, Span, parse_from_str};

use std::any::Any;
use std::borrow::Cow;
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

pub trait Value: Debug {
    fn evaluate_call(
        self: Rc<Self>,
        span: Span,
        _context: &mut Vec<(String, (RcValue, Span))>,
        _args: &[(Ast, Span)],
    ) -> EvaluateResult<(RcValue, Span)> {
        Err(EvaluateError {
            location: span,
            reason: "not yet implemented".to_string().into(),
        })
    }
}

pub trait Type: Debug {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        _context: &mut Vec<(String, (RcType, Span))>,
        _args: &[(Ast, Span)],
    ) -> (
        EvaluateResult<(RcType, Span)>,
        Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
    ) {
        let val = Err(EvaluateError {
            location: span,
            reason: "not yet implemented".to_string().into(),
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
        args: &[(Ast, Span)],
    ) -> (
        EvaluateResult<(RcType, Span)>,
        Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
    ) {
        let [value]: &[(Ast, Span); 1] = match args.try_into() {
            Ok(v) => v,
            Err(_e) => {
                let val = Err(EvaluateError {
                    location: span,
                    reason: "expected exactly one argument".to_string().into(),
                });
                return (val.clone(), Box::new(std::iter::once(val)));
            }
        };
        let (_value, value_trace) = typecheck_with_context(context, value.clone());
        let return_value: EvaluateResult<(RcType, Span)> = Ok((Rc::new(IntegerType(None)), span));
        (
            return_value.clone(),
            Box::new(value_trace.chain(std::iter::once(return_value))),
        )
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
        args: &[(Ast, Span)],
    ) -> EvaluateResult<(RcValue, Span)> {
        let [left, right]: &[(Ast, Span); 2] = args.try_into().map_err(|_| EvaluateError {
            location: span,
            reason: "expected exactly two arguments".to_string().into(),
        })?;
        let left_value = evaluate_with_context(context, left.clone())?;
        let right_value = evaluate_with_context(context, right.clone())?;
        let left_value = (&left_value.0 as &dyn Any)
            .downcast_ref::<IntegerValue>()
            .ok_or(EvaluateError {
                location: left_value.1,
                reason: format!("expected integer type, got {:?}", left_value.0).into(),
            })?;
        let right_value = (&right_value.0 as &dyn Any)
            .downcast_ref::<IntegerValue>()
            .ok_or(EvaluateError {
                location: right_value.1,
                reason: format!("expected integer type, got {:?}", right_value).into(),
            })?;
        Ok((
            Rc::new(IntegerValue(
                left_value
                    .0
                    .checked_add(right_value.0)
                    .ok_or(EvaluateError {
                        location: span,
                        reason: format!(
                            "integer overflow, adding {:?} and {:?}",
                            left_value, right_value
                        )
                        .into(),
                    })?,
            )),
            span,
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
        args: &[(Ast, Span)],
    ) -> (
        EvaluateResult<(RcType, Span)>,
        Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
    ) {
        let [left, right]: &[(Ast, Span); 2] = match args.try_into() {
            Ok(v) => v,
            Err(_e) => {
                let val = Err(EvaluateError {
                    location: span,
                    reason: "expected exactly two arguments".to_string().into(),
                });
                return (val.clone(), Box::new(std::iter::once(val)));
            }
        };
        let (left_value, left_value_trace) = typecheck_with_context(context, left.clone());
        let (right_value, right_value_trace) = typecheck_with_context(context, right.clone());
        let (left_value, right_value) = match (&left_value, &right_value) {
            (Ok(ref vl), Ok(ref vr)) => {
                match (
                    (&vl as &dyn Any).downcast_ref::<IntegerType>(),
                    (&vr as &dyn Any).downcast_ref::<IntegerType>(),
                ) {
                    (Some(vl), Some(vr)) => (vl, vr),
                    (None, None) => {
                        let vall = Err(EvaluateError {
                            location: vl.1,
                            reason: format!("expected integer type, got {:?}", vl).into(),
                        });
                        let valr = Err(EvaluateError {
                            location: vr.1,
                            reason: format!("expected integer type, got {:?}", vr).into(),
                        });
                        let val = Err(EvaluateError {
                            location: span,
                            reason: "some parameters are not integers".to_string().into(),
                        });
                        return (val.clone(), Box::new(vec![val, vall, valr].into_iter()));
                    }
                    (Some(_vl), None) => {
                        let valr = Err(EvaluateError {
                            location: vr.1,
                            reason: format!("expected integer type, got {:?}", vr).into(),
                        });
                        let val = Err(EvaluateError {
                            location: span,
                            reason: "some parameters are not integers".to_string().into(),
                        });
                        return (val.clone(), Box::new(vec![val, valr].into_iter()));
                    }
                    (None, Some(_vr)) => {
                        let vall = Err(EvaluateError {
                            location: vl.1,
                            reason: format!("expected integer type, got {:?}", vl).into(),
                        });
                        let val = Err(EvaluateError {
                            location: span,
                            reason: "some parameters are not integers".to_string().into(),
                        });
                        return (val.clone(), Box::new(vec![val, vall].into_iter()));
                    }
                }
            }
            (Err(ref _e), _) => {
                return (
                    left_value,
                    Box::new(left_value_trace.chain(right_value_trace)),
                )
            }
            (_, Err(ref _e)) => {
                return (
                    right_value,
                    Box::new(left_value_trace.chain(right_value_trace)),
                )
            }
        };
        let val = left_value
            .0
            .and_then(|l| {
                right_value.0.map(|r| {
                    l.checked_add(r).ok_or(EvaluateError {
                        location: span,
                        reason: format!(
                            "integer overflow, adding {:?} and {:?}",
                            left_value, right_value
                        )
                        .into(),
                    })
                })
            })
            .transpose();
        match val {
            Ok(val) => {
                let return_value: EvaluateResult<(RcType, Span)> =
                    Ok((Rc::new(IntegerType(val)), span));
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
        args: &[(Ast, Span)],
    ) -> EvaluateResult<(RcValue, Span)> {
        let [variable_value]: &[(Ast, Span); 1] = args.try_into().map_err(|_| EvaluateError {
            location: span,
            reason: "expected exactly one argument".to_string().into(),
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
        args: &[(Ast, Span)],
    ) -> (
        EvaluateResult<(RcType, Span)>,
        Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
    ) {
        let [variable_value]: &[(Ast, Span); 1] = match args.try_into() {
            Ok(v) => v,
            Err(_) => {
                let err = Err(EvaluateError {
                    location: span,
                    reason: "expected exactly one argument".to_string().into(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };
        let (arg_value, arg_value_trace) = typecheck_with_context(context, variable_value.clone());
        if let Ok(arg_value) = arg_value {
            context.push((self.variable.clone(), arg_value));
            let return_value = typecheck_with_context(context, self.body);
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
        args: &[(Ast, Span)],
    ) -> EvaluateResult<(RcValue, Span)> {
        let [variable, body]: &[(Ast, Span); 2] = args.try_into().map_err(|_| EvaluateError {
            location: span,
            reason: "expected exactly two arguments".to_string().into(),
        })?;
        let variable = match variable.0 {
            Ast::Identifier(identifier) => identifier,
            _ => Err(EvaluateError {
                location: variable.1,
                reason: "expected argument identifier".to_string().into(),
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
        _context: &mut Vec<(String, (RcType, Span))>,
        args: &[(Ast, Span)],
    ) -> (
        EvaluateResult<(RcType, Span)>,
        Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
    ) {
        let [variable, body]: &[(Ast, Span); 2] = match args.try_into() {
            Ok(val) => val,
            Err(_) => {
                let err = Err(EvaluateError {
                    location: span,
                    reason: "expected exactly two arguments".to_string().into(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };
        let variable = match variable.0 {
            Ast::Identifier(identifier) => identifier,
            _ => {
                let err = Err(EvaluateError {
                    location: variable.1,
                    reason: "expected argument identifier".to_string().into(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };
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
            (Rc::new(AddLambdaValue), Span {
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
            }),
        ),
    ];
    evaluate_with_context(&mut context, value)
}

pub fn typecheck(
    value: (Ast, Span),
) -> (
    EvaluateResult<(RcType, Span)>,
    Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
) {
    let mut context: Vec<(String, (RcType, Span))> = vec![
        (
            "lambda".to_string(),
            (Rc::new(DefineLambdaType), Span {
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
            }),
        ),
        (
            "add".to_string(),
            (Rc::new(AddLambdaType), Span {
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
            }),
        ),
        (
            "widen-integer".to_string(),
            (Rc::new(WidenInteger), Span {
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
            }),
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
        Some(value) => Ok((value.0, value.1)),
        None => Err(EvaluateError {
            location: identifier.1,
            reason: format!("could not find identifier {}", identifier.0).into(),
        }),
    }
}

pub fn typecheck_with_context(
    context: &mut Vec<(String, (RcType, Span))>,
    _type: (Ast, Span),
) -> (
    EvaluateResult<(RcType, Span)>,
    Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
) {
    match &_type.0 {
        Ast::Number(number) => {
            let rc: EvaluateResult<(RcType, Span)> = Ok((Rc::new(IntegerType(Some(*number))), _type.1));
            (rc.clone(), Box::new(std::iter::once(rc)))
        }
        Ast::String(string) => {
            let rc: EvaluateResult<(RcType, Span)> = Ok((Rc::new(StringType(Some(string.to_string()))), _type.1));
            (rc.clone(), Box::new(std::iter::once(rc)))
        }
        Ast::Identifier(identifier) => {
            let rc = resolve_identifier_type(
                context,
                (identifier.to_string(), _type.1),
            );
            (rc.clone(), Box::new(std::iter::once(rc)))
        }
        Ast::List(elements) => {
            let (callable, args) = match elements.split_first() {
                Some(v) => v,
                None => {
                    let err = Err(EvaluateError {
                        location: _type.1,
                        reason: "can't call an empty list".to_string().into(),
                    });
                    return (err.clone(), Box::new(std::iter::once(err)));
                }
            };
            let (callable, callable_trace) = match callable.0 {
                Ast::Identifier(identifier) => {
                    let val = resolve_identifier_type(
                        context,
                        (identifier, callable.1),
                    );
                    let val: (
                        EvaluateResult<(RcType, Span)>,
                        Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
                    ) = (val.clone(), Box::new(std::iter::once(val)));
                    val
                }
                Ast::List(_) => typecheck_with_context(context, callable.clone()),
                _ => {
                    let val = Err(EvaluateError {
                        location: _type.1,
                        reason: "can't call a string or number".to_string().into(),
                    });
                    return (val.clone(), Box::new(std::iter::once(val)));
                }
            };
            // TODO FIXME pass the whole list to get proper span information / pass an outer span (rewrap list)
            match callable {
                Ok(v) => {
                    let (res, res_trace) = v.0.typecheck_call(v.1, context, args);
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
fn test_primitives() {
    use crate::parser::parse;

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
    println!("{:?}", evaluate((span, fake_span)));

    let span = Ast::String("Hallo".to_string());
    println!("{:?}", evaluate((span, fake_span)));

    let span = Ast::Identifier("notexisting".to_string());
    println!("{:?}", evaluate((span, fake_span)));

    let span = Ast::Identifier("lambda".to_string());
    println!("{:?}", evaluate((span, fake_span)));

    let span = Ast::List(vec![]);
    println!("{:?}", evaluate((span, fake_span)));

    let span = Ast::List(vec![(Ast::Number(42), fake_span)]);
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
        .unwrap()
        ,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_from_str(
            r#"
        (add 1 (add 1 1))
    "#,
        )
        .unwrap()
        ,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse_from_str(
            r#"
        (add 1 (add 1 ""))
    "#,
        )
        .unwrap()
        ,
    );
    println!("{:?}", result);

    // TODO FIXME
    // (lambda x (add 1 ""))
    // the underlying problem is that lambdas are not typechecked when not called
}
