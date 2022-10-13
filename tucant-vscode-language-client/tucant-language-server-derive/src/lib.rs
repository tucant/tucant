use syn::{Error, parse_macro_input, parse::Nothing};
use tucant_language_server_derive_internal::handle_magic;

// cargo expand --test meta_model
// cargo doc --document-private-items
#[proc_macro]
pub fn magic(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(item as Nothing);
    proc_macro::TokenStream::from(handle_magic().unwrap_or_else(Error::into_compile_error))
}
