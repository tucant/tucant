use crate::parser::{Ast, Span};

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
        _context: &mut Vec<(String, Rc<dyn Value>)>,
        _args: &[(Ast, Span)],
    ) -> EvaluateResult<(RcValue, Span)> {
        Err(EvaluateError {
            location: span,
            reason: "not yet implemented".to_string().into(),
        })
    }

    fn downcast_integer_value(&self) -> Option<&IntegerValue> {
        None
    }
}

pub trait Type: Debug {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        _context: &mut Vec<(String, RcType)>,
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

impl Value for IntegerValue {
}

#[derive(Debug, Clone)]
pub struct IntegerType(Option<i64>);

impl Type for IntegerType {
}

#[derive(Debug, Clone)]
pub struct WidenInteger;

impl Type for WidenInteger {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, RcType)>,
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
        let (_value, value_trace) = typecheck_with_context(context, value);
        let return_value: EvaluateResult<(RcType, Span)> = Ok((Rc::new(IntegerType(None)), span));
        (
            return_value.clone(),
            Box::new(value_trace.chain(std::iter::once(return_value))),
        )
    }
}

#[derive(Debug)]
pub struct StringValue(String);

impl Value for StringValue {
    
}

#[derive(Debug, Clone)]
pub struct StringType(Option<String>);

impl Type for StringType {
    
}

#[derive(Debug)]
pub struct AddLambdaValue; // if this doesn't work maybe just add a span to every one of them and add a methdod that returns the span?

