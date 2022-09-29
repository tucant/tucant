# SPDX-FileCopyrightText: The tucant Contributors
#
# SPDX-License-Identifier: AGPL-3.0-or-later

FROM alpine:edge as builder
WORKDIR /usr/src/myapp
RUN apk add --no-cache cargo openssl pkgconfig
RUN apk add --no-cache --repository=http://dl-cdn.alpinelinux.org/alpine/edge/testing mold
COPY . .
RUN cargo install --path .

FROM scratch

COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

ENTRYPOINT ["/usr/local/bin/myapp"]