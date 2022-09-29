EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON) 
WITH RECURSIVE search_tree(parent) AS (
    SELECT '\x0001564607d4a10c00014d22800c5b4b000130aad487c66c'::bytea
  UNION
    SELECT t.parent
    FROM module_menu_unfinished t JOIN search_tree st
    ON t.tucan_id = st.parent
)
SELECT * FROM search_tree;