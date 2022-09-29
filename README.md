<!--
SPDX-FileCopyrightText: The tucant Contributors

SPDX-License-Identifier: AGPL-3.0-or-later
-->

# tucant

tucant - a nicer, faster and more featureful frontend to TUCaN

Copyright (C) The tucant Contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

## Architecture

### Backend

The backend is written in [Rust](https://www.rust-lang.org/) and is supposed to crawl TUCaN when first logging in. This data is then stored in a database to allow arbitrary analysis with it. There are also some web API endpoints for common things like navigating modules and full text search.

### Frontend

The frontend is written using [React](https://reactjs.org/) and [TypeScript](https://www.typescriptlang.org/). It should be a much faster, nicer looking and more featureful frontend to TUCaN.

## Installation of backend

```bash
cd backend-rust

# You might need to install libpq before: sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features postgres

# Depending on your system you might have to run these with sudo
docker build . -f Dockerfile-postgres --tag postgres-hunspell
docker run -d -e POSTGRES_INITDB_ARGS="--data-checksums" -e POSTGRES_PASSWORD=password -p 5432:5432 -it postgres-hunspell

$HOME/.cargo/bin/diesel setup

RUST_BACKTRACE=1 RUST_LOG=tucan_scraper=info,info cargo run
```

## Installation of frontend

```bash
cd frontend-react
npm ci
npm run dev
```

## Development

```bash
# if you want automatic formatting and linting on commit
ln -srf pre-commit.sh .git/hooks/pre-commit

# run some tests (currently not many)
cd backend-rust
RUST_BACKTRACE=1 cargo test -- -Z unstable-options --nocapture --report-time

# nice gui to access the database
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install flathub io.dbeaver.DBeaverCommunity

# console application to access the database
# Only used for debugging, you may need to install psql first: sudo apt-get install postgresql
psql postgres://postgres:password@localhost:5432/tucant
```

## Interesting queries

```sql
-- there seem to be modules that are in multiple menus
SELECT modules_unfinished.title, module_menu_unfinished.name, modules_unfinished.tucan_id FROM module_menu_module NATURAL JOIN (SELECT module_id FROM module_menu_module GROUP BY module_id HAVING COUNT(*) != 1) dm JOIN module_menu_unfinished ON module_menu_unfinished.tucan_id = module_menu_module.module_menu_id JOIN modules_unfinished ON modules_unfinished.tucan_id = module_menu_module.module_id ORDER BY modules_unfinished.tucan_id;

-- https://www.postgresql.org/docs/current/queries-with.html
-- get path from module menu entry to root
WITH RECURSIVE search_tree(parent, child) AS (
    SELECT t.parent, t.tucan_id
    FROM module_menu_unfinished t JOIN module_menu_module mmm ON mmm.module_menu_id = t.tucan_id WHERE mmm.module_id = '\x000154f481a77362'
  UNION
    SELECT t.parent, t.tucan_id
    FROM module_menu_unfinished t, search_tree st
    WHERE t.tucan_id = st.parent
)
SELECT * FROM search_tree;


SELECT title FROM modules_unfinished WHERE tucan_id = '\x000154f481a77362';

SELECT * FROM module_menu_module WHERE module_id = '\x000154f481a77362';

-- https://explain.dalibo.com/
SET enable_seqscan TO off;
WITH RECURSIVE search_tree AS (
    SELECT t.parent, t.tucan_id, t.name
    FROM module_menu_unfinished t JOIN module_menu_module mmm ON mmm.module_menu_id = t.tucan_id WHERE mmm.module_id = '\x000154f481a77362'
  UNION
    SELECT t.parent, t.tucan_id, t.name
    FROM module_menu_unfinished t JOIN search_tree st
    ON t.tucan_id = st.parent
)
SELECT * FROM search_tree;

psql postgres://postgres:password@localhost:5432/tucant -XqAt -f explain.sql > analyze.json


WITH RECURSIVE search_tree(parent, tucan_id, name) AS (
    SELECT '\x0001564607d4a10c00014d22800c5b4b000130aad487c66c'::bytea, '\x0001564607d4a10c00014d22800c5b4b000130f9c84409be'::bytea, 'Serviceveranstaltungen des FB Mathematik'
  UNION
    SELECT t.parent, t.tucan_id, t.name
    FROM module_menu_unfinished t JOIN search_tree st
    ON t.tucan_id = st.parent
)
SELECT * FROM search_tree;


WITH RECURSIVE search_tree(parent) AS (
    SELECT '\x0001564607d4a10c00014d22800c5b4b000130aad487c66c'::bytea
  UNION
    SELECT t.parent
    FROM module_menu_unfinished t JOIN search_tree st
    ON t.tucan_id = st.parent
)
SELECT * FROM search_tree;
```

## Add license headers

```bash
reuse addheader --copyright "The tucant Contributors" --license AGPL-3.0-or-later --exclude-year --recursive --skip-unrecognised .
```
