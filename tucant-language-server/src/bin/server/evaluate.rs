use std::rc::Rc;
use std::fmt::Debug;
use crate::parser::{Span, Ast, parse_root};
use anyhow::anyhow;

pub trait Value<'a>: Debug {
    fn evaluate_call(self: Rc<Self>, context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Value<'a> + 'a>> {
        Err(anyhow!("not yet implemented"))
    }
}

pub trait Type<'a>: Debug {
    fn typecheck_call(self: Rc<Self>, context: &mut Vec<(String, Rc<dyn Type<'a> + 'a>)>, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Type<'a> + 'a>> {
        Err(anyhow!("not yet implemented"))
    }
}

#[derive(Debug)]
pub struct IntegerValue(i64);

impl<'a> Value<'a> for IntegerValue {
   
}


#[derive(Debug)]
pub struct IntegerType(Option<i64>);

impl<'a> Type<'a> for IntegerType {
}

#[derive(Debug)]
pub struct StringValue(String);

impl<'a> Value<'a> for StringValue {
}


#[derive(Debug)]
pub struct StringType(Option<String>);

impl<'a> Type<'a> for StringType {
}


#[derive(Debug)]
pub struct Add;

impl<'a> Value<'a> for Add {
    fn evaluate_call(self: Rc<Self>, context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Value<'a> + 'a>> {
        let [left, right]: &[Span<'a, Ast<'a>>; 2] = args.try_into()?;
        let left_value = evaluate_with_context(context, left.clone())?;
        let right_value = evaluate_with_context(context, right.clone())?;
        let return_value = evaluate_with_context(context, self.body.clone());
        return_value
    }

}

#[derive(Debug)]
pub struct Lambda<'a> {
    variable: String,
    body: Span<'a, Ast<'a>>
}

impl<'a> Value<'a> for Lambda<'a> {
    fn evaluate_call(self: Rc<Self>, context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Value<'a> + 'a>> {
        let [variable_value]: &[Span<'a, Ast<'a>>; 1] = args.try_into()?;
        let arg_value = evaluate_with_context(context, variable_value.clone())?;
        context.push((self.variable.clone(), arg_value));
        let return_value = evaluate_with_context(context, self.body.clone());
        context.pop();
        return_value
    }
}

#[derive(Debug)]
pub struct DefineLambdaValue;

impl<'a> Value<'a> for DefineLambdaValue {
    fn evaluate_call(self: Rc<Self>, context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Value<'a> + 'a>> {
        let [variable, body]: &[Span<'a, Ast<'a>>; 2] = args.try_into()?;
        let variable = match variable.inner {
            Ast::Identifier(identifier) => identifier,
            _ => Err(anyhow!("expected argument identifier"))?
        };
        Ok(Rc::new(Lambda::<'_> {
            variable: variable.to_string(),
            body: body.clone(),
        }))
    }
}

#[derive(Debug)]
pub struct DefineLambdaType;

impl<'a> Type<'a> for DefineLambdaType {
    fn typecheck_call(self: Rc<Self>, context: &mut Vec<(String, Rc<dyn Type<'a> + 'a>)>, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Type<'a> + 'a>> {
        let [variable, body]: &[Span<'a, Ast<'a>>; 2] = args.try_into()?;
        let variable = match variable.inner {
            Ast::Identifier(identifier) => identifier,
            _ => Err(anyhow!("expected argument identifier"))?
        };
        Ok(Rc::new(Lambda::<'_> {
            variable: variable.to_string(),
            body: body.clone(),
        }))
    }
}

pub fn evaluate<'a>(value: Span<'a, Ast<'a>>) -> anyhow::Result<Rc<dyn Value<'a> + 'a>> {
    let mut context: Vec<(String, Rc<dyn Value>)> = vec![
        ("lambda".to_string(), Rc::new(DefineLambdaValue))
    ];
    evaluate_with_context(&mut context, value)
}

pub fn typecheck<'a>(value: Span<'a, Ast<'a>>) -> anyhow::Result<Rc<dyn Type<'a> + 'a>> {
    let mut context: Vec<(String, Rc<dyn Type>)> = vec![
        ("lambda".to_string(), Rc::new(DefineLambdaType))
    ];
    typecheck_with_context(&mut context, value)
}

fn resolve_identifier<'a, T>(context: &mut Vec<(String, T)>, identifier: &str) -> anyhow::Result<T> {
    context.iter().rev().find(|(ident, _)| identifier == ident).map(|(ident, value)| value).ok_or(anyhow!("could not find identifier {}", identifier)).cloned()
}

pub fn typecheck_with_context<'a>(context: &mut Vec<(String, Rc<dyn Type<'a> + 'a>)>, _type: Span<'a, Ast<'a>>) -> anyhow::Result<Rc<dyn Type<'a> + 'a>> {
    match _type.inner {
        Ast::Number(number) => Ok(Rc::new(IntegerType(Some(number)))),
        Ast::String(string) => Ok(Rc::new(StringType(Some(string.to_string())))),
        Ast::Identifier(identifier) => resolve_identifier(context, identifier),
        Ast::List(elements) => {
            let (callable, args) = elements.split_first().ok_or(anyhow!("can't call empty list"))?;
            let callable = match callable.inner {
                Ast::Identifier(identifier) => resolve_identifier(context, identifier),
                Ast::List(_) => typecheck_with_context(context, callable.clone()),
                _ => Err(anyhow!("can't call a string or number")),
            };
            callable?.typecheck_call(context, args)
        },
    }
}

pub fn evaluate_with_context<'a>(context: &mut Vec<(String, Rc<dyn Value<'a> + 'a>)>, value: Span<'a, Ast<'a>>) -> anyhow::Result<Rc<dyn Value<'a> + 'a>> {
    match value.inner {
        Ast::Number(number) => Ok(Rc::new(IntegerValue(number))),
        Ast::String(string) => Ok(Rc::new(StringValue(string.to_string()))),
        Ast::Identifier(identifier) => resolve_identifier(context, identifier),
        Ast::List(elements) => {
            let (callable, args) = elements.split_first().ok_or(anyhow!("can't call empty list"))?;
            let callable = match callable.inner {
                Ast::Identifier(identifier) => resolve_identifier(context, identifier),
                Ast::List(_) => evaluate_with_context(context, callable.clone()),
                _ => Err(anyhow!("can't call a string or number")),
            };
            callable?.evaluate_call(context, args)
        },
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

    let result = evaluate(parse_root(Span::new(r#"
        (lambda v v)
    "#)).unwrap().0);
    println!("{:?}", result);

    let result = evaluate(parse_root(Span::new(r#"
        (lambda 1 v)
    "#)).unwrap().0);
    println!("{:?}", result);

    let result = evaluate(parse_root(Span::new(r#"
        ((lambda v v) 1)
    "#)).unwrap().0);
    println!("{:?}", result);

    let result = evaluate(parse_root(Span::new(r#"
        (add 1 (add 1 1))
    "#)).unwrap().0);
    println!("{:?}", result);

    let result = evaluate(parse_root(Span::new(r#"
        (add 1 (add 1 ""))
    "#)).unwrap().0);
    println!("{:?}", result);
}