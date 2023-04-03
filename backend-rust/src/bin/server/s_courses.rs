// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::WithTucanUrl;
use tucant::MyError;

use axum::extract::State;

use axum::Json;

use tucant::models::TucanSession;
use tucant::models::VVMenuItem;
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
) -> Result<Json<WithTucanUrl<(VVMenuItem, Vec<VVMenuItem>, Vec<tucant::models::Course>)>>, MyError>
{
    let tucan = tucan.continue_session(session.clone()).as_unauthenticated();

    let value = match input.0 {
        None => tucan.vv_root().await?,
        Some(ref input) => {
            tucan
                .vv(Action {
                    magic: input.clone(),
                })
                .await?
        }
    };

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
