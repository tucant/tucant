{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    crate2nix.url = "github:nix-community/crate2nix";
  };

  outputs = inputs @ { self, nixpkgs, flake-utils, crate2nix }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ ];
          };
          lib = pkgs.lib;
          cargoNix = inputs.crate2nix.tools.${system}.appliedCargoNix {
            name = "tucant-extension";
            src = ./.;
          };
        in
        {
          packages.tucant-extension-incremental = cargoNix.workspaceMembers.tucant-yew.build.override { features = ["direct"]; };

          packages.tucant-extension = pkgs.clangStdenv.mkDerivation rec {
            pname = "tucant-extension.zip";
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
              (pkgs.wasm-bindgen-cli.override { version = "0.2.99"; hash = "sha256-1AN2E9t/lZhbXdVznhTcniy+7ZzlaEp/gwLEAucs6EA="; cargoHash = "sha256-DbwAh8RJtW38LJp+J9Ht8fAROK9OabaJ85D9C/Vkve4="; })
            ];

            buildPhase = ''
              cd tucant-yew
              ${pkgs.trunk}/bin/trunk build --skip-version-check --offline --features direct --dist ../tucant-extension/dist --public-url /dist
              cd ..
            '';

            installPhase = ''
              cd tucant-extension
              ${pkgs.zip}/bin/zip $out -r * -x "node_modules/*" icon-512.png jsconfig.json package-lock.json package.json run.sh README.md screenshot-large.png
              cd ..
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
