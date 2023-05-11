// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::AppState;
use crate::WithTucanUrl;
use tucant_core::MyError;

use axum::extract::State;
use axum::Json;
use tucant_core::models::MaybeCompleteModule;
use tucant_core::models::TucanSession;
use tucant_core::tucan::Tucan;
use tucant_core::url::Mymodules;
use tucant_core::url::TucanProgram;
use tucant_derive::ts;

#[ts]
#[axum::debug_handler(state=AppState)]
pub async fn my_modules(
    session: TucanSession,
    tucan: State<Tucan>,
    _input: Json<()>,
) -> Result<Json<WithTucanUrl<Vec<MaybeCompleteModule>>>, MyError> {
    let tucan = tucan.continue_session(session.clone()).await?;

    let result = tucan.my_modules().await?;

    Ok(Json(WithTucanUrl {
        tucan_url: Into::<TucanProgram>::into(Mymodules)
            .to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: result,
    }))
}
