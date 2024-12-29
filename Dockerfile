# podman build --output . .
# or
# sudo docker build --output . .
FROM docker.io/nixos/nix:2.25.3 AS build
WORKDIR /workdir
COPY . /workdir
RUN nix --extra-experimental-features nix-command --extra-experimental-features flakes build /workdir#extension

FROM scratch
COPY --from=0 /workdir/result /tucant-extension.zip