# tucan-scraper

## Design

Panics are used for unexpected things including changes to the TUCAN interaction.
This means unexpected changes there can cause your application to panic. We decided to do this because there usually wouldn't be a better thing to do at the application side as you could just reject that particular operation otherwise. You can probably wrap a function call in an unwind wrapper if you really need this. "Expected" things like session timeout, service unavailable etc will return an Err.

## Installation

```bash
cargo install diesel_cli --no-default-features --features postgres

sudo docker run -e POSTGRES_INITDB_ARGS="--data-checksums" -e POSTGRES_PASSWORD=password -p 5432:5432 -it postgres:15beta4-alpine

echo DATABASE_URL=postgres://postgres:password@localhost/postgres > .env

diesel setup

RUST_BACKTRACE=1 RUST_LOG=actix_server=debug cargo run

RUST_BACKTRACE=1 cargo test -- -Z unstable-options --nocapture --report-time
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

```