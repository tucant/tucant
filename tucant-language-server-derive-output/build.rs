use std::env;
use std::fs;
use std::path::Path;
use tucant_language_server_derive_internal::handle_magic;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lsp.rs");
    let token_stream = handle_magic().unwrap();
    //let file = syn::parse2::<syn::File>(token_stream).unwrap();
    //let formatted = prettyplease::unparse(&file);
    fs::write(
        &dest_path,
        token_stream.to_string()
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}