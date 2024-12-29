{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ ];
          };
          lib = pkgs.lib;
        in
        {
          packages.tucant-extension = pkgs.clangStdenv.mkDerivation rec {
            pname = "tucant-extension";
            version = "0.5.0";

            src = ./.;

            cargoDeps = pkgs.rustPlatform.importCargoLock {
              lockFile = ./Cargo.lock;
              allowBuiltinFetchGit = true;
            };

            nativeBuildInputs = [
              pkgs.rustPlatform.cargoSetupHook
              pkgs.rustc
              pkgs.cargo
              pkgs.llvmPackages_19.bintools
              pkgs.wasm-bindgen-cli
            ];

            buildPhase = ''
              cd tucant-yew
              ${pkgs.trunk}/bin/trunk build --skip-version-check --offline --features direct --dist ../tucant-extension/dist --public-url /dist
            '';
          };

          devShells.default = pkgs.mkShell {
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];

            shellHook = ''
              export PATH=$PATH:~/.cargo/bin
            '';

            buildInputs = [
              pkgs.at-spi2-atk
              pkgs.atkmm
              pkgs.cairo
              pkgs.gdk-pixbuf
              pkgs.glib
              pkgs.gtk3
              pkgs.harfbuzz
              pkgs.librsvg
              pkgs.libsoup_3
              pkgs.pango
              pkgs.webkitgtk_4_1
              pkgs.openssl
            ];

            nativeBuildInputs = [
              pkgs.bashInteractive
              pkgs.pkg-config
              pkgs.bacon
              pkgs.sqlitebrowser
              pkgs.gobject-introspection
              #pkgs.cargo
              #pkgs.cargo-tauri
              pkgs.nodejs
              pkgs.android-tools
              pkgs.lsb-release
              pkgs.openjdk
              pkgs.nixpkgs-fmt
            ];
          };
        }
      );
}
