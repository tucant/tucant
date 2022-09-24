# tucan-scraper

## Design

Panics are used for unexpected things including changes to the TUCAN interaction.
This means unexpected changes there can cause your application to panic. We decided to do this because there usually wouldn't be a better thing to do at the application side as you could just reject that particular operation otherwise. You can probably wrap a function call in an unwind wrapper if you really need this. "Expected" things like session timeout, service unavailable etc will return an Err.

## Installation

```bash
cargo install diesel_cli --no-default-features --features postgres

sudo docker build . -f Dockerfile-postgres --tag postgres-hunspell
sudo docker run -e POSTGRES_INITDB_ARGS="--data-checksums" -e POSTGRES_PASSWORD=password -p 5432:5432 -it postgres-hunspell

echo DATABASE_URL=postgres://postgres:password@localhost/tucant > .env

diesel setup

RUST_BACKTRACE=1 RUST_LOG=tucan_scraper=trace,info cargo run

RUST_BACKTRACE=1 cargo test -- -Z unstable-options --nocapture --report-time

flatpak install flathub io.dbeaver.DBeaverCommunity
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

-- https://www.postgresql.org/docs/current/textsearch-configuration.html
-- https://www.postgresql.org/docs/current/textsearch-psql.html

-- sudo apt-get install hunspell hunspell-tools hunspell-de-de-frami
-- postgresql is doing magic and autoinstalls these

CREATE TEXT SEARCH DICTIONARY english_hunspell (
    TEMPLATE = ispell,
    DictFile = en_us,
    AffFile = en_us,
    StopWords = english
);

CREATE TEXT SEARCH DICTIONARY german_hunspell (
    TEMPLATE = ispell,
    DictFile = de_de_frami,
    AffFile = de_de_frami,
    StopWords = german
);

-- https://www.postgresql.org/docs/current/sql-createtsconfig.html
CREATE TEXT SEARCH CONFIGURATION tucan (PARSER = default);

SELECT * FROM ts_token_type('default');

psql postgres://postgres:password@localhost:5432/tucant
\dF+ german
\dF+ english

ALTER TEXT SEARCH CONFIGURATION tucan ADD MAPPING FOR asciihword, asciiword, hword, hword_asciipart, hword_part, word WITH german_hunspell, english_hunspell, german_stem; -- maybe german_stem but also with english stop words?
ALTER TEXT SEARCH CONFIGURATION tucan ADD MAPPING FOR email, file, float, host, hword_numpart, int, numhword, numword, sfloat, uint, url, url_path, version WITH simple;

-- https://www.postgresql.org/docs/current/sql-altertsconfig.html


-- https://www.postgresql.org/docs/devel/textsearch-debugging.html
select title, (to_tsvector('tucan', content) @@ to_tsquery('tucan', 'chemie & analytik')) as text_found from MODULES_UNFINISHED order by text_found desc;

SELECT alias, description, token FROM ts_debug('tucan', 'Was machst du heute?');

SELECT ts_lexize('english_hunspell', 'stars');

SELECT * FROM ts_parse('tucan', (select content from modules_unfinished where tucan_id = 383852987293994));


SELECT ts_lexize('tucan', (select content from modules_unfinished where tucan_id = 383852987293994));



select title, to_tsvector('tucan', content) from modules_unfinished;


```