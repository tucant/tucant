// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// $HOME/.cargo/bin/diesel database reset && cargo run --bin test_client
#[allow(clippy::too_many_lines)]
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "full-text-search")]
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

            // Dashboard index pattern needs to not include timestamp

            env_logger::init();

            // https://codarium.substack.com/p/designing-an-optimal-multi-language
            // https://opensearch.org/docs/latest/opensearch/query-dsl/full-text/
            // https://opensearch.org/docs/latest/opensearch/query-dsl/text-analyzers

            let tucan = Tucan::new()?;

            let mut rng = rand::thread_rng();

            let rand_string: String = (0..10)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();

            let index_name: String = format!("tucant_modules_{rand_string}");

            // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-htmlstrip-charfilter.html
            // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-lang-analyzer.html#english-analyzer

            // TODO FIXME searching for "Funktional" doesnt highlight body because of no matches?
            // no it seems like its some other weird stuff

            // TODO https://www.elastic.co/guide/en/elasticsearch/reference/current/test-analyzer.html
            // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-dict-decomp-tokenfilter.html
            // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-hyp-decomp-tokenfilter.html
            // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-stemmer-tokenfilter.html
            // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-snowball-tokenfilter.html
            // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-hunspell-tokenfilter.html
            // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-multiplexer-tokenfilter.html
            // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-remove-duplicates-tokenfilter.html

            // maybe multiplex, then german and english and then combine?

            // http://localhost:5601/app/dev_tools#/console
            /*
            GET tucant_modules/_analyze
            {
              "analyzer" : "my_german",
              "text" : "Funktional Funktionale Funktionalen"
            }
            */

            let response = tucan
                .opensearch
                .indices()
                .create(IndicesCreateParts::Index(&index_name))
                .body(json!({
                    "settings": {
                        "analysis": {
                            "analyzer": {
                                "my_english": {
                                    "tokenizer": "standard",
                                    "filter": [
                                        "english_possessive_stemmer",
                                        "lowercase",
                                        "english_stop",
                                        "german_stop",
                                        "english_stemmer"
                                    ],
                                    "char_filter": [
                                        "html_strip"
                                    ]
                                },
                                "my_german": {
                                    "tokenizer": "standard",
                                    "filter": [
                                        "lowercase",
                                        "german_stop",
                                        "english_stop",
                                        // "german_normalization",
                                        "german_hunspell",
                                        "german_snowball"
                                    ],
                                    "char_filter": [
                                        "html_strip"
                                    ]
                                }
                            },
                            "filter": {
                                "english_stop": {
                                    "type": "stop",
                                    "stopwords": "_english_"
                                },
                                "english_stemmer": {
                                    "type": "stemmer",
                                    "language": "english"
                                },
                                "english_possessive_stemmer": {
                                    "type": "stemmer",
                                    "language": "possessive_english"
                                },
                                "german_stop": {
                                    "type": "stop",
                                    "stopwords": "_german_"
                                },
                                "german_stemmer": {
                                    "type": "stemmer",
                                    "language": "german"
                                },
                                "german_snowball": {
                                    "type": "snowball",
                                    "language": "German"
                                },
                                "german_hunspell": {
                                    "type": "hunspell",
                                    "locale": "de_DE",
                                }
                            }
                        }
                    },
                    "mappings": {
                        "properties": {
                            "content": {
                                "type": "text",
                                "term_vector": "with_positions_offsets",
                                "fields": {
                                    "de": {
                                        "term_vector": "with_positions_offsets",
                                        "type": "text",
                                        "analyzer": "my_german"
                                    },
                                    "en": {
                                        "term_vector": "with_positions_offsets",
                                        "type": "text",
                                        "analyzer": "my_english"
                                    },
                                }
                            },
                            "title": {
                                "type": "text",
                                "term_vector": "with_positions_offsets",
                                "fields": {
                                    "de": {
                                        "term_vector": "with_positions_offsets",
                                        "type": "text",
                                        "analyzer": "my_german"
                                    },
                                    "en": {
                                        "term_vector": "with_positions_offsets",
                                        "type": "text",
                                        "analyzer": "my_english"
                                    },
                                }
                            }
                        }
                    }
                }
                ))
                .send()
                .await?;

            let exception = response.exception().await?;
            if let Some(exception) = exception {
                Err(anyhow::anyhow!("{:?}", exception))?;
            }

            // let response_body = response.json::<Value>().await?;

            let mut connection = tucan.pool.get()?;
            let modules: Vec<MaybeCompleteModule> = modules_unfinished::table
                .select(MODULES_UNFINISHED)
                .order(modules_unfinished::title)
                .load::<MaybeCompleteModule>(&mut connection)
                .await?;

            let body: Vec<JsonBody<_>> = modules
                .iter()
                .map(Into::<InternalModule>::into)
                .flat_map(|m| {
                    let base64_tucan_id = BASE64_URL_SAFE_NO_PAD.encode(&m.tucan_id);
                    [
                        json!({"index": {"_id": base64_tucan_id}}).into(),
                        json!({
                            "id": base64_tucan_id,
                            "last_checked": m.tucan_last_checked,
                            "title": m.title,
                            "module_id": m.module_id,
                            "credits": m.credits,
                            "content": m.content
                        })
                        .into(),
                    ]
                    .into_iter()
                })
                .collect_vec();

            let response = tucan
                .opensearch
                .bulk(BulkParts::Index(&index_name))
                .refresh(Refresh::WaitFor)
                .body(body)
                .send()
                .await?;

            let exception = response.exception().await?;
            if let Some(exception) = exception {
                Err(anyhow::anyhow!("{:?}", exception))?;
            }

            // https://www.elastic.co/guide/en/elasticsearch/reference/current/search-suggesters.html#completion-suggester

            let response = tucan
                .opensearch
                .indices()
                .update_aliases()
                .body(json!({
                      "actions": [
                  {
                    "remove": {
                      "index": "tucant_modules_*",
                      "alias": "tucant_modules"
                    }
                  },
                  {
                      "add": {
                          "index": index_name,
                          "alias": "tucant_modules"
                      }
                  }
                ]
                  }))
                .send()
                .await?;

            let exception = response.exception().await?;
            if let Some(exception) = exception {
                Err(anyhow::anyhow!("{:?}", exception))?;
            }

            let response = tucan
                .opensearch
                .indices()
                .get(opensearch::indices::IndicesGetParts::Index(&[
                    "tucant_modules_*",
                ]))
                .send()
                .await?;

            let _response_body = response.json::<Value>().await?;
            // TODO FIXME delete indexes here

            Ok(())
        })?;
    Ok(())
}
