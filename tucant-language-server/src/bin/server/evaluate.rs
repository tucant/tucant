use std::rc::Rc;
use std::fmt::Debug;
use crate::parser::{Span, Ast, parse_root};
use anyhow::anyhow;

pub trait Object: Debug {
    fn call<'a>(&self, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Object + 'a>> {
        Err(anyhow!("not yet implemented"))
    }
}

#[derive(Debug)]
pub struct IntegerType(i64);

impl Object for IntegerType {

}

#[derive(Debug)]
pub struct StringType(String);

impl Object for StringType {
}

#[derive(Debug)]
pub struct Lambda<'a> {
    variable: String,
    body: Span<'a, Ast<'a>>
}

impl<'a> Object for Lambda<'a> {
    
}

#[derive(Debug)]
pub struct DefineLambda;

impl Object for DefineLambda {
    fn call<'a>(&self, args: &[Span<'a, Ast<'a>>]) -> anyhow::Result<Rc<dyn Object + 'a>> {
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

pub fn evaluate<'a>(value: Span<'a, Ast<'a>>) -> anyhow::Result<Rc<dyn Object + 'a>> {
    let context: Vec<(String, Rc<dyn Object>)> = vec![
        ("lambda".to_string(), Rc::new(DefineLambda))
    ];
    evaluate_with_context(context, value)
}

fn resolve_identifier<'a>(context: Vec<(String, Rc<dyn Object + 'a>)>, identifier: &str) -> anyhow::Result<Rc<dyn Object + 'a>> {
    context.iter().rev().find(|(ident, _)| identifier == ident).map(|(ident, value)| value).ok_or(anyhow!("could not find identifier {}", identifier)).cloned()
}

pub fn evaluate_with_context<'a>(context: Vec<(String, Rc<dyn Object + 'a>)>, value: Span<'a, Ast<'a>>) -> anyhow::Result<Rc<dyn Object + 'a>> {
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
            callable?.call(args)
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
}