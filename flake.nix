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
          packages.tucant-extension = pkgs.rustPlatform.buildRustPackage {
              pname = "tucant-extension";
              version = "0.5.0";

              src = ./.;

              /*cargoLock = {
                lockFile = ./Cargo.lock;
                 outputHashes = {
                  "web-extensions-sys-0.4.1" = "sha256-BfWWPbITueBwU2lPA2hCjR9w+YTpS4s6fYOmyGdPIro=";
                  "yew-0.21.0" = "sha256-H0pWPhWtpIDsVwl2j0dp2lA9oQwk0145KeDzoXSvjeM=";
                };
              };*/
              cargoLock = {
                lockFile = ./Cargo.lock;
                allowBuiltinFetchGit = true;
              };
             

              # ...
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
            ];
          };
        }
      );
}
