use std::env;
use std::fs;
use std::path::Path;
use tucant_language_server_derive_internal::handle_magic;

// we really need this, otherwise you can't see SendableAndForget::R and other stuff
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lsp.rs");
    let output = handle_magic().unwrap();
    let output = syn::parse2::<syn::File>(output).unwrap_or_else(|err| {
        panic!(
            "{} {err:#?} {:?}",
            dest_path.to_string_lossy().as_ref(),
            err.span().start()
        )
    });
    let output = prettyplease::unparse(&output);
    fs::write(dest_path, output).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
