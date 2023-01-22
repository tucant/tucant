<!--
SPDX-FileCopyrightText: The tucant Contributors

SPDX-License-Identifier: AGPL-3.0-or-later
-->

<!-- Copyright (C) The tucant Contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>. -->

<h1 align="center">
  TUCaN't

[![GitHub license](https://img.shields.io/github/license/mohe2015/tucant.svg)](https://github.com/mohe2015/tucant/blob/main/LICENSE)
![GitHub commit activity](https://img.shields.io/github/commit-activity/w/mohe2015/tucant)
[![Github stars](https://img.shields.io/github/stars/mohe2015/tucant.svg)](https://GitHub.com/mohe2015/tucant/stargazers/)
[![Node.js CI](https://github.com/mohe2015/tucant/actions/workflows/frontend-react.yml/badge.svg)](https://github.com/mohe2015/tucant/actions/workflows/frontend-react.yml)
[![Rust](https://github.com/mohe2015/tucant/actions/workflows/rust.yml/badge.svg)](https://github.com/mohe2015/tucant/actions/workflows/rust.yml)
</h1>

A **nicer**, **faster** and more **featureful** frontend to <a href="https://www.tucan.tu-darmstadt.de/" target="_blank">TUCaN</a>.

## How it works

TUCaN't consists of three components: a fontend, a backend and a database. The frontend only communicates with the backend, which in turn communicates with the database.

### Frontend

The frontend is written using [React](https://reactjs.org/) and [TypeScript](https://www.typescriptlang.org/). It should be a much faster, nicer looking and more featureful frontend to TUCaN.

### Backend

The backend is written in [Rust](https://www.rust-lang.org/) and is supposed to crawl TUCaN when first logging in. This data is then stored in a database to allow arbitrary analysis with it. There are also some web API endpoints for common things like navigating modules and full text search.

### Database

The database is a [PostgreSQL](https://www.postgresql.org/) database. It is used to store the crawled data from TUCaN.

When resetting the database remember to also remove `sessions.key`.

## Bookmarklet

The following bookmarklet opens the tucan page in tucant (and optionally logs you in):

```
javascript:window.location.href = `http://localhost:8080/login-hack?${document.querySelector("#logoutButton") ? new URL(document.querySelector("#logoutButton").href).searchParams.get("ARGUMENTS").split(",")[0].replace("-N", "session_nr=") + "&" : ""}${document.cookie.split(";").find((item) => item.trim().startsWith("cnsc=")) ? "session_id=" + document.cookie.split(";").find((item) => item.trim().startsWith("cnsc=")).split("=")[1] + "&" : ""}redirect=${encodeURIComponent(window.location.href)}`
```

## How to run

### Requirements

- [Docker](https://www.docker.com/)
- [Node.js](https://nodejs.org/en/)
- [NPM](https://www.npmjs.com/)
- [Rust](https://www.rust-lang.org/)
- [libpq-dev[_el_]](https://www.postgresql.org/docs/current/libpq.html) (might be called differently on other distributions)

### Database

```bash
cd backend-rust

# We recommend using podman (with docker compat)
docker build . --pull -f Dockerfile-postgres --tag postgres-hunspell
docker run --name tucant-postgres -d --restart unless-stopped -e POSTGRES_INITDB_ARGS="--data-checksums" -e POSTGRES_PASSWORD=password -p 5432:5432 -it postgres-hunspell
```

### Backend

```bash
cd backend-rust

# https://github.com/rust-lang/rust-clippy/issues/10134
rustup default nightly-2022-12-29

cargo install diesel_cli --no-default-features --features postgres
cp .env.sample .env
$HOME/.cargo/bin/diesel setup

# run this each time you want to run the backend
RUST_BACKTRACE=1 RUST_LOG=tucan_scraper=info,info cargo run --bin server
```

### Frontend

```bash
cd frontend-react

# install dependencies each time the package.json changed
npm ci

# run this each time you want to run the frontend
npm run dev
```

## Development Notes

If you want automatic formatting and linting on commit

```bash
ln -srf pre-commit.sh .git/hooks/pre-commit
```

If you want the backend to automatically restart on file change

```bash
cargo install cargo-watch
cargo watch -x check -s 'touch .trigger'
RUST_BACKTRACE=1 cargo watch --no-gitignore -w ./.trigger -s 'cargo run --bin server'
```

To test the backend

```bash
cd backend-rust
RUST_BACKTRACE=1 cargo test -- -Z unstable-options --nocapture --report-time
```

To get a nice GUI of the database on Linux

```bash
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install flathub io.dbeaver.DBeaverCommunity
flatpak run io.dbeaver.DBeaverCommunity
```

To access the database from using a CLI on Linux  
`posgresql` needs to be installed on the host system

```bash
psql postgres://postgres:password@localhost:5432/tucant
```

Tracing:

```bash
# https://www.jaegertracing.io/docs/1.39/getting-started/
sudo docker run -d --name jaeger \
  -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 6831:6831/udp \
  -p 6832:6832/udp \
  -p 5778:5778 \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 14250:14250 \
  -p 14268:14268 \
  -p 14269:14269 \
  -p 9411:9411 \
  jaegertracing/all-in-one:latest
cargo run --bin server
echo http://localhost:16686/
```

Add license headers  
`reuse` needs to be installed on the host system

```bash
reuse addheader --copyright "The tucant Contributors" --license AGPL-3.0-or-later --exclude-year --recursive --skip-unrecognised .
```

Optimize dependencies:

```bash
cargo tree -d --format "{p} {f}"

cargo hack build --workspace --all-targets

cargo udeps --workspace --all-targets

cargo install cargo-machete
cargo machete --workspace --all-targets

cargo install --locked cargo-deny
cargo deny check --workspace --all-targets

cargo install --locked cargo-outdated
cargo outdated --workspace --all-targets

cargo tree --no-dedupe --prefix none | sort -k 1 | uniq -c | sort -k 1 -n -r


```

Clippy

```
cargo clippy --all-targets --all-features
```

## Search

https://opensearch.org/docs/latest/opensearch/search-template/

https://opensearch.org/docs/latest/opensearch/ux/
autocomplete

https://opensearch.org/docs/latest/api-reference/document-apis/index-document/

https://opensearch.org/docs/latest/api-reference/document-apis/bulk/

https://opensearch.org/docs/latest/api-reference/explain/

https://opensearch.org/docs/latest/api-reference/search/
