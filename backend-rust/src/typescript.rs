use std::collections::{BTreeSet, VecDeque};

use actix_web::{
    dev::{HttpServiceFactory, ServiceFactory, ServiceRequest},
    web::{Form, Json},
    Error,
};
use chrono::NaiveDateTime;

pub trait Typescriptable {
    fn name() -> String;
    fn code() -> BTreeSet<String> {
        BTreeSet::new()
    }
}

impl Typescriptable for u32 {
    fn name() -> String {
        "number".to_string()
    }
}

impl Typescriptable for i64 {
    fn name() -> String {
        "number".to_string()
    }
}

impl Typescriptable for String {
    fn name() -> String {
        "string".to_string()
    }
}

impl Typescriptable for NaiveDateTime {
    fn name() -> String {
        "string".to_string()
    }
}

impl Typescriptable for () {
    fn name() -> String {
        "null".to_string()
    }
}

impl Typescriptable for bool {
    fn name() -> String {
        "boolean".to_string()
    }
}

impl Typescriptable for i16 {
    fn name() -> String {
        "number".to_string()
    }
}

impl Typescriptable for u8 {
    fn name() -> String {
        "number".to_string()
    }
}

impl Typescriptable for f32 {
    fn name() -> String {
        "number".to_string()
    }
}

impl Typescriptable for i32 {
    fn name() -> String {
        "number".to_string()
    }
}

impl<T1: Typescriptable, T2: Typescriptable> Typescriptable for (T1, T2) {
    fn name() -> String {
        "[".to_string() + &T1::name() + ", " + &T2::name() + "]"
    }
    fn code() -> BTreeSet<String> {
        T1::code() + "\n" + T2::code()
    }
}

impl<T: Typescriptable> Typescriptable for Vec<T> {
    fn name() -> String {
        T::name() + "[]"
    }
    fn code() -> BTreeSet<String> {
        T::code()
    }
}

impl<T: Typescriptable> Typescriptable for VecDeque<T> {
    fn name() -> String {
        T::name() + "[]"
    }
    fn code() -> BTreeSet<String> {
        T::code()
    }
}

impl<T: Typescriptable> Typescriptable for Option<T> {
    fn name() -> String {
        T::name() + " | null"
    }
    fn code() -> BTreeSet<String> {
        T::code()
    }
}

impl<T: Typescriptable, E> Typescriptable for Result<T, E> {
    fn name() -> String {
        T::name()
    }
    fn code() -> BTreeSet<String> {
        T::code()
    }
}

impl<T: Typescriptable> Typescriptable for Json<T> {
    fn name() -> String {
        T::name()
    }
    fn code() -> BTreeSet<String> {
        T::code()
    }
}

impl<T: Typescriptable> Typescriptable for Form<T> {
    fn name() -> String {
        T::name()
    }
    fn code() -> BTreeSet<String> {
        T::code()
    }
}

pub struct TypescriptableApp<T> {
    pub app: actix_web::App<T>,
    pub codes: BTreeSet<String>,
}

impl<T> TypescriptableApp<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: Typescriptable + HttpServiceFactory + 'static,
    {
        self.codes.extend(<F as Typescriptable>::code());
        self.app = self.app.service(factory);
        self
    }
}
