// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::VecDeque;

use crate::utils::calculate_paths;
use crate::WithTucanUrl;
use diesel::sql_query;
use diesel::sql_types::Text;
use tucant::MyError;

use axum::extract::State;

use axum::Json;

use tucant::models::TucanSession;
use tucant::models::VVMenuItem;
use tucant::models::VVMenuPathPart;
use tucant::tucan::Tucan;
use tucant::url::Action;
use tucant::url::Externalpages;
use tucant::url::TucanProgram;
use tucant_derive::ts;

#[ts]
pub async fn courses(
    session: TucanSession,
    tucan: State<Tucan>,
    input: Json<Option<String>>,
) -> Result<
    Json<
        WithTucanUrl<(
            VVMenuItem,
            Vec<VVMenuItem>,
            Vec<tucant::models::Course>,
            Vec<VecDeque<VVMenuPathPart>>,
        )>,
    >,
    MyError,
> {
    let tucan = tucan
        .continue_session(session.clone())
        .await?
        .as_unauthenticated();

    let value = match input.0 {
        None => {
            let result = tucan.vv_root().await?;

            (result.0, result.1, result.2, Vec::new())
        }
        Some(ref input) => {
            use diesel_async::RunQueryDsl;

            let result = tucan
                .vv(Action {
                    magic: input.clone(),
                })
                .await?;

            let mut connection = tucan.pool.get().await?;

            let path_to_root: Vec<VVMenuPathPart> = sql_query(
                r#"
                        WITH RECURSIVE search_tree AS (
                            SELECT t.parent, t.tucan_id, t.name, true as leaf
                            FROM vv_menu_unfinished t WHERE t.tucan_id = $1
                          UNION
                            SELECT t.parent, t.tucan_id, t.name, false as leaf
                            FROM vv_menu_unfinished t JOIN search_tree st
                            ON t.tucan_id = st.parent
                        )
                        SELECT * FROM search_tree;
        "#,
            )
            .bind::<Text, _>(input.clone())
            .load::<VVMenuPathPart>(&mut connection)
            .await?;

            let paths = calculate_paths(&path_to_root);

            (result.0, result.1, result.2, paths)
        }
    };

    // TODO FIXME show path

    let url: TucanProgram = input.0.as_ref().map_or_else(
        || {
            Externalpages {
                id: 344,
                name: "welcome".to_string(),
            }
            .into()
        },
        |input| {
            Action {
                magic: input.clone(),
            }
            .into()
        },
    );

    Ok(Json(WithTucanUrl {
        tucan_url: url.to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: value,
    }))
}
