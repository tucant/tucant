{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    crate2nix.url = "github:nix-community/crate2nix";

    naersk.url = "github:nix-community/naersk";
    fenix.url = "github:nix-community/fenix";

    rust-overlay.url = "github:oxalica/rust-overlay/stable";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    cargo2nix.url = "github:mohe2015/cargo2nix/";
    cargo2nix.inputs.nixpkgs.follows = "nixpkgs";
    cargo2nix.inputs.flake-utils.follows = "flake-utils";
    cargo2nix.inputs.rust-overlay.follows = "rust-overlay";
  };

  outputs = inputs @ { self, nixpkgs, flake-utils, crate2nix, cargo2nix, rust-overlay, naersk, fenix }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [cargo2nix.overlays.default];
          };
          crossPkgs = import nixpkgs {
            inherit system;
            crossSystem = nixpkgs.lib.systems.examples.wasm32-unknown-none;
            overlays = [cargo2nix.overlays.default];
          };
          lib = pkgs.lib;
          cargoNix = inputs.crate2nix.tools.${system}.appliedCargoNix {
            name = "tucant-extension";
            src = ./.;
          };
          rustPkgs = crossPkgs.rustBuilder.makePackageSet {
            rustVersion = "1.83.0";
            packageFun = import ./Cargo.nix;
            target = "wasm32-unknown-unknown";
          };
          toolchain = with fenix.packages.${system};
            combine [
              minimal.rustc
              minimal.cargo
              targets.x86_64-unknown-linux-musl.latest.rust-std
              targets.x86_64-pc-windows-gnu.latest.rust-std
              targets.i686-pc-windows-gnu.latest.rust-std
            ];

          naersk' = naersk.lib.${system}.override {
            cargo = toolchain;
            rustc = toolchain;
          };

          naerskBuildPackage = target: args:
            naersk'.buildPackage (
              args
                // { CARGO_BUILD_TARGET = target; }
            );
        in
        {
          packages.tucant-extension-two-drvs = naerskBuildPackage "x86_64-pc-windows-gnu" {
            src = ./.;
            doCheck = true;
            strictDeps = true;

            depsBuildBuild = with pkgs; [
            ];

            nativeBuildInputs = with pkgs; [
            ];
          };

          # nix run github:mohe2015/cargo2nix/24ebb6c
          #packages.tucant-extension-incremental = (rustPkgs.workspace.tucant-yew {});
          #packages.tucant-extension-incremental = cargoNix.workspaceMembers.tucant-yew.build.override { features = ["direct"]; };

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
