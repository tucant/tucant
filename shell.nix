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
  ];
  RUST_BACKTRACE = 1;
}
