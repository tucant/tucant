use std::{collections::BTreeMap, convert::identity};

use proc_macro2::Span;
use syn::{
    punctuated::Punctuated,
    token::{Brace, Comma},
    LitStr,
};

use crate::json_parser::{JSONValue, KeyValue};

#[derive(Eq, PartialEq)]
pub struct LitStrOrd(pub LitStr);

impl PartialOrd for LitStrOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.value().partial_cmp(&other.0.value())
    }
}

impl Ord for LitStrOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.value().cmp(&other.0.value())
    }
}

pub fn extract_keys<const N: usize>(
    (brace, value): (Brace, Punctuated<KeyValue<JSONValue>, Comma>),
    keys: [&str; N],
) -> Result<[JSONValue; N], syn::Error> {
    let mut map: BTreeMap<_, _> = value
        .into_iter()
        .map(|e| (LitStrOrd(e.key), e.value))
        .collect();
    let result = keys.map(|key| {
        let corresponding_value = map.remove(&LitStrOrd(LitStr::new(key, Span::call_site())));
        corresponding_value
            .ok_or_else(|| syn::Error::new(brace.span, format!("Could not find key {key}")))
    });
    if result.iter().any(Result::is_err) || !map.is_empty() {
        let unexpected_keys = map.into_iter().map(|key| {
            return syn::Error::new(
                key.0 .0.span(),
                format!("Unexpected key {}", key.0 .0.token()),
            );
        });

        let results = unexpected_keys
            .chain(result.into_iter().filter_map(Result::err))
            .reduce(|mut e1, e2| {
                e1.combine(e2);
                e1
            })
            .unwrap();
        return Err(results);
    }
    result.try_map(identity)
}

pub fn unexpected_keys<T>(
    map: BTreeMap<LitStrOrd, JSONValue>,
    value: Result<T, syn::Error>,
) -> Result<T, syn::Error> {
    if value.is_err() || !map.is_empty() {
        let unexpected_keys = map.into_iter().map(|key| {
            return syn::Error::new(
                key.0 .0.span(),
                format!("Unexpected key {}", key.0 .0.token()),
            );
        });

        Err(std::iter::once(value)
            .filter_map(Result::err)
            .chain(unexpected_keys)
            .reduce(|mut e1, e2| {
                e1.combine(e2);
                e1
            })
            .unwrap())
    } else {
        value
    }
}
