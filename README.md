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
-- Fachprüfungen mit mind. 2 CP
WITH RECURSIVE
  works_for_alice(n) AS (
    VALUES(5)
    UNION
    SELECT id FROM module_menu, works_for_alice
     WHERE module_menu.parent=works_for_alice.n
  )
SELECT modules.title, modules.credits FROM module_menu
 JOIN module_menu_module ON module_menu_module.module_menu_id = module_menu.id
 NATURAL JOIN modules
 WHERE module_menu.id IN works_for_alice AND modules.credits >= 2 ORDER BY modules.credits ASC;

-- Wahl- und Pflichtbereich
WITH RECURSIVE
  works_for_alice(n) AS (
    VALUES(3)
	UNION
	VALUES(4)
    UNION
    SELECT id FROM module_menu, works_for_alice
     WHERE module_menu.parent=works_for_alice.n
  )
SELECT modules.title, modules.credits FROM module_menu
 JOIN module_menu_module ON module_menu_module.module_menu_id = module_menu.id
 NATURAL JOIN modules
 WHERE module_menu.id IN works_for_alice AND modules.credits IS NOT NULL ORDER BY modules.credits ASC;

-- 4 CP Module
WITH RECURSIVE
  works_for_alice(n) AS (
    VALUES(3)
	UNION
	VALUES(4)
    UNION
    SELECT id FROM module_menu, works_for_alice
     WHERE module_menu.parent=works_for_alice.n
  )
SELECT modules.module_id, modules.title, modules.credits FROM module_menu
 JOIN module_menu_module ON module_menu_module.module_menu_id = module_menu.id
 NATURAL JOIN modules
 WHERE module_menu.id IN works_for_alice AND modules.credits IS NOT NULL AND modules.credits = 4 ORDER BY modules.credits ASC;

EXPLAIN ANALYZE SELECT "modules_unfinished"."tucan_id", "modules_unfinished"."title", ts_headline('tucan', (((((((("modules_unfinished"."module_id" || ' ')) || "modules_unfinished"."title")) || ' ')) || "modules_unfinished"."content")), websearch_to_tsquery('tucan', 'papierprüfung')), ts_rank_cd(tsv, websearch_to_tsquery('tucan', 'papierprüfung'), 1) FROM "modules_unfinished" WHERE tsv @@ websearch_to_tsquery('tucan', 'papierprüfung') ORDER BY ts_rank_cd(tsv, websearch_to_tsquery('tucan', 'papierprüfung'), 1) DESC;s
```
