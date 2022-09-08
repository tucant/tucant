# tucan-scraper

```bash
cargo install sqlx-cli
sqlx database setup
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
 WHERE module_menu.id IN works_for_alice;


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








WITH RECURSIVE
  works_for_alice(n) AS (
    VALUES(4)
    UNION
    SELECT id FROM module_menu, works_for_alice
     WHERE module_menu.parent=works_for_alice.n
  )
SELECT module_menu.id FROM module_menu
 WHERE module_menu.id IN works_for_alice;

WITH RECURSIVE
  parent_of(name, parent) AS
    (SELECT name, mom FROM family UNION SELECT name, dad FROM family),
  ancestor_of_alice(name) AS
    (SELECT parent FROM parent_of WHERE name='Alice'
     UNION ALL
     SELECT parent FROM parent_of JOIN ancestor_of_alice USING(name))
SELECT family.name FROM ancestor_of_alice, family
 WHERE ancestor_of_alice.name=family.name
   AND died IS NULL
 ORDER BY born;

https://www.sqlite.org/lang_with.html
-- SELECT title, module_id FROM modules NATURAL JOIN module_menu_module WHERE credits >= 2 AND 




```