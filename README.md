# tucan-scraper

```bash
cargo install diesel_cli --no-default-features --features postgres

sudo docker run -e POSTGRES_INITDB_ARGS="--data-checksums" -e POSTGRES_PASSWORD=password -p 5432:5432 -it postgres:15beta4-alpine

echo DATABASE_URL=postgres://postgres:password@localhost/postgres > .env

RUST_BACKTRACE=1 cargo +nightly run --release -- --help
RUST_BACKTRACE=1 cargo +nightly run --release tuid login
```

```
hashcat -a 0 -m 0 hash wordlist -r /usr/share/hashcat/rules/best64.rule

princeprocessor wordlist | hashcat -m 0 hash
princeprocessor wordlist | hashcat -m 900 hash
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


https://www.sqlite.org/lang_with.html



CREATE VIRTUAL TABLE modules_fts5 USING fts5(module_id UNINDEXED, title, content);

INSERT INTO modules_fts5 (module_id, title, content) SELECT module_id, title, content FROM modules;

SELECT snippet(modules_fts5, 2, '<b>', '</b>', '...', 10) FROM modules_fts5 WHERE content MATCH 'Objektorientierte' ORDER BY rank;


https://www.sqlite.org/fts5.html

```