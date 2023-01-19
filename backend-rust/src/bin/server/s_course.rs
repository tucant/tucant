// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::AppState;
use crate::WithTucanUrl;

use axum::extract::State;
use axum::Json;

use tucant::models::CourseEvent;
use tucant::models::CourseGroup;
use tucant::models::TucanSession;

use base64::prelude::*;
use tucant::url::Coursedetails;
use tucant::url::TucanProgram;
use tucant::MyError;
use tucant::{models::Course, tucan::Tucan};
use tucant_derive::ts;

#[ts]
#[axum::debug_handler(state=AppState)]
pub async fn course(
    session: TucanSession,
    tucan: State<Tucan>,
    input: Json<String>,
) -> Result<Json<WithTucanUrl<(Course, Vec<CourseGroup>, Vec<CourseEvent>)>>, MyError> {
    let binary_path = BASE64_URL_SAFE_NO_PAD.decode(input.as_bytes()).unwrap();

    let tucan = tucan.continue_session(session.clone());

    let url = Coursedetails {
        id: binary_path.clone(),
    };

    let result = tucan.course(url.clone()).await?;

    Ok(Json(WithTucanUrl {
        tucan_url: Into::<TucanProgram>::into(url)
            .to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: result,
    }))
}
