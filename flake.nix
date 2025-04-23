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

  outputs = inputs@{ self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustNightlyToolchainFor = p: p.rust-bin.nightly."2025-04-02".minimal.override {
          extensions = [ "rust-docs" "clippy" "rust-src" "rustc-dev" "llvm-tools-preview" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        craneNightlyLib = (crane.mkLib pkgs).overrideToolchain rustNightlyToolchainFor;

        commonArgs = {
          strictDeps = true;
        };

        nativeArgs = commonArgs // {
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              (craneNightlyLib.fileset.commonCargoSources ./crates)
              ./Cargo.toml
              ./Cargo.lock
              (lib.fileset.fileFilter
                (file: lib.any file.hasExt [ "html" "scss" ])
                ./.
              )
              ./tucant-extension/bootstrap.bundle.min.js
              ./tucant-extension/bootstrap.min.css
              ./crates/tucant-yew/fixup.sh
              ./rustfmt.toml
            ];
          };
          pname = "tucant-workspace-native";
        };

        cargoArtifacts = craneNightlyLib.buildDepsOnly nativeArgs;

        tests = craneNightlyLib.buildPackage (commonArgs // {
          pname = "tucant-workspace-native-tests";
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (craneNightlyLib.fileset.commonCargoSources ./crates/tucant-tests)
            ];
          };
          cargoTestExtraArgs = "--no-run";
          cargoExtraArgs = "--package=tucant-tests";
          inherit cargoArtifacts;
        });

        api = craneNightlyLib.buildPackage (commonArgs // {
          pname = "tucant-workspace-native-api";
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (craneNightlyLib.fileset.commonCargoSources ./crates/tucant-types)
              (craneNightlyLib.fileset.commonCargoSources ./crates/key-value-database)
              (craneNightlyLib.fileset.commonCargoSources ./crates/html-extractor)
              (craneNightlyLib.fileset.commonCargoSources ./crates/tucan-connector)
              (craneNightlyLib.fileset.commonCargoSources ./crates/tucant-api)
              (craneNightlyLib.fileset.commonCargoSources ./crates/html-handler)
            ];
          };
          cargoTestExtraArgs = "--no-run";
          cargoExtraArgs = "--package=tucant-api";
          inherit cargoArtifacts;
        });

        tucant-extension-typescript = pkgs.buildNpmPackage rec {
          pname = "tucant-extension-typescript";
          version = "0.1.0";

          src = lib.fileset.toSource {
            root = ./tucant-extension;
            fileset = fileset-extension;
          };

          npmDepsHash = "sha256-EqI/SfntW2DYQaIpjm5noPdht3MSIeC3mhuV6PyW8xo=";
        };

        tucant-extension-typescript-minimal = pkgs.buildNpmPackage rec {
          pname = "tucant-extension-typescript-minimal";
          version = "0.1.0";

          src = lib.fileset.toSource {
            root = ./tucant-extension;
            fileset = lib.fileset.unions [
              ./tucant-extension/helper.ts
              ./tucant-extension/open-in-tucan.ts
              ./tucant-extension/tsconfig.json
              ./tucant-extension/package.json
              ./tucant-extension/package-lock.json
            ];
          };

          npmDepsHash = "sha256-EqI/SfntW2DYQaIpjm5noPdht3MSIeC3mhuV6PyW8xo=";
        };

        schema = pkgs.runCommandNoCC "schema.json" {
          } ''
            ${api}/bin/schema > $out
          '';

        fileset-wasm = lib.fileset.unions [
          ./Cargo.toml
          ./Cargo.lock
          (craneNightlyLib.fileset.commonCargoSources ./crates/tucant-types)
          (craneNightlyLib.fileset.commonCargoSources ./crates/key-value-database)
          (craneNightlyLib.fileset.commonCargoSources ./crates/html-extractor)
          (craneNightlyLib.fileset.commonCargoSources ./crates/tucan-connector)
          (craneNightlyLib.fileset.commonCargoSources ./crates/tucant-yew)
          (craneNightlyLib.fileset.commonCargoSources ./crates/html-handler)
          ./crates/tucant-yew/index.html
          ./tucant-extension/bootstrap.bundle.min.js
          ./tucant-extension/bootstrap.min.css
          ./crates/tucant-yew/fixup.sh
        ];

        wasmArgs = commonArgs // {
          src = lib.fileset.toSource {
            root = ./.;
            fileset = fileset-wasm;
          };
          pname = "tucant-workspace-wasm";
          cargoExtraArgs = "--package=tucant-yew";
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        };

        cargoArtifactsWasm = craneNightlyLib.buildDepsOnly (wasmArgs // {
          doCheck = false;
        });

        client = craneNightlyLib.buildTrunkPackage (wasmArgs // {
          trunkExtraBuildArgs = "--features direct --public-url /dist";
          pname = "tucant-workspace-tucant-yew";
          cargoArtifacts = cargoArtifactsWasm;
          preBuild = ''
            cp -r ${tucant-extension-typescript-minimal}/lib/node_modules/tucant-extension/dist/. ./tucant-extension/dist/
            cd ./crates/tucant-yew
          '';
          postBuild = ''
            mv ./dist ..
            cd ..
          '';
          wasm-bindgen-cli = pkgs.wasm-bindgen-cli_0_2_100;
        });

        fileset-extension = lib.fileset.unions [
          ./tucant-extension/background.ts
          ./tucant-extension/fix-session-id-in-url.ts
          ./tucant-extension/content-script.ts
          ./tucant-extension/content-script-redirect.ts
          ./tucant-extension/open-in-tucan.ts
          ./tucant-extension/mobile.ts
          ./tucant-extension/options.ts
          ./tucant-extension/popup.ts
          ./tucant-extension/helper.ts
          ./tucant-extension/screenshot.png
          ./tucant-extension/tsconfig.json
          ./tucant-extension/package.json
          ./tucant-extension/package-lock.json
        ];

        fileset-extension-raw = lib.fileset.unions [
          ./tucant-extension/bootstrap.bundle.min.js
          ./tucant-extension/bootstrap.min.css
          ./tucant-extension/icon.png
          ./tucant-extension/manifest.json
          ./tucant-extension/mobile.css
          ./tucant-extension/options.html
          ./tucant-extension/popup.html
          ./tucant-extension/rules.json
        ];

        extension-unpacked = pkgs.stdenv.mkDerivation {
          pname = "tucant-extension";
          version = (lib.importJSON ./tucant-extension/manifest.json).version;

          src = lib.fileset.toSource {
            root = ./tucant-extension;
            fileset = fileset-extension-raw;
          };

          installPhase = ''
            mkdir -p $out/dist
            cp -r ${tucant-extension-typescript}/lib/node_modules/tucant-extension/dist/. $out/dist/
            cp -r $src/. $out/
            cp -r ${client}/. $out/dist/
          '';
        };

        extension = pkgs.runCommand "tucant-extension.zip" { } ''
          cd ${extension-unpacked}
          ${pkgs.zip}/bin/zip -r $out *
          ${pkgs.strip-nondeterminism}/bin/strip-nondeterminism --type zip $out
        '';

        source-with-build-instructions = lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            fileset-wasm
            fileset-extension
            fileset-extension-raw
            ./flake.nix
            ./flake.lock
            ./Dockerfile
            ./README.md
            ./rustfmt.toml
          ];
        };

        source = pkgs.runCommand "tucant-extension-source.zip" { } ''
          cd ${source-with-build-instructions}
          ${pkgs.zip}/bin/zip -r $out *
          ${pkgs.strip-nondeterminism}/bin/strip-nondeterminism --type zip $out
        '';

        source-unpacked = pkgs.runCommand "tucant-extension-source.zip" { } ''
          cp -r ${source-with-build-instructions} $out
        '';

        rustfmt = craneNightlyLib.buildPackage {
          pname = "rustfmt";
          doNotRemoveReferencesToRustToolchain = true;
          src = pkgs.fetchFromGitHub {
            owner = "tucant";
            repo = "rustfmt";
            rev = "a385d03d387388bbd645a498fa946c9ff203e4dd";
            hash = "sha256-Zii39Um2aeiV0lnVrDTzDplRnJqJdcKFljFuI1ur/as=";
          };
          doCheck = false;
        };
      in
      {
        checks = {
          inherit api schema client;

          # todo also clippy the frontend
          my-app-clippy = craneNightlyLib.cargoClippy (nativeArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          my-app-fmt = craneNightlyLib.cargoFmt.override { rustfmt = rustfmt; } (nativeArgs // {
            RUSTFMT = "${rustfmt}/bin/rustfmt";
            src = source-with-build-instructions;
          });
        };

        packages.tucant-extension-typescript = tucant-extension-typescript;
        packages.schema = schema;
        packages.client = client;
        packages.server = api;
        packages.tests = tests;
        packages.extension = extension;
        packages.extension-unpacked = extension-unpacked;
        packages.extension-source = source;
        packages.extension-source-unpacked = source-unpacked;
        packages.rustfmt = rustfmt;

        apps.server = flake-utils.lib.mkApp {
          name = "server";
          drv = api;
        };

        packages.publish =
          let
            version = (lib.importJSON ./tucant-extension/manifest.json).version;
          in
          pkgs.writeShellScriptBin "publish"
            ''
              set -ex
              mkdir -p out
              cd out
              # seems like chromium writes into the parent folder of the pack-extension argument
              chmod -R ug+rw tucant-extension-${version} || true
              rm -Rf tucant-extension-${version}
              cp -r ${extension-unpacked} tucant-extension-${version}
              ${pkgs.chromium}/bin/chromium --no-sandbox --pack-extension=tucant-extension-${version} --pack-extension-key=$CHROMIUM_EXTENSION_SIGNING_KEY
              chmod 644 tucant-extension-${version}.crx

              chmod -R ug+rw tucant-extension-${version}
              rm -Rf tucant-extension-${version}
              cp -r ${extension-unpacked} tucant-extension-${version}
              chmod -R ug+rw tucant-extension-${version}

              ${pkgs.web-ext}/bin/web-ext sign --channel unlisted --source-dir tucant-extension-${version} --upload-source-code ${source}
              chmod 644 web-ext-artifacts/tucant-${version}.xpi
              cp web-ext-artifacts/tucant-${version}.xpi tucant-extension-${version}.xpi
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
            ${tests}/bin/chromium-extension
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
            cargo nextest run
          '';
        };

        devShells.default = pkgs.mkShell {

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.zlib}/lib''${LD_LIBRARY_PATH:+:}''${LD_LIBRARY_PATH}"
          '';

          packages = [
            pkgs.trunk
            rustfmt
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
      });
}
