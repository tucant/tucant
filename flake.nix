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
            nativeBuildInputs = with pkgs; [
              bashInteractive
              fenix.packages.${system}.complete.toolchain
              fenix.packages.${system}.rust-analyzer
              llvmPackages_latest.clang
              llvmPackages_latest.lld
              postgresql_15
              nodejs_latest
              pkg-config
              openssl.dev
            ];
            RUST_BACKTRACE = 1;
            # export PATH=$PATH:/home/moritz/Documents/rome/target/debug/
          };
        }
      );
}