impl Value for AddLambdaValue {
    fn evaluate_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, Rc<dyn Value >)>,
        args: &[(Ast, Span)],
    ) -> EvaluateResult<Rc<dyn Value >> {
        let [left, right]: &[Span<Ast>; 2] =
            args.try_into().map_err(|_| EvaluateError {
                location: None,
                reason: "expected exactly two arguments".to_string().into(),
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

}

#[derive(Debug, Clone)]
pub struct AddLambdaType;

impl Type for AddLambdaType {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        context: &mut Vec<(String, Rc<dyn Type >)>,
        args: &[(Ast, Span)],
    ) -> (
        EvaluateResult<RcType>,
        Box<dyn Iterator<Item = EvaluateResult<RcType>> >,
    ) {
        let [left, right]: &[Span<Ast>; 2] = match args.try_into() {
            Ok(v) => v,
            Err(_e) => {
                let val = Err(EvaluateError {
                    location: None,
                    reason: "expected exactly two arguments".to_string().into(),
                });
                return (val.clone(), Box::new(std::iter::once(val)));
            }
        };
        let (left_value, left_value_trace) = typecheck_with_context(context, left);
        let (right_value, right_value_trace) = typecheck_with_context(context, right);
        let (left_value, right_value) = match (&left_value, &right_value) {
            (Ok(ref vl), Ok(ref vr)) => {
                match (vl.downcast_integer_type(), vr.downcast_integer_type()) {
                    (Some(vl), Some(vr)) => (vl, vr),
                    (None, None) => {
                        let vall = Err(EvaluateError {
                            location: Some(vl.span()),
                            reason: format!("expected integer type, got {:?}", vl).into(),
                        });
                        let valr = Err(EvaluateError {
                            location: Some(vr.span()),
                            reason: format!("expected integer type, got {:?}", vr).into(),
                        });
                        let val = Err(EvaluateError {
                            location: Some(self.span()), // it needs its own span
                            reason: "some parameters are not integers".to_string().into(),
                        });
                        return (val.clone(), Box::new(vec![val, vall, valr].into_iter()));
                    }
                    (Some(_vl), None) => {
                        let valr = Err(EvaluateError {
                            location: Some(vr.span()),
                            reason: format!("expected integer type, got {:?}", vr).into(),
                        });
                        let val = Err(EvaluateError {
                            location: Some(self.span()),
                            reason: "some parameters are not integers".to_string().into(),
                        });
                        return (val.clone(), Box::new(vec![val, valr].into_iter()));
                    }
                    (None, Some(_vr)) => {
                        let vall = Err(EvaluateError {
                            location: Some(vl.span()),
                            reason: format!("expected integer type, got {:?}", vl).into(),
                        });
                        let val = Err(EvaluateError {
                            location: Some(self.span()),
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
                        location: None,
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
                let return_value: EvaluateResult<RcType> = Ok(Rc::new(Span {
                    inner: IntegerType(val),
                    full_string: "",
                    string: "",
                }));
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
                let return_value: EvaluateResult<RcType> = Err(err);
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
        context: &mut Vec<(String, Rc<dyn Value >)>,
        args: &[(Ast, Span)],
    ) -> EvaluateResult<Rc<dyn Value >> {
        let [variable_value]: &[Span<Ast>; 1] =
            args.try_into().map_err(|_| EvaluateError {
                location: None,
                reason: "expected exactly one argument".to_string().into(),
            })?;
        let arg_value = evaluate_with_context(context, variable_value.clone())?;
        context.push((self.inner.variable.clone(), arg_value));
        let return_value = evaluate_with_context(context, self.inner.body.clone());
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
        context: &mut Vec<(String, Rc<dyn Type >)>,
        args: &[Ast],
    ) -> (
        EvaluateResult<RcType>,
        Box<dyn Iterator<Item = EvaluateResult<RcType>> >,
    ) {
        let [variable_value]: &[Span<Ast>; 1] = match args.try_into() {
            Ok(v) => v,
            Err(_) => {
                let err = Err(EvaluateError {
                    location: None,
                    reason: "expected exactly one argument".to_string().into(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };
        let (arg_value, arg_value_trace) = typecheck_with_context(context, variable_value);
        if let Ok(arg_value) = arg_value {
            context.push((self.inner.variable.clone(), arg_value));
            let return_value = typecheck_with_context(context, &self.inner.body);
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
        _context: &mut Vec<(String, Rc<dyn Value >)>,
        args: &[(Ast, Span)],
    ) -> EvaluateResult<Rc<dyn Value >> {
        let [variable, body]: &[Span<Ast>; 2] =
            args.try_into().map_err(|_| EvaluateError {
                location: None,
                reason: "expected exactly two arguments".to_string().into(),
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

}

#[derive(Debug, Clone)]
pub struct DefineLambdaType;

impl Type for DefineLambdaType {
    fn typecheck_call(
        self: Rc<Self>,
        span: Span,
        _context: &mut Vec<(String, Rc<dyn Type >)>,
        args: &[(Ast, Span)],
    ) -> (
        EvaluateResult<RcType>,
        Box<dyn Iterator<Item = EvaluateResult<RcType>> >,
    ) {
        let [variable, body]: &[Span<Ast>; 2] = match args.try_into() {
            Ok(val) => val,
            Err(_) => {
                let err = Err(EvaluateError {
                    location: None,
                    reason: "expected exactly two arguments".to_string().into(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };
        let variable = match variable.inner {
            Ast::Identifier(identifier) => identifier,
            _ => {
                let err = Err(EvaluateError {
                    location: None,
                    reason: "expected argument identifier".to_string().into(),
                });
                return (err.clone(), Box::new(std::iter::once(err)));
            }
        };
        let val: EvaluateResult<RcType> = Ok(Rc::new(Span {
            inner: LambdaType::<'_> {
                variable: variable.to_string(),
                body: body.clone(),
            },
            full_string: "lambda", // TODO FIXME fix span info to whole list?
            string: "lambda",
        }));
        (val.clone(), Box::new(std::iter::once(val)))
    }

}

pub fn evaluate(value: (Ast, Span)) -> EvaluateResult<RcValue> {
    let mut context: Vec<(String, Rc<dyn Value>)> = vec![
        (
            "lambda".to_string(),
            Rc::new(Span {
                inner: DefineLambdaValue,
                full_string: "lambda", // this shouldn't really have a location but als it should in some kind of system library, maybe spans should have filenames
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

pub fn typecheck(
    value: (Ast, Span),
) -> (
    EvaluateResult<(RcType, Span)>,
    Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
) {
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
        (
            "widen-integer".to_string(),
            Rc::new(Span {
                inner: WidenInteger,
                full_string: "widen-integer",
                string: "widen-integer",
            }),
        ),
    ];
    typecheck_with_context(&mut context, value)
}

// TODO FIXME probably return an IdentiferType that also contains the location of the definition
fn resolve_identifier_type(
    context: &mut [(String, Rc<dyn Type >)],
    identifier: (String, Span),
) -> EvaluateResult<Rc<dyn Type >> {
    match context
        .iter()
        .rev()
        .find(|(ident, _)| identifier.inner == ident)
        .map(|(_ident, value)| value)
    {
        Some(value) => Ok(value.clone().with_span(Span {
            inner: (),
            full_string: identifier.full_string,
            string: identifier.string,
        })),
        None => Err(EvaluateError {
            location: Some(Span {
                full_string: identifier.full_string,
                string: identifier.string,
                inner: (),
            }),
            reason: format!("could not find identfier {}", identifier.string).into(),
        }),
    }
}

pub fn typecheck_with_context(
    context: &mut Vec<(String, Rc<dyn Type >)>,
    _type: &(Ast, Span),
) -> (
    EvaluateResult<(RcType, Span)>,
    Box<dyn Iterator<Item = EvaluateResult<(RcType, Span)>>>,
) {
    match &_type.inner {
        Ast::Number(number) => {
            let rc: EvaluateResult<Rc<(dyn Type )>> = Ok(Rc::new(Span {
                inner: IntegerType(Some(*number)),
                full_string: _type.full_string,
                string: _type.string,
            }));
            (rc.clone(), Box::new(std::iter::once(rc)))
        }
        Ast::String(string) => {
            let rc: EvaluateResult<Rc<(dyn Type )>> = Ok(Rc::new(Span {
                inner: StringType(Some(string.to_string())),
                full_string: _type.full_string,
                string: _type.string,
            }));
            (rc.clone(), Box::new(std::iter::once(rc)))
        }
        Ast::Identifier(identifier) => {
            let rc = resolve_identifier_type(
                context,
                Span {
                    full_string: _type.full_string,
                    string: _type.string,
                    inner: identifier,
                },
            );
            (rc.clone(), Box::new(std::iter::once(rc)))
        }
        Ast::List(elements) => {
            let (callable, args) = match elements.split_first() {
                Some(v) => v,
                None => {
                    let err = Err(EvaluateError {
                        location: None,
                        reason: "can't call an empty list".to_string().into(),
                    });
                    return (err.clone(), Box::new(std::iter::once(err)));
                }
            };
            let (callable, callable_trace) = match callable.inner {
                Ast::Identifier(identifier) => {
                    let val = resolve_identifier_type(
                        context,
                        Span {
                            full_string: callable.full_string,
                            string: callable.string,
                            inner: identifier,
                        },
                    );
                    let val: (
                        EvaluateResult<RcType>,
                        Box<dyn Iterator<Item = EvaluateResult<RcType>>>,
                    ) = (val.clone(), Box::new(std::iter::once(val)));
                    val
                }
                Ast::List(_) => typecheck_with_context(context, callable),
                _ => {
                    let val = Err(EvaluateError {
                        location: None,
                        reason: "can't call a string or number".to_string().into(),
                    });
                    return (val.clone(), Box::new(std::iter::once(val)));
                }
            };
            // TODO FIXME pass the whole list to get proper span information / pass an outer span (rewrap list)
            match callable {
                Ok(v) => {
                    let (res, res_trace) = v.typecheck_call(context, args);
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
    _context: &mut Vec<(String, Rc<dyn Value >)>,
    _value: (Ast, Span),
) -> EvaluateResult<Rc<dyn Value >> {
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

    let span = Ast::Number(5);
    println!("{:?}", evaluate(span.into()));

    let span = Ast::String("Hallo".to_string());
    println!("{:?}", evaluate(span.into()));

    let span = Ast::Identifier("notexisting".to_string());
    println!("{:?}", evaluate(span.into()));

    let span = Ast::Identifier("lambda".to_string());
    println!("{:?}", evaluate(span.into()));

    let span = Ast::List(vec![]);
    println!("{:?}", evaluate(span.into()));

    let span = Ast::List(vec![Ast::Number(42).into()]).into();
    println!("{:?}", evaluate(span));

    let result = evaluate(
        parse(Span::new(
            r#"
        (lambda v v)
    "#,
        ))
        .unwrap()
        .0,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse(Span::new(
            r#"
        (lambda 1 v)
    "#,
        ))
        .unwrap()
        .0,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse(Span::new(
            r#"
        ((lambda v v) 1)
    "#,
        ))
        .unwrap()
        .0,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse(Span::new(
            r#"
        (add 1 (add 1 1))
    "#,
        ))
        .unwrap()
        .0,
    );
    println!("{:?}", result);

    let result = evaluate(
        parse(Span::new(
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
