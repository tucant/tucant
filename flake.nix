{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchainFor =
          p:
          p.rust-bin.nightly.latest.minimal.override {
            targets = [ "wasm32-unknown-unknown" ];
            extensions = [ "rustfmt" ];
          };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchainFor;

        dioxus-cli = craneLib.buildPackage {
          src = pkgs.fetchFromGitHub {
            owner = "mohe2015";
            repo = "dioxus";
            rev = "c5ca0c8576825ea2b6478ac3087d2f0d778f9ff6";
            hash = "sha256-Uy5Ug3bu8Reae6i8UpT2/tV0yjAfubO44oFs0EYN8Rc=";
          };
          doCheck = false;
          strictDeps = true;
          pname = "dioxus-cli";
          cargoExtraArgs = "-p dioxus-cli --features no-downloads";
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };

        nativeArgs = {
          strictDeps = true;
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources ./crates)
              (lib.fileset.fileFilter (
                file:
                lib.any file.hasExt [
                  "html"
                  "scss"
                ]
              ) ./.)
              ./tucan-plus-extension/bootstrap.bundle.min.js
              ./tucan-plus-extension/bootstrap.css
            ];
          };
          pname = "tucan-plus-workspace-native";
        };

        tests = craneLib.buildPackage {
          cargoToml = ./crates/tucan-plus-tests/Cargo.toml;
          cargoLock = ./crates/tucan-plus-tests/Cargo.lock;
          preBuild = ''
            cd ./crates/tucan-plus-tests
          '';
          strictDeps = true;
          pname = "tucan-plus-workspace-native-tests";
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources ./crates/tucan-plus-tests)
            ];
          };
          cargoTestExtraArgs = "--no-run";
          cargoExtraArgs = "--package=tucan-plus-tests";
        };

        api = craneLib.buildPackage {
          cargoToml = ./crates/tucan-plus-api/Cargo.toml;
          cargoLock = ./crates/tucan-plus-api/Cargo.lock;
          preBuild = ''
            cd ./crates/tucan-plus-api
          '';
          strictDeps = true;
          pname = "tucan-plus-workspace-native-api";
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources ./crates/tucan-types)
              (craneLib.fileset.commonCargoSources ./crates/key-value-database)
              (craneLib.fileset.commonCargoSources ./crates/html-extractor)
              (craneLib.fileset.commonCargoSources ./crates/tucan-connector)
              (craneLib.fileset.commonCargoSources ./crates/tucan-plus-api)
              (craneLib.fileset.commonCargoSources ./crates/html-handler)
            ];
          };
          cargoTestExtraArgs = "--no-run";
          cargoExtraArgs = "--package=tucan-plus-api";
        };

        schema =
          pkgs.runCommandNoCC "schema.json"
            {
            }
            ''
              ${api}/bin/schema > $out
            '';

        fileset-wasm = lib.fileset.unions [
          (craneLib.fileset.commonCargoSources ./crates/tucan-types)
          (craneLib.fileset.commonCargoSources ./crates/key-value-database)
          (craneLib.fileset.commonCargoSources ./crates/html-extractor)
          (craneLib.fileset.commonCargoSources ./crates/tucan-connector)
          (craneLib.fileset.commonCargoSources ./crates/tucan-plus-dioxus)
          (craneLib.fileset.commonCargoSources ./crates/html-handler)
          (craneLib.fileset.commonCargoSources ./crates/tucan-plus-planning)
          ./crates/tucan-plus-dioxus/migrations
          ./crates/tucan-plus-dioxus/assets/logo.svg
          ./crates/tucan-plus-dioxus/assets/manifest.json
          ./crates/tucan-plus-dioxus/assets/bootstrap.css
          ./crates/tucan-plus-dioxus/assets/bootstrap.bundle.min.js
          ./crates/tucan-plus-dioxus/assets/bootstrap.patch.js
          ./crates/tucan-plus-dioxus/index.html
        ];

        client = craneLib.buildPackage {
          cargoToml = ./crates/tucan-plus-dioxus/Cargo.toml;
          cargoLock = ./crates/tucan-plus-dioxus/Cargo.lock;
          preBuild = ''
            cd ./crates/tucan-plus-dioxus
          '';
          strictDeps = true;
          stdenv = p: p.emscriptenStdenv;
          doCheck = false;
          cargoArtifacts = null; # building deps only does not work with the default stub entrypoint
          src = lib.fileset.toSource {
            root = ./.;
            fileset = fileset-wasm;
          };
          cargoExtraArgs = "--package=tucan-plus-dioxus";
          pname = "tucan-plus-workspace-tucan-plus-dioxus";
          buildPhaseCargoCommand = ''
            export HOME=$(mktemp -d)
            #export EMCC_DEBUG=1
            export CC=emcc
            export CXX=emcc
            emcc --version
            ${dioxus-cli}/bin/dx bundle --platform web --verbose --release --out-dir $out --base-path public --features direct
          '';
          installPhaseCommand = '''';
          checkPhaseCargoCommand = '''';
          nativeBuildInputs = [
            pkgs.which
            pkgs.emscripten
            (pkgs.buildWasmBindgenCli rec {
              src = pkgs.fetchCrate {
                pname = "wasm-bindgen-cli";
                version = "0.2.103";
                hash = "sha256-ZMK/MpThET2b2uO+9gt9orjXbqLH5ZaoOQ9CAUU9PZY=";
              };

              cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
                inherit src;
                inherit (src) pname version;
                hash = "sha256-J+F9SqTpH3T0MbvlNKVyKnMachgn8UXeoTF0Pk3Xtnc=";
              };
            })
            pkgs.binaryen
            (pkgs.writeShellScriptBin "git" ''
              echo ${self.rev or "dirty"}
            '')
          ];
          doNotPostBuildInstallCargoBinaries = true;
        };

        fileset-extension = lib.fileset.unions [
          ./tucan-plus-extension/background.js
          ./tucan-plus-extension/fix-session-id-in-url.js
          ./tucan-plus-extension/context-menu.js
          ./tucan-plus-extension/content-script.js
          ./tucan-plus-extension/content-script-redirect.js
          ./tucan-plus-extension/open-in-tucan.js
          ./tucan-plus-extension/bootstrap.bundle.min.js
          ./tucan-plus-extension/bootstrap.css
          ./tucan-plus-extension/manifest.json
          ./tucan-plus-extension/mobile.css
          ./tucan-plus-extension/mobile.js
          ./tucan-plus-extension/options.html
          ./tucan-plus-extension/options.js
          ./tucan-plus-extension/popup.html
          ./tucan-plus-extension/popup.js
          ./tucan-plus-extension/custom-ui.js
          ./tucan-plus-extension/recover-tabs.js
          ./tucan-plus-extension/url-mappings.js
          ./tucan-plus-extension/utils.js
          ./tucan-plus-extension/rules.json
          ./tucan-plus-extension/logo.png
        ];

        extension-unpacked = pkgs.stdenv.mkDerivation {
          pname = "tucan-plus-extension";
          version = (lib.importJSON ./tucan-plus-extension/manifest.json).version;

          src = lib.fileset.toSource {
            root = ./tucan-plus-extension;
            fileset = fileset-extension;
          };

          installPhase = ''
            mkdir $out
            cp -r $src/. $out/
            cp -r ${client}/public/. $out/public/
          '';
        };

        extension = pkgs.runCommand "tucan-plus-extension.zip" { } ''
          cd ${extension-unpacked}
          ${pkgs.zip}/bin/zip -r $out *
          ${pkgs.strip-nondeterminism}/bin/strip-nondeterminism --type zip $out
        '';

        source-with-build-instructions = lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            fileset-wasm
            fileset-extension
            ./flake.nix
            ./flake.lock
            ./Dockerfile
            ./README.md
            ./rustfmt.toml
          ];
        };

        source = pkgs.runCommand "tucan-plus-extension-source.zip" { } ''
          cd ${source-with-build-instructions}
          ${pkgs.zip}/bin/zip -r $out *
          ${pkgs.strip-nondeterminism}/bin/strip-nondeterminism --type zip $out
        '';

        source-unpacked = pkgs.runCommand "tucan-plus-extension-source.zip" { } ''
          cp -r ${source-with-build-instructions} $out
        '';
      in
      {
        formatter = pkgs.nixfmt-tree;
        checks = {
          inherit api schema client;

          # todo also clippy the frontend
          my-app-clippy = craneLib.cargoClippy (
            nativeArgs
            // {
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          my-app-fmt = craneLib.cargoFmt (
            nativeArgs
            // {
              cargoToml = ./crates/tucan-plus-dioxus/Cargo.toml;
              cargoLock = ./crates/tucan-plus-dioxus/Cargo.lock;
              preBuild = ''
                cd ./crates/tucan-plus-dioxus
              '';
              cargoExtraArgs = "--all";
              src = source-with-build-instructions;
            }
          );
        };

        packages.schema = schema;
        packages.client = client;
        packages.server = api;
        packages.tests = tests;
        packages.extension = extension;
        packages.extension-unpacked = extension-unpacked;
        packages.extension-source = source;
        packages.extension-source-unpacked = source-unpacked;
        packages.dioxus-cli = dioxus-cli;

        apps.server = flake-utils.lib.mkApp {
          name = "server";
          drv = api;
        };

        packages.publish =
          let
            version = (lib.importJSON ./tucan-plus-extension/manifest.json).version;
          in
          pkgs.writeShellScriptBin "publish" ''
            set -ex
            mkdir -p out
            cd out
            # seems like chromium writes into the parent folder of the pack-extension argument
            chmod -R ug+rw tucan-plus-extension-${version} || true
            rm -Rf tucan-plus-extension-${version}
            cp -r ${extension-unpacked} tucan-plus-extension-${version}
            ${pkgs.chromium}/bin/chromium --no-sandbox --pack-extension=tucan-plus-extension-${version} --pack-extension-key=$CHROMIUM_EXTENSION_SIGNING_KEY
            chmod 644 tucan-plus-extension-${version}.crx

            chmod -R ug+rw tucan-plus-extension-${version}
            rm -Rf tucan-plus-extension-${version}
            cp -r ${extension-unpacked} tucan-plus-extension-${version}
            chmod -R ug+rw tucan-plus-extension-${version}

            ${pkgs.web-ext}/bin/web-ext sign --channel unlisted --source-dir tucan-plus-extension-${version} --upload-source-code ${source}
            chmod 644 web-ext-artifacts/tucan_plus-${version}.xpi
            cp web-ext-artifacts/tucan_plus-${version}.xpi tucan-plus-extension-${version}.xpi
          '';

        packages.test = pkgs.writeShellApplication {
          name = "test";

          runtimeInputs = [
            pkgs.chromedriver
            pkgs.geckodriver
            pkgs.chromium
            pkgs.firefox
          ];

          text = ''
            set -ex
            EXTENSION_DIR=$(mktemp -d)
            export EXTENSION_DIR
            cp -r ${extension-unpacked}/. "$EXTENSION_DIR"/
            chmod -R ug+rw "$EXTENSION_DIR"
            cargo test --package tucan-plus-tests -- --nocapture
          '';
        };

        packages.test-dev = pkgs.writeShellApplication {
          name = "test-dev";

          text = ''
            set -ex
            EXTENSION_DIR=$(mktemp -d)
            export EXTENSION_DIR
            cp -r ${extension-unpacked}/. "$EXTENSION_DIR"/
            chmod -R ug+rw "$EXTENSION_DIR"
            cargo test --package tucan-plus-tests -- --nocapture
          '';
        };

        devShells.default = pkgs.mkShell {

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.zlib}/lib''${LD_LIBRARY_PATH:+:}''${LD_LIBRARY_PATH}"
          '';

          packages = [
            pkgs.bashInteractive
            pkgs.diffoscope
            pkgs.bacon
            pkgs.geckodriver
            pkgs.chromedriver
            pkgs.chromium
            pkgs.firefox
            pkgs.nodejs_latest
          ];
        };
      }
    );
}
