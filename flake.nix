{
  description = "my project description";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          devShells.default = pkgs.mkShell {
            nativeBuildInputs =
              let
                rust =
#                  fenix.packages.${system}.stable;
                   fenix.packages.${system}.toolchainOf { channel = "stable"; sha256 = "sha256-eMJethw5ZLrJHmoN2/l0bIyQjoTX1NsvalWSscTixpI="; };
              in
              with pkgs; [
                bashInteractive
                nixpkgs-fmt
                rust.toolchain
                rust.rust-analyzer
                llvmPackages_latest.clang
                llvmPackages_latest.bintools
                nodejs_latest
                pkg-config
                openssl.dev
              ];
            buildInputs = with pkgs; [
              postgresql_15
            ];
            RUST_BACKTRACE = 1;
            # export PATH=$PATH:/home/moritz/Documents/rome/target/debug/
          };
        }
      );
}
