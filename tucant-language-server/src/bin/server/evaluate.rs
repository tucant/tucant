use std::rc::Rc;
use std::fmt::Debug;
use crate::parser::{Span, Ast, parse_root};
use anyhow::anyhow;

pub trait Object<'a>: Debug {
    fn call(&self, context: &mut Vec<(String, Rc<dyn Object<'a> + 'a>)>, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Object<'a> + 'a>> {
        Err(anyhow!("not yet implemented"))
    }
}

#[derive(Debug)]
pub struct IntegerType(i64);

impl<'a> Object<'a> for IntegerType {

}

#[derive(Debug)]
pub struct StringType(String);

impl<'a> Object<'a> for StringType {
}

#[derive(Debug)]
pub struct Lambda<'a> {
    variable: String,
    body: Span<'a, Ast<'a>>
}

impl<'a> Object<'a> for Lambda<'a> {
    fn call(&self, context: &mut Vec<(String, Rc<dyn Object<'a> + 'a>)>, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Object<'a> + 'a>> {
        let [variable_value]: &[Span<'a, Ast<'a>>; 1] = args.try_into()?;
        let arg_value = evaluate_with_context(context, variable_value.clone())?;
        context.push((self.variable.clone(), arg_value));
        let return_value = evaluate_with_context(context, self.body.clone());
        context.pop();
        return_value
    }
}

#[derive(Debug)]
pub struct DefineLambda;

impl<'a> Object<'a> for DefineLambda {
    fn call(&self, context: &mut Vec<(String, Rc<dyn Object<'a> + 'a>)>, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Object<'a> + 'a>> {
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

pub fn evaluate<'a>(value: Span<'a, Ast<'a>>) -> anyhow::Result<Rc<dyn Object<'a> + 'a>> {
    let mut context: Vec<(String, Rc<dyn Object>)> = vec![
        ("lambda".to_string(), Rc::new(DefineLambda))
    ];
    evaluate_with_context(&mut context, value)
}

fn resolve_identifier<'a>(context: &mut Vec<(String, Rc<dyn Object<'a> + 'a>)>, identifier: &str) -> anyhow::Result<Rc<dyn Object<'a> + 'a>> {
    context.iter().rev().find(|(ident, _)| identifier == ident).map(|(ident, value)| value).ok_or(anyhow!("could not find identifier {}", identifier)).cloned()
}

pub fn evaluate_with_context<'a>(context: &mut Vec<(String, Rc<dyn Object<'a> + 'a>)>, value: Span<'a, Ast<'a>>) -> anyhow::Result<Rc<dyn Object<'a> + 'a>> {
    match value.inner {
        Ast::Number(number) => Ok(Rc::new(IntegerType(number))),
        Ast::String(string) => Ok(Rc::new(StringType(string.to_string()))),
        Ast::Identifier(identifier) => resolve_identifier(context, identifier),
        Ast::List(elements) => {
            let (callable, args) = elements.split_first().ok_or(anyhow!("can't call empty list"))?;
            let callable = match callable.inner {
                Ast::Identifier(identifier) => resolve_identifier(context, identifier),
                Ast::List(_) => evaluate_with_context(context, callable.clone()),
                _ => Err(anyhow!("can't call a string or number")),
            };
            callable?.call(context, args)
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