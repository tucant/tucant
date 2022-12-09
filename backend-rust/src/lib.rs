// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
//#![deny(unused_results)]

use std::fmt::Display;

use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
extern crate self as tucant;

pub mod models;
#[cfg(feature = "server")]
pub mod schema;
#[cfg(feature = "server")]
pub mod tucan;
#[cfg(feature = "server")]
pub mod tucan_user;
#[cfg(feature = "server")]
pub mod typescript;
#[cfg(feature = "server")]
pub mod url;

#[derive(Debug)]
pub struct MyError {
    err: anyhow::Error,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}

impl<E: Into<anyhow::Error>> From<E> for MyError {
    fn from(err: E) -> MyError {
        MyError { err: err.into() }
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.err)).into_response()
    }
}
