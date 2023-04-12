#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]

use axum::Json;
use chrono::NaiveDateTime;
use std::collections::{BTreeSet, VecDeque};
pub trait Typescriptable {
    fn name() -> String;
    #[must_use]
    fn code() -> BTreeSet<String> {
        BTreeSet::new()
    }
}

pub trait TypescriptRoute {
    fn code(path: &str) -> BTreeSet<String>;
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

impl Typescriptable for f64 {
    fn name() -> String {
        "number".to_string()
    }
}

impl Typescriptable for i32 {
    fn name() -> String {
        "number".to_string()
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

impl<T1: Typescriptable, T2: Typescriptable> Typescriptable for (T1, T2) {
    fn name() -> String {
        "[".to_string() + &T1::name() + ", " + &T2::name() + "]"
    }
    fn code() -> BTreeSet<String> {
        let mut val = T1::code();
        val.extend(T2::code());
        val
    }
}

impl<T1: Typescriptable, T2: Typescriptable, T3: Typescriptable> Typescriptable for (T1, T2, T3) {
    fn name() -> String {
        "[".to_string() + &T1::name() + ", " + &T2::name() + ", " + &T3::name() + "]"
    }
    fn code() -> BTreeSet<String> {
        let mut val = T1::code();
        val.extend(T2::code());
        val.extend(T3::code());
        val
    }
}

impl<T1: Typescriptable, T2: Typescriptable, T3: Typescriptable, T4: Typescriptable> Typescriptable
    for (T1, T2, T3, T4)
{
    fn name() -> String {
        "[".to_string()
            + &T1::name()
            + ", "
            + &T2::name()
            + ", "
            + &T3::name()
            + ", "
            + &T4::name()
            + "]"
    }
    fn code() -> BTreeSet<String> {
        let mut val = T1::code();
        val.extend(T2::code());
        val.extend(T3::code());
        val.extend(T4::code());
        val
    }
}

impl<
        T1: Typescriptable,
        T2: Typescriptable,
        T3: Typescriptable,
        T4: Typescriptable,
        T5: Typescriptable,
    > Typescriptable for (T1, T2, T3, T4, T5)
{
    fn name() -> String {
        "[".to_string()
            + &T1::name()
            + ", "
            + &T2::name()
            + ", "
            + &T3::name()
            + ", "
            + &T4::name()
            + ", "
            + &T5::name()
            + "]"
    }
    fn code() -> BTreeSet<String> {
        let mut val = T1::code();
        val.extend(T2::code());
        val.extend(T3::code());
        val.extend(T4::code());
        val.extend(T5::code());
        val
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
