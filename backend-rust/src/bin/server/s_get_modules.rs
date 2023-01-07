// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::utils::calculate_paths;
use crate::WithTucanUrl;
use tucant::MyError;

use axum::extract::State;

use axum::Json;

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

#[ts]
pub async fn get_modules(
    session: TucanSession,
    tucan: State<Tucan>,
    input: Json<Option<String>>,
) -> Result<Json<WithTucanUrl<ModuleMenuResponse>>, MyError> {
    let tucan = tucan.continue_session(session.clone());

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
            let binary_path = base64::decode_engine(
                input.as_bytes(),
                &base64::engine::fast_portable::FastPortable::from(
                    &base64::alphabet::URL_SAFE,
                    base64::engine::fast_portable::NO_PAD,
                ),
            )
            .unwrap();
            let (module_menu, subentries) = tucan
                .registration(Registration {
                    path: binary_path.clone(),
                })
                .await?;

            let mut connection = tucan.tucan.pool.get().await?;

            let path_to_root: Vec<ModuleMenuPathPart> = sql_query(
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

            let paths = calculate_paths(&path_to_root);

            ModuleMenuResponse {
                module_menu,
                entries: subentries,
                path: paths,
            }
        }
    };

    let url: TucanProgram = input.0.as_ref().map_or_else(
        || RootRegistration {}.into(),
        |input| {
            let binary_path = base64::decode_engine(
                input.as_bytes(),
                &base64::engine::fast_portable::FastPortable::from(
                    &base64::alphabet::URL_SAFE,
                    base64::engine::fast_portable::NO_PAD,
                ),
            )
            .unwrap();
            Registration { path: binary_path }.into()
        },
    );

    Ok(Json(WithTucanUrl {
        tucan_url: url.to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: value,
    }))
}
