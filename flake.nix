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
            shellHook = ''
              export PATH=$PATH:~/.cargo/bin
            '';

            buildInputs = [ pkgs.openssl ];

            nativeBuildInputs = [ pkgs.bashInteractive pkgs.pkg-config pkgs.nodejs pkgs.bacon ];
          };
        }
      );
}
