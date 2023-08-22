// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
//#![deny(unused_results)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions,
    clippy::unused_async
)]

extern crate self as tucant;

pub mod models;

pub mod schema;
pub mod tucan;
pub mod tucan_user;
pub mod url;

#[derive(Debug)]
pub struct MyError {
    err: anyhow::Error,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}

impl<E: Into<anyhow::Error>> From<E> for MyError {
    fn from(err: E) -> Self {
        Self { err: err.into() }
    }
}

#[cfg(feature = "axum")]
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
#[cfg(feature = "axum")]
impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        println!("{:?}", self.err);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.err)).into_response()
    }
}
