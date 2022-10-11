#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tucant_language_server_derive::magic;
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
