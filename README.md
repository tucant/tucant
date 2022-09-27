# tucan-scraper

## Design

Panics are used for unexpected things including changes to the TUCAN interaction.
This means unexpected changes there can cause your application to panic. We decided to do this because there usually wouldn't be a better thing to do at the application side as you could just reject that particular operation otherwise. You can probably wrap a function call in an unwind wrapper if you really need this. "Expected" things like session timeout, service unavailable etc will return an Err.

## Installation

```bash
cargo install diesel_cli --no-default-features --features postgres

sudo docker build . -f Dockerfile-postgres --tag postgres-hunspell
sudo docker run -e POSTGRES_INITDB_ARGS="--data-checksums" -e POSTGRES_PASSWORD=password -p 5432:5432 -it postgres-hunspell

$HOME/.cargo/bin/diesel setup

RUST_BACKTRACE=1 RUST_LOG=tucan_scraper=info,info cargo run

cd tucant
npm install
npm start
```

##  Development

```bash
RUST_BACKTRACE=1 cargo test -- -Z unstable-options --nocapture --report-time

flatpak install flathub io.dbeaver.DBeaverCommunity

psql postgres://postgres:password@localhost:5432/tucant
```

## Interesting queries

```sql
SELECT encode(x.child, 'base64'), mmu."name"  FROM public.module_menu_tree x join module_menu_unfinished mmu  on x.child  = mmu.tucan_id  
ORDER BY encode(x.child, 'base64')

-- https://www.postgresql.org/docs/current/queries-with.html
WITH RECURSIVE search_tree(parent, child) AS (
    SELECT t.parent, t.child
    FROM module_menu_tree t WHERE child = decode('AAFWRgfUoQwAAU0igA5HiwABOeh5Z465', 'base64')
  UNION ALL
    SELECT t.parent, t.child
    FROM module_menu_tree t, search_tree st
    WHERE t.child = st.parent
)
SELECT * FROM search_tree;

```
