FROM docker.io/nixos/nix:2.25.3

COPY . /workdir

RUN nix build /workdir#tucant-extension