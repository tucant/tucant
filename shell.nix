{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  nativeBuildInputs = [
    bashInteractive
    cargo
    cargo.rustc.llvmPackages.clang
    cargo.rustc.llvmPackages.bintools
    rust-analyzer
    clippy
    rustfmt
  ];
  RUST_BACKTRACE = 1;
}
