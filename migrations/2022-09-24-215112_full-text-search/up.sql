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

CREATE TEXT SEARCH CONFIGURATION tucan (PARSER = default);
ALTER TEXT SEARCH CONFIGURATION tucan ADD MAPPING FOR asciihword, asciiword, hword, hword_asciipart, hword_part, word WITH german_hunspell, english_hunspell, german_stem; -- maybe german_stem but also with english stop words?
ALTER TEXT SEARCH CONFIGURATION tucan ADD MAPPING FOR email, file, float, host, hword_numpart, int, numhword, numword, sfloat, uint, url, url_path, version WITH simple;

CREATE INDEX modules_idx ON modules_unfinished USING GIN (to_tsvector('tucan', content));
