use std::env;
use std::fs;
use std::path::Path;

use tucant_language_server_derive_internal::handle_magic;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lsp.rs");
    let file_contents = handle_magic().unwrap().to_string();
    fs::write(
        &dest_path,
        file_contents
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}