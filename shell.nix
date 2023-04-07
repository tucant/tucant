{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  nativeBuildInputs = [
    bashInteractive
    cargo
    cargo.rustc.llvmPackages.clang
    cargo.rustc.llvmPackages.lld
  ];

  shellHook = ''
    # ...
  '';
}
