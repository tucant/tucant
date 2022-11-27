// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use std::collections::VecDeque;

use crate::MyError;
use crate::WithTucanUrl;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::response::Parts;
use axum::response::Response;
use axum::Json;
use axum_extra::extract::PrivateCookieJar;
use diesel::sql_query;

use diesel::sql_types::Bytea;

use diesel_async::RunQueryDsl;

use tucant::models::ModuleMenuPathPart;
use tucant::models::ModuleMenuResponse;
use tucant::models::TucanSession;
use tucant::tucan::Tucan;
use tucant::url::Registration;
use tucant::url::RootRegistration;
use tucant::url::TucanProgram;
use tucant_derive::ts;

// trailing slash is menu
#[ts]
#[axum::debug_handler]
pub async fn get_modules(
    session: TucanSession,
    tucan: State<Tucan>,
    input: Json<Option<String>>,
) -> Result<Json<WithTucanUrl<ModuleMenuResponse>>, MyError> {
    let tucan = tucan.continue_session(session.clone()).await.unwrap();

    let value = match input.0 {
        None => {
            let module_menu = tucan.root_registration().await?;
            ModuleMenuResponse {
                module_menu: module_menu.clone(),
                entries: tucant::models::Registration {
                    modules_and_courses: vec![],
                    submenus: vec![module_menu],
                },
                path: Vec::new(),
            }
        }
        Some(ref input) => {
            let binary_path =
                base64::decode_config(input.as_bytes(), base64::URL_SAFE_NO_PAD).unwrap();
            let (module_menu, subentries) = tucan
                .registration(Registration {
                    path: binary_path.clone(),
                })
                .await?;

            let mut connection = tucan.tucan.pool.get().await?;

            let path_to_root = sql_query(
                r#"
                        WITH RECURSIVE search_tree AS (
                            SELECT t.parent, t.tucan_id, t.name, true as leaf
                            FROM module_menu_unfinished t WHERE t.tucan_id = $1
                          UNION
                            SELECT t.parent, t.tucan_id, t.name, false as leaf
                            FROM module_menu_unfinished t JOIN search_tree st
                            ON t.tucan_id = st.parent
                        )
                        SELECT * FROM search_tree;
        "#,
            )
            .bind::<Bytea, _>(binary_path)
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

            ModuleMenuResponse {
                module_menu,
                entries: subentries,
                path: paths,
            }
        }
    };

    let url: TucanProgram = match input.0 {
        Some(ref input) => {
            let binary_path =
                base64::decode_config(input.as_bytes(), base64::URL_SAFE_NO_PAD).unwrap();
            Registration { path: binary_path }.into()
        }
        None => RootRegistration {}.into(),
    };

    Ok(Json(WithTucanUrl {
        tucan_url: url.to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: value,
    }))
}
