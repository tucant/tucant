{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "git+file:///home/moritz/Documents/nixpkgs";

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

        cargoArtifactsCommon = craneLib.buildDepsOnly {
          stdenv = p: p.emscriptenStdenv;
          preBuild = ''
            env | grep CC
            env | grep WASM
            env | grep wasm
            exit 1
          '';
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
          pname = "tucan-plus";
        };
        cargoArtifacts = cargoArtifactsCommon.overrideAttrs { stdenv = p: p.emscriptenStdenv; };
        cargoArtifactsWasm = cargoArtifactsCommon.overrideAttrs { CARGO_BUILD_TARGET = "wasm32-unknown-unknown"; };
        # packages.x86_64-linux.experiment.stdenv.cc.passthru 
        experiment = pkgs.pkgsCross.emscripten.stdenv.mkDerivation {
          pname = "test";
          version = "0.1.0";
        };
      in
      rec {
        packages.experiment = experiment;
      }
    );
}
