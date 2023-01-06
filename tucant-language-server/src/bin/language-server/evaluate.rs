#![allow(clippy::result_unit_err)]
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

pub type GenericCall<T> = Result<(T, Span), ()>;

pub type EvaluateCall = GenericCall<RcValue>;

pub type TypecheckCall = GenericCall<RcType>;

pub type TypeTrace = Vec<Result<(RcType, Span), EvaluateError>>;

pub trait Value: Debug + Any {
    fn evaluate_call(
        self: Rc<Self>,
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
        _context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
        type_trace: &mut TypeTrace,
    ) -> TypecheckCall {
        let val = EvaluateError {
            location: args.1,
            reason: "not yet implemented".to_string(),
        };
        type_trace.push(Err(val));
        Err(())
    }

    fn downcast_integer_type(self: Rc<Self>) -> Result<Rc<IntegerType>, ()> {
        Err(())
    }
}

#[derive(Debug)]
pub struct IntegerValue(i64);

impl Value for IntegerValue {}

#[derive(Debug, Clone)]
pub struct IntegerType(Option<i64>);

impl Type for IntegerType {
    fn downcast_integer_type(self: Rc<Self>) -> Result<Rc<IntegerType>, ()> {
        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub struct WidenInteger;

fn expect_n<'a, T: 'static, const N: usize>(
    args: (&'a [(Ast, Span)], Span),
    trace: &mut Vec<Result<(T, Span), EvaluateError>>,
) -> Result<&'a [(Ast, Span); N], ()> {
    match TryInto::<&[(Ast, Span); N]>::try_into(args.0) {
        Ok(v) => Ok(v),
        Err(_err) => {
            trace.push(Err(EvaluateError {
                location: args.1,
                reason: format!("expected exactly {N} arguments"),
            }));
            Err(())
        }
    }
}

impl Type for WidenInteger {
    fn typecheck_call(
        self: Rc<Self>,
        context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
        trace: &mut TypeTrace,
    ) -> TypecheckCall {
        let [value]: &[(Ast, Span); 1] = expect_n(args.clone(), trace)?;
        let value = typecheck_with_context(context, value.clone(), trace)?;
        match value.0.clone().downcast_integer_type() {
            Ok(_) => {
                let return_value: (RcType, Span) = (Rc::new(IntegerType(None)), args.1);
                trace.push(Ok(return_value.clone()));
                Ok(return_value.clone())
            }
            Err(_err) => {
                let vall = Err(EvaluateError {
                    location: value.1.clone(),
                    reason: format!("expected integer type, got {:?}", value.0),
                });
                trace.push(vall);
                Err(())
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
        _context: &mut Vec<(String, (RcValue, Span))>,
        _args: (&[(Ast, Span)], Span),
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
        context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
        trace: &mut TypeTrace,
    ) -> TypecheckCall {
        let [left, right]: &[(Ast, Span); 2] = expect_n(args.clone(), trace)?;
        let left_value = typecheck_with_context(context, left.clone(), trace)?;
        let right_value = typecheck_with_context(context, right.clone(), trace)?;
        let left_value = left_value
            .0
            .clone()
            .downcast_integer_type()
            .map_err(|_err| {
                trace.push(Err(EvaluateError {
                    location: left_value.1.clone(),
                    reason: format!("expected integer type, got {:?}", left_value.0),
                }));
            })?;
        let right_value = right_value
            .0
            .clone()
            .downcast_integer_type()
            .map_err(|_err| {
                trace.push(Err(EvaluateError {
                    location: right_value.1.clone(),
                    reason: format!("expected integer type, got {:?}", right_value.0),
                }));
            })?;
        let val = left_value
            .0
            .and_then(|l| {
                right_value.0.map(|r| {
                    l.checked_add(r).ok_or_else(|| {
                        trace.push(Err(EvaluateError {
                            location: args.1.clone(),
                            reason: format!(
                                "integer overflow, adding {left_value:?} and {right_value:?}"
                            ),
                        }));
                    })
                })
            })
            .transpose()?;
        let res = (Rc::new(IntegerType(val)) as RcType, args.1.clone());
        trace.push(Ok(res.clone()));
        Ok(res)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LambdaValue {
    variable: String,
    body: (Ast, Span),
}

impl Value for LambdaValue {
    fn evaluate_call(
        self: Rc<Self>,
        _context: &mut Vec<(String, (RcValue, Span))>,
        _args: (&[(Ast, Span)], Span),
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
        context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
        trace: &mut TypeTrace,
    ) -> TypecheckCall {
        let [variable_value]: &[(Ast, Span); 1] = expect_n(args, trace)?;
        let arg_value = typecheck_with_context(context, variable_value.clone(), trace)?;
        context.push((self.variable.clone(), arg_value)); // TODO FIXME make this in some way you can't forget popping on drop? (like try syntax?)
        let return_value = typecheck_with_context(context, self.body.clone(), trace);
        context.pop();
        let return_value = return_value?;
        trace.push(Ok(return_value.clone()));
        Ok(return_value)
    }
}

#[derive(Debug)]
pub struct DefineLambdaValue;

impl Value for DefineLambdaValue {
    fn evaluate_call(
        self: Rc<Self>,
        _context: &mut Vec<(String, (RcValue, Span))>,
        _args: (&[(Ast, Span)], Span),
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
        context: &mut Vec<(String, (RcType, Span))>,
        args: (&[(Ast, Span)], Span),
        trace: &mut TypeTrace,
    ) -> TypecheckCall {
        let [variable, _type, body]: &[(Ast, Span); 3] = expect_n(args.clone(), trace)?;
        let variable = match &variable.0 {
            Ast::Identifier(identifier) => identifier,
            _ => {
                let err = Err(EvaluateError {
                    location: variable.1.clone(),
                    reason: "expected argument identifier".to_string(),
                });
                trace.push(err);
                return Err(());
            }
        };
        let type_identifier = match &_type.0 {
            Ast::Identifier(identifier) => identifier,
            _ => {
                let err = Err(EvaluateError {
                    location: _type.1.clone(),
                    reason: "expected argument type".to_string(),
                });
                trace.push(err);
                return Err(());
            }
        };
        let param_type =
            resolve_identifier_type(context, (type_identifier.clone(), _type.1.clone()), trace)?;
        context.push((variable.clone(), param_type));
        let return_value = typecheck_with_context(context, body.clone(), trace);
        context.pop();
        let return_value = return_value?;
        trace.push(Ok(return_value));
        let val = (
            Rc::new(LambdaType {
                variable: variable.to_string(),
                body: body.clone(),
            }) as RcType,
            args.1.clone(),
        );
        trace.push(Ok(val.clone()));
        Ok(val.clone())
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
    evaluate_with_context(&mut context, &value)
}

pub fn typecheck(value: (Ast, Span)) -> (TypecheckCall, TypeTrace) {
    let mut trace: TypeTrace = Vec::new();
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
    (
        typecheck_with_context(&mut context, value, &mut trace),
        trace,
    )
}

// TODO FIXME probably return an IdentiferType that also contains the location of the definition
fn resolve_identifier_type(
    context: &mut [(String, (RcType, Span))],
    identifier: (String, Span),
    trace: &mut TypeTrace,
) -> TypecheckCall {
    match context
        .iter()
        .rev()
        .find(|(ident, _)| &identifier.0 == ident)
        .map(|(_ident, value)| value)
    {
        Some(value) => {
            let val = (value.0.clone(), identifier.1);
            trace.push(Ok(val.clone()));
            Ok(val)
        }
        None => {
            trace.push(Err(EvaluateError {
                location: identifier.1,
                reason: format!("could not find identifier {}", identifier.0),
            }));
            Err(())
        }
    }
}

pub fn typecheck_with_context(
    context: &mut Vec<(String, (RcType, Span))>,
    the_type: (Ast, Span),
    trace: &mut TypeTrace,
) -> TypecheckCall {
    match &the_type.0 {
        Ast::Number(number) => {
            let rc = (Rc::new(IntegerType(Some(*number))) as RcType, the_type.1);
            trace.push(Ok(rc.clone()));
            Ok(rc.clone())
        }
        Ast::String(string) => {
            let rc = (
                Rc::new(StringType(Some(string.to_string()))) as RcType,
                the_type.1,
            );
            trace.push(Ok(rc.clone()));
            Ok(rc.clone())
        }
        Ast::Identifier(identifier) => {
            resolve_identifier_type(context, (identifier.to_string(), the_type.1), trace)
        }
        Ast::List(elements) => {
            let (callable, args) = match elements.split_first() {
                Some(v) => v,
                None => {
                    let err = Err(EvaluateError {
                        location: the_type.1,
                        reason: "can't call an empty list".to_string(),
                    });
                    trace.push(err);
                    return Err(());
                }
            };
            let callable = match &callable.0 {
                Ast::Identifier(identifier) => resolve_identifier_type(
                    context,
                    (identifier.clone(), callable.1.clone()),
                    trace,
                )?,
                Ast::List(_) => typecheck_with_context(context, callable.clone(), trace)?,
                _ => {
                    let val = Err(EvaluateError {
                        location: the_type.1,
                        reason: "can't call a string or number".to_string(),
                    });
                    trace.push(val);
                    return Err(());
                }
            };
            let return_value = callable
                .0
                .typecheck_call(context, (args, the_type.1), trace)?;
            trace.push(Ok(return_value.clone()));
            Ok(return_value)
        }
    }
}

#[allow(clippy::all)]
pub fn evaluate_with_context(
    _context: &mut Vec<(String, (RcValue, Span))>,
    _value: &(Ast, Span),
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
