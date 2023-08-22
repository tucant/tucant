// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::AppState;
use crate::WithTucanUrl;

use axum::extract::State;
use axum::Json;

use tucant_core::models::Exam;
use tucant_core::models::MaybeCompleteCourse;
use tucant_core::models::MaybeCompleteModule;
use tucant_core::models::TucanSession;

use base64::prelude::*;
use tucant_core::tucan::Tucan;
use tucant_core::url::Examdetails;
use tucant_core::url::TucanProgram;
use tucant_core::MyError;
use tucant_derive::ts;

#[ts]
#[axum::debug_handler(state=AppState)]
pub async fn exam(
    session: TucanSession,
    tucan: State<Tucan>,
    input: Json<String>,
) -> Result<Json<WithTucanUrl<(Exam, Vec<MaybeCompleteModule>, Vec<MaybeCompleteCourse>)>>, MyError>
{
    let binary_path = BASE64_URL_SAFE_NO_PAD.decode(input.as_bytes()).unwrap();

    let tucan = tucan.continue_session(session.clone()).await?;

    let url = Examdetails {
        id: binary_path.clone(),
    };

    let result = tucan.exam_details(url.clone()).await?;

    Ok(Json(WithTucanUrl {
        tucan_url: Into::<TucanProgram>::into(url).to_tucan_url(Some(session.session_nr)),
        inner: result,
    }))
}
