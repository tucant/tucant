{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:mohe2015/nixpkgs/update-dogtail";

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
          config.allowUnfree = true;
          config.android_sdk.accept_license = true;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchainFor =
          p:
          p.rust-bin.stable.latest.minimal.override {
            targets = [
              "aarch64-unknown-linux-gnu"
              "wasm32-unknown-unknown"
              "x86_64-pc-windows-gnu"
              "aarch64-apple-darwin"
              "x86_64-apple-darwin"
              "x86_64-linux-android"
              "aarch64-linux-android"
            ];
            extensions = [ "rustfmt" ];
          };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchainFor;
        craneLibAarch64Linux = (crane.mkLib pkgs.pkgsCross.aarch64-multiplatform).overrideToolchain rustToolchainFor;
        craneLibAarch64Android = (crane.mkLib pkgs.pkgsCross.aarch64-android-prebuilt).overrideToolchain rustToolchainFor;
        craneLibWindows = (crane.mkLib pkgs.pkgsCross.mingwW64).overrideToolchain rustToolchainFor;

        dioxus-cli = craneLib.buildPackage {
          src = pkgs.fetchFromGitHub {
            owner = "mohe2015";
            repo = "dioxus";
            rev = "10baa108f7f8df5ee1f08e99bdc900dd459c05ae";
            hash = "sha256-A9cod9tevPzH/QcRWOc4IzDSBoFMEgmB3A3dy5QAbzk=";
          };
          doCheck = false;
          strictDeps = true;
          pname = "dioxus-cli";
          cargoExtraArgs = "-p dioxus-cli --features no-downloads --features disable-telemetry";
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          meta = {
            mainProgram = "dx";
          };
        };

        cargoDioxus =
          craneLib:
          {
            profile ? "--release",
            dioxusCommand ? "bundle",
            dioxusExtraArgs ? "",
            dioxusMainArgs ? "",
            cargoExtraArgs ? "",
            notBuildDepsOnly ? { },
            buildDepsOnly ? { },
            dioxusBuildDepsOnlyCommand ? "build",
            ...
          }@origArgs:
          let
            args = {
              pnameSuffix = "-dioxus";
            }
            // (builtins.removeAttrs origArgs [
              "dioxusCommand"
              "dioxusExtraArgs"
              "dioxusMainArgs"
              "cargoExtraArgs"
              "notBuildDepsOnly"
              "buildDepsOnly"
              "dioxusBuildDepsOnlyCommand"
            ]);
          in
          craneLib.mkCargoDerivation (
            {
              buildPhaseCargoCommand = ''
                ls -R
                set -x
                DX_HOME=$(mktemp -d) DIOXUS_LOG=trace,walrus=debug ${dioxus-cli}/bin/dx ${dioxusCommand} --trace ${profile} --base-path public ${dioxusExtraArgs} ${dioxusMainArgs} ${cargoExtraArgs}
                set +x
              '';
              cargoArtifacts = craneLib.buildDepsOnly (
                {
                  # build, don't bundle
                  # TODO make dx home persistent as it's useful
                  buildPhaseCargoCommand = ''
                    set -x
                    DX_HOME=$(mktemp -d) DIOXUS_LOG=trace,walrus=debug ${dioxus-cli}/bin/dx ${dioxusBuildDepsOnlyCommand} --trace ${profile} --base-path public ${dioxusExtraArgs} ${cargoExtraArgs}
                    set +x
                  '';
                  doCheck = false;
                  dummySrc = craneLib.mkDummySrc {
                    src = args.src;
                    extraDummyScript = ''
                      cp ${./crates/tucan-plus-dioxus/Dioxus.toml} $out/crates/tucan-plus-dioxus/Dioxus.toml
                    '';
                  };
                }
                // args
                // buildDepsOnly
              );
            }
            // args
            // notBuildDepsOnly
          );

        fileset-worker = lib.fileset.unions [
          (craneLib.fileset.commonCargoSources ./crates/tucan-plus-worker)
          (craneLib.fileset.commonCargoSources ./crates/tucan-types)
          ./crates/tucan-plus-worker/migrations
          ./.cargo/config.toml
          ./Cargo.toml
          ./Cargo.lock
        ];

        fileset-service-worker = lib.fileset.unions [
          (craneLib.fileset.commonCargoSources ./crates/tucan-plus-service-worker)
        ];

        fileset-dioxus = lib.fileset.unions [
          (craneLib.fileset.commonCargoSources ./crates/tucan-plus-dioxus)
          ./crates/tucan-plus-dioxus/assets/logo.svg
          ./crates/tucan-plus-dioxus/assets/logo.png
          ./crates/tucan-plus-dioxus/assets/manifest.json
          ./crates/tucan-plus-dioxus/assets/bootstrap.css
          ./crates/tucan-plus-dioxus/assets/bootstrap.bundle.min.js
          ./crates/tucan-plus-dioxus/assets/bootstrap.patch.js
          ./crates/tucan-plus-dioxus/index.html
          ./crates/tucan-plus-dioxus/Dioxus.toml
          ./crates/tucan-plus-dioxus/.cargo/config.toml
        ];

        fileset-wasm = lib.fileset.unions [
          ./Cargo.toml
          ./Cargo.lock
          (craneLib.fileset.commonCargoSources ./crates/html-extractor)
          (craneLib.fileset.commonCargoSources ./crates/tucan-connector)
          (craneLib.fileset.commonCargoSources ./crates/html-handler)
          (craneLib.fileset.commonCargoSources ./crates/tucan-plus-planning)
          fileset-dioxus
          fileset-worker
        ];

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

        fileset-native = lib.fileset.unions [
          ./Cargo.toml
          ./Cargo.lock
          (craneLib.fileset.commonCargoSources ./crates/html-extractor)
          (craneLib.fileset.commonCargoSources ./crates/html-handler)
          (craneLib.fileset.commonCargoSources ./crates/tucan-connector)
          (craneLib.fileset.commonCargoSources ./crates/tucan-plus-planning)
          fileset-dioxus
          fileset-worker # TODO rename to database
        ];

        fileset-api = lib.fileset.unions [
          ./Cargo.toml
          ./Cargo.lock
          (craneLib.fileset.commonCargoSources ./crates/html-handler)
          (craneLib.fileset.commonCargoSources ./crates/html-extractor)
          (craneLib.fileset.commonCargoSources ./crates/tucan-connector)
          (craneLib.fileset.commonCargoSources ./crates/tucan-plus-api)
          fileset-worker
        ];

        nativeArgs = {
          pname = "tucan-plus-native";
          cargoExtraArgs = "--package tucan-plus-dioxus";
          preBuild = ''
            cd ./crates/tucan-plus-dioxus
          '';
          postBuild = ''
            cd ../..
          '';
          strictDeps = true;
          src = lib.fileset.toSource {
            root = ./.;
            fileset = fileset-native;
          };
          nativeBuildInputs = [
            pkgs.pkg-config
            (pkgs.writeShellScriptBin "git" ''
              echo ${self.rev or "dirty"}
            '') # TODO we probably need to remove this from the deps derivation.
          ];
          buildInputs = [
            pkgs.sqlite
          ];
        };

        nativeLinuxArgs = nativeArgs // {
          dioxusExtraArgs = "--linux";
          nativeBuildInputs = nativeArgs.nativeBuildInputs ++ [ pkgs.gobject-introspection ];
          buildInputs = nativeArgs.buildInputs ++ [
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
            pkgs.xdotool
            pkgs.zlib
          ];
        };

        nativeLinux = cargoDioxus craneLib (
          nativeLinuxArgs
          // {
            # likely fails in or something https://github.com/tauri-apps/tauri/blob/2e089f6acb854e4d7f8eafb9b2f8242b1c9fa491/crates/tauri-bundler/src/bundle/linux/appimage.rs#L224 because it's before the log message
            dioxusMainArgs = "--out-dir $out --package-types deb --package-types rpm"; # --package-types appimage
            # TODO make this depend on the unbundled derivation?
          }
        );

        nativeLinuxUnbundled = cargoDioxus craneLib (
          nativeLinuxArgs
          // {
            dioxusCommand = "build";
            notBuildDepsOnly = {
              installPhase = ''
                cp -r target/dx/tucan-plus-dioxus/release/linux/app $out
              '';
            };
          }
        );

        # https://v2.tauri.app/distribute/appimage/#appimages-for-arm-based-devices cross compiling not possible
        nativeLinuxAarch64Unbundled = cargoDioxus craneLibAarch64Linux (
          nativeLinuxArgs
          // {
            dioxusCommand = "build"; # TODO we could try building and not bundling?
            dioxusExtraArgs = "--target aarch64-unknown-linux-gnu --linux";
          }
        );

        nativeAndroidArgs = nativeArgs // {
          dioxusExtraArgs = "--android --target aarch64-linux-android";
          # build produces .apk, bundle produces .aab
          dioxusCommand = "build";
          buildInputs = [
            #pkgsCross.aarch64-android-prebuilt.sqlite # should work with my patched nixpkgs
          ];
        };

        /*
          # https://github.com/gradle/gradle/blob/360f9eab2f6f1595025f746a03ee5895659b0b8c/platforms/core-runtime/wrapper-shared/src/main/java/org/gradle/wrapper/PathAssembler.java#L63
          jshell
          java.security.MessageDigest messageDigest = java.security.MessageDigest.getInstance("MD5");
          byte[] bytes = "https://services.gradle.org/distributions/gradle-9.1.0-bin.zip".getBytes("UTF-8");
          messageDigest.update(bytes);
          String result = new BigInteger(1, messageDigest.digest()).toString(36);
        */
        nativeAndroid = cargoDioxus craneLibAarch64Android (
          nativeAndroidArgs
          // rec {
            ANDROID_HOME = "${
              (pkgs.androidenv.composeAndroidPackages {
                includeNDK = true;
                platformVersions = [ "33" ];
                buildToolsVersions = [ "34.0.0" ];
              }).androidsdk
            }/libexec/android-sdk";
            preBuild = ''
              keytool -genkey -v -keystore my-release-key.jks -keyalg RSA -keysize 2048 -validity 10000 -alias my-alias --keypass password --storepass password -dname "CN=Test"
              export GRADLE_USER_HOME=$(mktemp -d)
              mkdir -p $GRADLE_USER_HOME/wrapper/dists/gradle-9.1.0-bin/9agqghryom9wkf8r80qlhnts3/
              cp ${
                pkgs.fetchurl {
                  url = "https://services.gradle.org/distributions/gradle-9.1.0-bin.zip";
                  hash = "sha256-oX3dhaJran9d23H/iwX8UQTAICxuZHgkKXkMkzaGyAY=";
                }
              } $GRADLE_USER_HOME/wrapper/dists/gradle-9.1.0-bin/9agqghryom9wkf8r80qlhnts3/gradle-9.1.0-bin.zip
              ${pkgs.unzip}/bin/unzip $GRADLE_USER_HOME/wrapper/dists/gradle-9.1.0-bin/9agqghryom9wkf8r80qlhnts3/gradle-9.1.0-bin.zip -d $GRADLE_USER_HOME/wrapper/dists/gradle-9.1.0-bin/9agqghryom9wkf8r80qlhnts3/
              touch $GRADLE_USER_HOME/wrapper/dists/gradle-9.1.0-bin/9agqghryom9wkf8r80qlhnts3/gradle-9.1.0-bin.zip.ok
              export GRADLE_OPTS="-Dorg.gradle.project.android.aapt2FromMavenOverride=$ANDROID_HOME/build-tools/34.0.0/aapt2 -Dhttp.proxyHost=$MITM_CACHE_HOST -Dhttp.proxyPort=$MITM_CACHE_PORT -Dhttps.proxyHost=$MITM_CACHE_HOST -Dhttps.proxyPort=$MITM_CACHE_PORT -Djavax.net.ssl.trustStore=$MITM_CACHE_KEYSTORE -Djavax.net.ssl.trustStorePassword=$MITM_CACHE_KS_PWD"
            ''
            + nativeAndroidArgs.preBuild;
            installPhase = ''
              runHook preInstall
              mkdir $out
              cp target/dx/tucan-plus-dioxus/release/android/app/app/build/outputs/apk/release/app-release.apk $out/app-debug.apk
              runHook postInstall
            '';
            nativeBuildInputs = nativeAndroidArgs.nativeBuildInputs ++ [
              pkgs.jdk
              pkgs.gradle_9 # version must match the wrapper version, otherwise you get Failed to assemble apk: Exception in thread "main" java.net.UnknownHostException: services.gradle.org
            ];
            # https://github.com/NixOS/nixpkgs/blob/master/pkgs/development/tools/build-managers/gradle/setup-hook.sh
            gradleUpdateScript = ''
              DX_HOME=$(mktemp -d) ${dioxus-cli}/bin/dx bundle --android --trace --release --base-path public --package tucan-plus-dioxus || true
              cd target/dx/tucan-plus-dioxus/release/android/app/
              patchShebangs ./gradlew
              # the hook overrides gradle user home and stuff so we can't call gradlew
              gradle -Dorg.gradle.project.android.aapt2FromMavenOverride=$ANDROID_HOME/build-tools/34.0.0/aapt2 --info --no-daemon bundleRelease
            '';
            # nix build -L .#nativeAndroid.mitmCache.updateScript && ./result
            mitmCache = pkgs.gradle_9.fetchDeps {
              pkg = nativeAndroid.cargoArtifacts;
              data = ./deps.json;
            };
          }
        );

        # https://github.com/DioxusLabs/dioxus/blob/ad40f816073f91da67c0287a5512a5111e5a1617/packages/cli/src/config/bundle.rs#L263 we could use fixedruntime but it would be better if the others would also allow specifying a path
        # https://github.com/DioxusLabs/dioxus/issues/4281
        # https://github.com/DioxusLabs/dioxus/issues/4599
        # https://github.com/DioxusLabs/dioxus/issues/4076 probably not possible yet
        nativeWindowsArgs = nativeArgs // {
          dioxusCommand = "build"; # TODO we could try building and not bundling?
          cargoDioxusExtraArgs = "--target x86_64-pc-windows-gnu --windows"; # TODO FIXME dioxus should auto-detect the target from the env variable
          cargoExtraArgs = "--package tucan-plus-dioxus";
          nativeBuildInputs = nativeArgs.nativeBuildInputs ++ [ pkgs.pkg-config ];
          buildInputs = nativeArgs.buildInputs ++ [
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
            pkgs.xdotool
            pkgs.zlib
          ];
        };

        nativeWindows = cargoDioxus craneLibWindows (
          nativeWindowsArgs
          // {

          }
        );

        api-server = craneLib.buildPackage {
          strictDeps = true;
          buildInputs = [
            pkgs.sqlite
          ];
          pname = "tucan-plus-workspace-native-api";
          src = lib.fileset.toSource {
            root = ./.;
            fileset = fileset-api;
          };
          cargoTestExtraArgs = "--no-run";
          cargoExtraArgs = "--package=tucan-plus-api";
        };

        schema =
          pkgs.runCommandNoCC "schema.json"
            {
            }
            ''
              ${api-server}/bin/schema > $out
            '';

        wasm-bindgen = (
          pkgs.buildWasmBindgenCli rec {
            src = pkgs.fetchCrate {
              pname = "wasm-bindgen-cli";
              version = "0.2.104";
              hash = "sha256-9kW+a7IreBcZ3dlUdsXjTKnclVW1C1TocYfY8gUgewE=";
            };

            cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
              inherit src;
              inherit (src) pname version;
              hash = "sha256-V0AV5jkve37a5B/UvJ9B3kwOW72vWblST8Zxs8oDctE=";
            };
          }
        );

        service-worker-args = {
          strictDeps = true;
          doCheck = false;
          cargoExtraArgs = "--package=tucan-plus-service-worker";
          pname = "tucan-plus-workspace-tucan-plus-service-worker";
          buildPhaseCargoCommand = ''
            cargo build --target wasm32-unknown-unknown
            wasm-bindgen target/wasm32-unknown-unknown/debug/tucan-plus-service-worker.wasm --target no-modules --out-dir ./target/dx/tucan-plus-service-worker/debug/web/public/wasm/ --no-typescript
            echo "wasm_bindgen.initSync({ module: Uint8Array.fromBase64(\"$(base64 -w0 target/dx/tucan-plus-service-worker/debug/web/public/wasm/tucan-plus-service-worker_bg.wasm)\")})" >> ./target/dx/tucan-plus-service-worker/debug/web/public/wasm/tucan-plus-service-worker.js
          '';
          installPhaseCommand = ''
            mkdir $out
            cp ./crates/tucan-plus-service-worker/target/dx/tucan-plus-service-worker/debug/web/public/wasm/tucan-plus-service-worker.js $out/tucan-plus-service-worker.js
          '';
          checkPhaseCargoCommand = '''';
          nativeBuildInputs = [
            pkgs.which
            wasm-bindgen
            pkgs.binaryen
            (pkgs.writeShellScriptBin "git" ''
              echo ${self.rev or "dirty"}
            '')
          ];
          doNotPostBuildInstallCargoBinaries = true;
          src = lib.fileset.toSource {
            root = ./.;
            fileset = fileset-service-worker;
          };
        };

        service-worker = craneLib.buildPackage (
          service-worker-args
          // {
            cargoArtifacts = craneLib.buildDepsOnly (
              service-worker-args
              // {
                dummySrc = craneLib.mkDummySrc {
                  src = service-worker-args.src;
                  extraDummyScript = ''
                    rm $out/crates/tucan-plus-service-worker/src/main.rs
                    cp ${pkgs.writeText "main.rs" ''
                      use wasm_bindgen::prelude::*;

                      #[wasm_bindgen(main)]
                      pub async fn main() {

                      }
                    ''} $out/crates/tucan-plus-service-worker/src/main.rs
                  '';
                };
              }
            );
          }
        );

        client-args = {
          dioxusExtraArgs = "--features direct --web";
          dioxusMainArgs = "--out-dir $out --wasm-split";
          buildDepsOnly = {
            preBuild = ''
              export CC=emcc
              export CXX=emcc
            '';
            dummySrc = craneLib.mkDummySrc {
              src = client-args.src;
              extraDummyScript = ''
                rm $out/crates/tucan-plus-dioxus/src/main.rs
                cp ${pkgs.writeText "main.rs" ''
                  use wasm_bindgen::prelude::*;

                  #[wasm_bindgen(main)]
                  pub async fn main() {

                  }
                ''} $out/crates/tucan-plus-dioxus/src/main.rs
              '';
            };
          };
          notBuildDepsOnly = {
            preBuild = ''
              export CC=emcc
              export CXX=emcc
              mkdir -p assets/
              rm -R ./target/dx/tucan-plus-dioxus/release/web/public/assets || true
            '';
            # temporary https://github.com/DioxusLabs/dioxus/issues/4758
            postBuild = ''
              substituteInPlace $out/public/assets/tucan-plus-dioxus-*.js --replace-fail "importMeta.url" "import.meta.url"
            '';
          };
          
          strictDeps = true;
          stdenv = p: p.emscriptenStdenv;
          doCheck = false;
          src = lib.fileset.toSource {
            root = ./.;
            fileset = fileset-wasm;
          };
          cargoExtraArgs = "--package=tucan-plus-dioxus";
          pname = "tucan-plus-workspace-tucan-plus-dioxus";
          installPhaseCommand = '''';
          checkPhaseCargoCommand = '''';
          nativeBuildInputs = [
            pkgs.which
            pkgs.emscripten
            wasm-bindgen
            pkgs.binaryen
            (pkgs.writeShellScriptBin "git" ''
              echo ${self.rev or "dirty"}
            '')
          ];
          doNotPostBuildInstallCargoBinaries = true;
        };

        client = cargoDioxus craneLib (client-args);

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
            fileset-worker
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
      rec {
        formatter = pkgs.nixfmt-tree;
        checks = {
          #inherit api schema client;

          # todo also clippy the frontend
          my-app-clippy = craneLib.cargoClippy ({
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            src = source-with-build-instructions;
          });

          my-app-fmt = craneLib.cargoFmt ({
            cargoExtraArgs = "--all";
            src = source-with-build-instructions;
          });

          # https://nixos.org/manual/nixos/unstable/index.html#sec-nixos-tests
          # https://github.com/NixOS/nixpkgs/blob/a25a80403e18d80ffb9e5a2047c7936e57fbae68/nixos/tests/installed-tests/default.nix#L15
          # https://github.com/NixOS/nixpkgs/blob/a25a80403e18d80ffb9e5a2047c7936e57fbae68/nixos/tests/installed-tests/gnome-photos.nix#L10
          # nix run -L .#checks.x86_64-linux.extension-test.driverInteractive
          # test_script()
          # nix flake check -L

          extension-test = pkgs.testers.runNixOSTest {
            name = "extension-test";
            nodes = {
              machine =
                { pkgs, ... }:
                {
                  virtualisation = {
                    sharedDirectories = {
                      projects = {
                        source = "/home/moritz/Documents/tucan-plus/demo-video/";
                        target = "/home/test/tucan_plus";
                      };
                    };
                  };
                  virtualisation.memorySize = 8192;

                  services.gnome.at-spi2-core.enable = true;

                  services.xserver.xkb.layout = "de";

                  boot.kernelPackages = pkgs.linuxPackages_latest;

                  services.displayManager.gdm.enable = true;
                  services.desktopManager.gnome.enable = true;

                  #services.gnome.core-apps.enable = false;
                  #services.gnome.core-developer-tools.enable = false;
                  #services.gnome.games.enable = false;
                  environment.gnome.excludePackages = with pkgs; [ gnome-tour ]; # gnome-user-docs

                  services.displayManager.autoLogin.enable = true;
                  services.displayManager.autoLogin.user = "test";

                  users.users.test = {
                    isNormalUser = true;
                    uid = 1000;
                  };

                  environment.systemPackages = [
                    pkgs.firefox
                    # TODO https://nixos.org/manual/nixpkgs/unstable/#ssec-gnome-common-issues-double-wrapped
                    (pkgs.python3.pkgs.buildPythonApplication {
                      pname = "run-test";
                      version = "3.32.2";
                      pyproject = true;
                      build-system = with pkgs.python3Packages; [ setuptools ];

                      dependencies = with pkgs.python3Packages; [
                        dogtail
                      ];

                      src = ./demo-video;

                      nativeBuildInputs = [
                        pkgs.wrapGAppsHook3
                        pkgs.gobject-introspection
                      ];

                      dontWrapGApps = true;

                      # Arguments to be passed to `makeWrapper`, only used by buildPython*
                      preFixup = ''
                        makeWrapperArgs+=("''${gappsWrapperArgs[@]}")
                      '';
                    })
                    (pkgs.writeShellScriptBin "ponytail" ''
                      ${pkgs.gnome-ponytail-daemon}/libexec/gnome-ponytail-daemon
                    '')
                  ];

                  programs.dconf.enable = true;
                  programs.dconf.profiles.test.databases = [
                    {
                      settings = {
                        "org/gnome/desktop/interface" = {
                          toolkit-accessibility = true;
                        };
                      };
                    }
                  ];

                  systemd.user.services = {
                    "org.gnome.Shell@wayland" = {
                      serviceConfig = {
                        ExecStart = [
                          # Clear the list before overriding it.
                          ""
                          # Eval API is now internal so Shell needs to run in unsafe mode.
                          # TODO: improve test driver so that it supports openqa-like manipulation
                          # that would allow us to drop this mess.
                          "${pkgs.gnome-shell}/bin/gnome-shell --unsafe-mode"
                        ];
                      };
                    };
                  };

                  system.stateVersion = "25.11";
                };
            };
            testScript =
              { nodes, ... }:
              lib.mkForce ''
                print("a")
                start_all()
                print("b")
                machine.wait_until_succeeds(
                    "systemd-run --pipe --machine=test@.host --user /usr/bin/env bash -c 'gdbus call --session -d org.gnome.Shell -o /org/gnome/Shell -m org.gnome.Shell.Eval Main.layoutManager._startingUp' | grep -q \"true,..false\"",
                    timeout=60
                )
                print("c")
                machine.succeed("systemd-run --machine=test@.host --user /usr/bin/env bash -c 'gsettings set org.gnome.desktop.interface toolkit-accessibility true'")
                print("d")
                machine.succeed("systemd-run --machine=test@.host --user /usr/bin/env bash -c firefox")
                print("e")
                machine.succeed("systemd-run --machine=test@.host --user /usr/bin/env bash -c ponytail")
                print("f")
                machine.succeed("systemd-run --pipe --machine=test@.host --user /usr/bin/env bash -c tucan_plus")
                print("g")
              '';
            interactive = {
              sshBackdoor.enable = true; # ssh vsock/3 -o User=root
            };
            # https://wiki.nixos.org/wiki/Python
            # ssh vsock/3 -o User=root
            # machinectl shell test@
            # nix-shell -I nixpkgs=channel:nixos-unstable -p gobject-introspection gtk3 'python3.withPackages (ps: with ps; [ dogtail ])' --run python /home/test/tucan_plus/tucan_plus.py
          };

        };
        packages.schema = schema;
        packages.service-worker = service-worker;
        packages.client = client;
        packages.api-server = api-server;

        packages.extension = extension;
        packages.extension-unpacked = extension-unpacked;
        packages.extension-source = source;
        packages.extension-source-unpacked = source-unpacked;

        packages.dioxus-cli = dioxus-cli;
        packages.nativeLinux = nativeLinux;
        packages.nativeLinuxUnbundled = nativeLinuxUnbundled;
        packages.nativeAndroid = nativeAndroid;

        packages.nativeLinuxAarch64Unbundled = nativeLinuxAarch64Unbundled; # cross compiling appimage not possible

        # maybe dioxus downloads stuff here
        # https://github.com/tauri-apps/tauri/blob/2e089f6acb854e4d7f8eafb9b2f8242b1c9fa491/crates/tauri-bundler/src/bundle/windows/util.rs#L45
        packages.nativeWindows = nativeWindows; # cross building is broken for dioxus

        apps.api-server = flake-utils.lib.mkApp {
          name = "api-server";
          drv = api-server;
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
            export PATH=~/.cargo/bin/:$PATH
            export CC_wasm32_unknown_emscripten=emcc
            #export SERVICE_WORKER_JS_PATH=/assets/wasm/tucan-plus-service-worker.js
          '';
          buildInputs = [
            pkgs.openssl
            pkgs.sqlite
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
            pkgs.xdotool
            pkgs.zlib
          ];
          packages = [
            pkgs.bashInteractive
            pkgs.wabt
            pkgs.wasm-tools
            pkgs.nodejs
            pkgs.bun
            pkgs.pkg-config
            pkgs.emscripten
            pkgs.gobject-introspection
            pkgs.jdk
            pkgs.android-tools
            pkgs.binaryen
            pkgs.llvmPackages_21.bintools
            dioxus-cli
          ];
        };
      }
    );
}
