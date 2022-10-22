use std::rc::Rc;

use crate::parser::{Span, Ast};

pub trait Object {
    fn call(&self, args: &[Span<Ast>]) -> Rc<dyn Object> {
        unimplemented!()
    }
}

pub struct IntegerType(i64);

impl Object for IntegerType {

}

pub struct StringType(String);

impl Object for StringType {
}

pub struct Lambda;

impl Object for Lambda {
    fn call(&self, args: &[Span<Ast>]) -> Rc<dyn Object> {
        todo!()
    }
}

pub fn evaluate(value: Span<Ast>) -> Rc<dyn Object> {
    let context: Vec<(String, Rc<dyn Object>)> = vec![
        ("lambda".to_string(), Rc::new(Lambda))
    ];
    evaluate_with_context(context, value)
}

pub fn evaluate_with_context(context: Vec<(String, Rc<dyn Object>)>, value: Span<Ast>) -> Rc<dyn Object> {
    match value.inner {
        Ast::Number(number) => Rc::new(IntegerType(number)),
        Ast::String(string) => Rc::new(StringType(string.to_string())),
        Ast::Identifier(identifier) => context.iter().rev().find(|(ident, _)| identifier == ident).unwrap().1.clone(),
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