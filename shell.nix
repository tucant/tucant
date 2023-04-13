{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  nativeBuildInputs = [
    bashInteractive
    cargo
    cargo.rustc
    cargo.rustc.llvmPackages.clang
    cargo.rustc.llvmPackages.bintools
    rust-analyzer
    clippy
    rustfmt
    postgresql_15
    nodejs
    pkg-config
    openssl.dev
  ];
  RUST_BACKTRACE = 1;
  # export PATH=$PATH:/home/moritz/Documents/rome/target/debug/
}
