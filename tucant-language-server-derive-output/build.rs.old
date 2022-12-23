use std::env;
use std::fs;
use std::path::Path;
use tucant_language_server_derive_internal::handle_magic;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lsp.rs");
    let output = handle_magic().unwrap();
    let output = syn::parse2::<syn::File>(output).unwrap();
    let output = prettyplease::unparse(&output);
    fs::write(dest_path, output).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
