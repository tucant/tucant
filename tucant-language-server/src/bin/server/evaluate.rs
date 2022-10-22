use std::rc::Rc;
use std::fmt::Debug;
use crate::parser::{Span, Ast};
use anyhow::anyhow;

pub trait Object: Debug {
    fn call(&self, args: &[Span<Ast>]) -> anyhow::Result<Rc<dyn Object>> {
        unimplemented!()
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
pub struct DefineLambda;

impl Object for DefineLambda {
    fn call(&self, args: &[Span<Ast>]) -> anyhow::Result<Rc<dyn Object>> {
        todo!()
    }
}

pub fn evaluate(value: Span<Ast>) -> anyhow::Result<Rc<dyn Object>> {
    let context: Vec<(String, Rc<dyn Object>)> = vec![
        ("lambda".to_string(), Rc::new(DefineLambda))
    ];
    evaluate_with_context(context, value)
}

pub fn evaluate_with_context(context: Vec<(String, Rc<dyn Object>)>, value: Span<Ast>) -> anyhow::Result<Rc<dyn Object>> {
    match value.inner {
        Ast::Number(number) => Ok(Rc::new(IntegerType(number))),
        Ast::String(string) => Ok(Rc::new(StringType(string.to_string()))),
        Ast::Identifier(identifier) => context.iter().rev().find(|(ident, _)| identifier == ident).map(|(ident, value)| value).ok_or(anyhow!("could not find identifier {}", identifier)).cloned(),
        Ast::List(elements) => {
            let (callable, args) = elements.split_first().unwrap();
            let callable = match callable.inner {
                Ast::Identifier(identifier) => context.iter().rev().find(|(ident, _)| identifier == ident).unwrap().1.clone(),
                _ => panic!(),
            };
            callable.call(args)
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
}