#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]

use std::{env, path::Path};

// this is done using this special source macro so source information is preserved and working when clicking on generated structs in your IDE.
#[proc_macro]
pub fn magic_include(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lsp.rs");
    let path = dest_path.to_string_lossy();
    quote::quote! {
        #[path = #path]
        pub mod lsp;
        pub use lsp::*;
    }
    .into()
}
