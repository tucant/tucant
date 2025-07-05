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

        rustToolchainFor = p: p.rust-bin.stable.latest.minimal.override {
          targets = [ "wasm32-unknown-unknown" ];
          extensions = [ "rustfmt" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchainFor;

        commonArgs = {
          strictDeps = true;
        };

        dioxus-cli = craneLib.buildPackage {
          src = pkgs.fetchFromGitHub {
            owner = "mohe2015";
            repo = "dioxus";
            rev = "8711b6af3e98780a2be2871d6bc50866b8813f07";
            hash = "sha256-IVjauXjiOYBiD2xqja8p5ZAJfeB4s6VA0zikucDrGb8=";
          };
          doCheck = false;
          strictDeps = true;
          pname = "dioxus-cli";
          cargoExtraArgs = "-p dioxus-cli --features no-downloads";
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };

        nativeArgs = commonArgs // {
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources ./crates)
              ./Cargo.toml
              ./Cargo.lock
              (lib.fileset.fileFilter
                (file: lib.any file.hasExt [ "html" "scss" ])
                ./.
              )
              ./tucant-extension/bootstrap.bundle.min.js
              ./tucant-extension/bootstrap.css
              ./rustfmt.toml
            ];
          };
          pname = "tucant-workspace-native";
        };

        tests = craneLib.buildPackage (commonArgs // {
          pname = "tucant-workspace-native-tests";
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (craneLib.fileset.commonCargoSources ./crates/tucant-tests)
            ];
          };
          cargoTestExtraArgs = "--no-run";
          cargoExtraArgs = "--package=tucant-tests";
        });

        api = craneLib.buildPackage (commonArgs // {
          pname = "tucant-workspace-native-api";
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (craneLib.fileset.commonCargoSources ./crates/tucant-types)
              (craneLib.fileset.commonCargoSources ./crates/key-value-database)
              (craneLib.fileset.commonCargoSources ./crates/html-extractor)
              (craneLib.fileset.commonCargoSources ./crates/tucan-connector)
              (craneLib.fileset.commonCargoSources ./crates/tucant-api)
              (craneLib.fileset.commonCargoSources ./crates/html-handler)
            ];
          };
          cargoTestExtraArgs = "--no-run";
          cargoExtraArgs = "--package=tucant-api";
        });

        schema = pkgs.runCommandNoCC "schema.json" {
          } ''
            ${api}/bin/schema > $out
          '';

        fileset-wasm = lib.fileset.unions [
          ./Cargo.toml
          ./Cargo.lock
          (craneLib.fileset.commonCargoSources ./crates/tucant-types)
          (craneLib.fileset.commonCargoSources ./crates/key-value-database)
          (craneLib.fileset.commonCargoSources ./crates/html-extractor)
          (craneLib.fileset.commonCargoSources ./crates/tucan-connector)
          (craneLib.fileset.commonCargoSources ./crates/tucant-dioxus)
          (craneLib.fileset.commonCargoSources ./crates/html-handler)
          ./crates/tucant-dioxus/assets/bootstrap.css
          ./crates/tucant-dioxus/assets/bootstrap.bundle.min.js
        ];

        client = craneLib.buildPackage (commonArgs // {
          doCheck = false;
          cargoArtifacts = null; # building deps only does not work with the default stub entrypoint
          src = lib.fileset.toSource {
            root = ./.;
            fileset = fileset-wasm;
          };
          cargoExtraArgs = "--package=tucant-dioxus";
          pname = "tucant-workspace-tucant-dioxus";
          preBuild = ''
            cd ./crates/tucant-dioxus
          '';
          buildPhaseCargoCommand = ''
            export HOME=$(mktemp -d)
            ${dioxus-cli}/bin/dx bundle --platform web --verbose --release --out-dir $out --base-path public --features direct
          '';
          installPhaseCommand = ''
          '';
          nativeBuildInputs = [ pkgs.wasm-bindgen-cli_0_2_100 pkgs.binaryen ];
          doNotPostBuildInstallCargoBinaries = true;
        });

        fileset-extension = lib.fileset.unions [
          ./tucant-extension/background.js
          ./tucant-extension/fix-session-id-in-url.js
          ./tucant-extension/context-menu.js
          ./tucant-extension/content-script.js
          ./tucant-extension/content-script-redirect.js
          ./tucant-extension/open-in-tucan.js
          ./tucant-extension/bootstrap.bundle.min.js
          ./tucant-extension/bootstrap.css
          ./tucant-extension/icon.png
          ./tucant-extension/manifest.json
          ./tucant-extension/mobile.css
          ./tucant-extension/mobile.js
          ./tucant-extension/options.html
          ./tucant-extension/options.js
          ./tucant-extension/popup.html
          ./tucant-extension/popup.js
          ./tucant-extension/utils.js
          ./tucant-extension/rules.json
          ./tucant-extension/screenshot.png
        ];

        extension-unpacked = pkgs.stdenv.mkDerivation {
          pname = "tucant-extension";
          version = (lib.importJSON ./tucant-extension/manifest.json).version;

          src = lib.fileset.toSource {
            root = ./tucant-extension;
            fileset = fileset-extension;
          };

          installPhase = ''
            mkdir $out
            cp -r $src/. $out/
            cp -r ${client}/public/. $out/public/
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
      in
      {
        checks = {
          inherit api schema client;

          # todo also clippy the frontend
          my-app-clippy = craneLib.cargoClippy (nativeArgs // {
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          my-app-fmt = craneLib.cargoFmt (nativeArgs // {
            src = source-with-build-instructions;
          });
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
            cargo test -- --nocapture
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
            cargo test -- --nocapture
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
      });
}
