// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
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

