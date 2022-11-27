// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::{HashMap, VecDeque};

use tucant::MyError;
use crate::WithTucanUrl;

use axum::extract::State;
use axum::Json;
use diesel::sql_types::Bytea;

use diesel::sql_query;
use diesel_async::RunQueryDsl;

use tucant::models::ModuleMenuPathPart;
use tucant::models::ModuleResponse;
use tucant::models::TucanSession;
use tucant::tucan::Tucan;
use tucant::url::Moduledetails;
use tucant::url::TucanProgram;
use tucant_derive::ts;

#[ts]
pub async fn module(
    session: TucanSession,
    tucan: State<Tucan>,
    input: Json<String>,
) -> Result<Json<WithTucanUrl<ModuleResponse>>, MyError> {
    let mut connection = tucan.pool.get().await?;

    let binary_path = base64::decode_config(input.as_bytes(), base64::URL_SAFE_NO_PAD).unwrap();

    let tucan = tucan.continue_session(session.clone()).await.unwrap();

    let result = tucan
        .module(Moduledetails {
            id: binary_path.clone(),
        })
        .await?
        .0;

    let path_to_root = sql_query(
            r#"
                WITH RECURSIVE search_tree AS (
                    SELECT t.parent, t.tucan_id, t.name, true as leaf
                    FROM module_menu_unfinished t JOIN module_menu_module mmm ON mmm.module_menu_id = t.tucan_id WHERE mmm.module_id = $1
                  UNION
                    SELECT t.parent, t.tucan_id, t.name, false as leaf
                    FROM module_menu_unfinished t JOIN search_tree st
                    ON t.tucan_id = st.parent
                )
                SELECT * FROM search_tree;
"#,
        )
        .bind::<Bytea, _>(binary_path.clone())
        .load::<ModuleMenuPathPart>(&mut connection)
        .await?;

    let leaves = path_to_root.iter().take_while(|v| v.leaf);

    let nonleaves = path_to_root
        .iter()
        .rev()
        .take_while(|v| !v.leaf)
        .map(|v| (&v.tucan_id, v))
        .collect::<HashMap<_, _>>();

    let paths = leaves
        .map(|l| {
            let mut current = Some(&l);
            let mut path = VecDeque::new();
            while let Some(curr) = current {
                path.push_front(curr.to_owned().to_owned());
                if let Some(parent) = &curr.parent {
                    current = nonleaves.get(&parent);
                } else {
                    break;
                }
            }
            path
        })
        .collect::<Vec<_>>();

    let result = ModuleResponse {
        module: result,
        path: paths,
    };

    Ok(Json(WithTucanUrl {
        tucan_url: if *input == "TUCANSCHEISS" {
            "https://github.com/mohe2015/tucant/issues/104".to_string()
        } else {
            Into::<TucanProgram>::into(Moduledetails {
                id: binary_path.clone(),
            })
            .to_tucan_url(Some(session.session_nr.try_into().unwrap()))
        },
        inner: result,
    }))
}
