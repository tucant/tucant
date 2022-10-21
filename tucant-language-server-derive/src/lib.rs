use std::{env, path::Path};

use syn::{parse::Nothing, parse_macro_input, Error};
use tucant_language_server_derive_internal::handle_magic;

// cargo expand --test meta_model
// cargo doc --document-private-items
#[proc_macro]
pub fn magic(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(item as Nothing);
    proc_macro::TokenStream::from(handle_magic().unwrap_or_else(Error::into_compile_error))
}

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
