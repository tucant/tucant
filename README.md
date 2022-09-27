# tucan-scraper

## Design

Panics are used for unexpected things including changes to the TUCAN interaction.
This means unexpected changes there can cause your application to panic. We decided to do this because there usually wouldn't be a better thing to do at the application side as you could just reject that particular operation otherwise. You can probably wrap a function call in an unwind wrapper if you really need this. "Expected" things like session timeout, service unavailable etc will return an Err.

## Installation

```bash
# if you want automatic formatting and linting on commit
ln -srf pre-commit.sh .git/hooks/pre-commit

# You might need to install libpq before: sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features postgres

# Depending on your system you might have to run these with sudo
docker build . -f Dockerfile-postgres --tag postgres-hunspell
docker run -e POSTGRES_INITDB_ARGS="--data-checksums" -e POSTGRES_PASSWORD=password -p 5432:5432 -it postgres-hunspell

$HOME/.cargo/bin/diesel setup

RUST_BACKTRACE=1 RUST_LOG=tucan_scraper=info,info cargo run

cd tucant
npm install
npm run dev
```

## Development

```bash
RUST_BACKTRACE=1 cargo test -- -Z unstable-options --nocapture --report-time

flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install flathub io.dbeaver.DBeaverCommunity

# Only used for debugging, you may need to install psql first: sudo apt-get install postgresql
psql postgres://postgres:password@localhost:5432/tucant
```

## Interesting queries

```sql
-- there don't seem to be module entries that have multiple parents
SELECT child, COUNT(*) FROM module_menu_tree GROUP BY child HAVING COUNT(*) != 1;

-- there seem to be modules that are in multiple menus
SELECT modules_unfinished.title, module_menu_unfinished.name, modules_unfinished.tucan_id FROM module_menu_module NATURAL JOIN (SELECT module_id FROM module_menu_module GROUP BY module_id HAVING COUNT(*) != 1) dm JOIN module_menu_unfinished ON module_menu_unfinished.tucan_id = module_menu_module.module_menu_id JOIN modules_unfinished ON modules_unfinished.tucan_id = module_menu_module.module_id ORDER BY modules_unfinished.tucan_id;

-- https://www.postgresql.org/docs/current/queries-with.html
-- get path from module menu entry to root
WITH RECURSIVE search_tree(parent, child) AS (
    SELECT t.parent, t.child
    FROM module_menu_tree t JOIN module_menu_module mmm ON mmm.module_menu_id = t.child WHERE mmm.module_id = '\x000154f481a77362'
  UNION ALL
    SELECT t.parent, t.child
    FROM module_menu_tree t, search_tree st
    WHERE t.child = st.parent
)
SELECT * FROM search_tree;

```
