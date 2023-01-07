#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]

//include!(concat!(env!("OUT_DIR"), "/lsp.rs"));

// https://github.com/rust-lang/rust/issues/48250
// https://github.com/rust-lang/rfcs/issues/1516

// https://doc.rust-lang.org/stable/core/macro.stringify.html

tucant_language_server_derive::magic_include!();
//tucant_language_server_derive::magic!();
