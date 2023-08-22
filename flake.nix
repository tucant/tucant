{
  # ln -sf /home/moritz/Documents/rome/target/debug/rome /home/moritz/Documents/tucant/tucant_react/node_modules/@rometools/cli-linux-x64/rome
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
                  fenix.packages.${system}.complete;
                  #fenix.packages.${system}.toolchainOf { channel = "stable"; sha256 = "sha256-gdYqng0y9iHYzYPAdkC/ka3DRny3La/S5G8ASj0Ayyc="; };
              in
              with pkgs; [
                bashInteractive
                nixpkgs-fmt
                (fenix.packages.${system}.combine [
                  rust.toolchain
                  rust.rust-analyzer
                  fenix.packages.${system}.targets.wasm32-unknown-unknown.latest.toolchain
                  fenix.packages.${system}.targets.wasm32-unknown-emscripten.latest.toolchain
                  fenix.packages.${system}.targets.wasm32-wasi.latest.toolchain
                ])
                llvmPackages_latest.clang
                llvmPackages_latest.bintools
                llvmPackages_latest.llvm
                llvmPackages_latest.libclang
                nodejs_latest
                pkg-config
                openssl.dev
                emscripten
                yarn
              ];
            buildInputs = with pkgs; [
              postgresql_15
              sqlite
              mysql
            ];
            RUST_BACKTRACE = 1;

            hardeningDisable = [ "fortify" ];

            #             export PATH=$PATH:$HOME/Documents/rome/target/debug/
            shellHook = ''
              export LIBCLANG_PATH="${pkgs.llvmPackages_latest.libclang}/lib";
            '';



          };

        }
      );
}
