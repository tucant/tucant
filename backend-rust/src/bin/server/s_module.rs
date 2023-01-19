// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::utils::calculate_paths;
use crate::AppState;
use crate::WithTucanUrl;
use tucant::MyError;

use axum::extract::State;
use axum::Json;
use diesel::sql_types::Bytea;

use diesel::sql_query;
use diesel_async::RunQueryDsl;

use base64::prelude::*;
use tucant::models::ModuleMenuPathPart;
use tucant::models::ModuleResponse;
use tucant::models::TucanSession;
use tucant::tucan::Tucan;
use tucant::url::Moduledetails;
use tucant::url::TucanProgram;
use tucant_derive::ts;

#[ts]
#[axum::debug_handler(state=AppState)]
pub async fn module(
    session: TucanSession,
    tucan: State<Tucan>,
    input: Json<String>,
) -> Result<Json<WithTucanUrl<ModuleResponse>>, MyError> {
    let mut connection = tucan.pool.get().await?;

    let binary_path = BASE64_URL_SAFE_NO_PAD.decode(input.as_bytes()).unwrap();

    let tucan = tucan.continue_session(session.clone());

    let result = tucan
        .module(Moduledetails {
            id: binary_path.clone(),
        })
        .await?
        .0;

    let path_to_root: Vec<ModuleMenuPathPart> = sql_query(
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

    let paths = calculate_paths(&path_to_root);

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
