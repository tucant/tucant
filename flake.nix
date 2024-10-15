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
          devShells.default = pkgs.mkShell {
            buildInputs = [ pkgs.bashInteractive pkgs.openssl pkgs.wasm-pack ];

            nativeBuildInputs = [ pkgs.pkg-config ];
          };
        }
      );
}
