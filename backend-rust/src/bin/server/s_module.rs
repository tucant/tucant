// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::{HashMap, VecDeque};

use crate::s_get_modules::ModuleMenuPathPart;
use crate::MyError;
use actix_session::Session;
use actix_web::web::Json;

use actix_web::{get, web::Data};
use diesel::sql_types::Bytea;
use diesel::QueryDsl;
use diesel::{sql_query, ExpressionMethods};
use diesel_async::RunQueryDsl;
use serde::Serialize;
use tucant::{models::Module, schema::modules_unfinished, tucan::Tucan};
use tucant_derive::{ts, Typescriptable};

#[derive(Serialize, Typescriptable)]
pub struct ModuleResponse {
    module: Module,
    path: Vec<VecDeque<ModuleMenuPathPart>>,
}

#[ts]
#[get("/module")]
pub async fn module(
    _: Session,
    tucan: Data<Tucan>,
    input: Json<String>,
) -> Result<Json<ModuleResponse>, MyError> {
    let mut connection = tucan.pool.get().await?;

    let binary_path = base64::decode(input.as_bytes()).unwrap();

    let result = modules_unfinished::table
        .filter(modules_unfinished::tucan_id.eq(&binary_path))
        .select((
            modules_unfinished::tucan_id,
            modules_unfinished::tucan_last_checked,
            modules_unfinished::title,
            modules_unfinished::module_id,
            modules_unfinished::credits,
            modules_unfinished::content,
            modules_unfinished::done,
        ))
        .get_result::<Module>(&mut connection)
        .await?;

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

    println!("{:?}", paths);

    Ok(Json(ModuleResponse {
        module: result,
        path: paths,
    }))
}
