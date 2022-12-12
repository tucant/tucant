// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io::ErrorKind;

use crate::AppState;
use crate::WithTucanUrl;

use axum::extract::State;
use axum::Json;

use tucant::models::Exam;
use tucant::models::TucanSession;
use tucant::tucan_user::CourseOrCourseGroup;
use tucant::url::Coursedetails;
use tucant::url::Examdetails;
use tucant::url::TucanProgram;
use tucant::MyError;
use tucant::{models::Course, tucan::Tucan};
use tucant_derive::ts;

#[ts]
#[axum::debug_handler(state=AppState)]
pub async fn exam(
    session: TucanSession,
    tucan: State<Tucan>,
    input: Json<String>,
) -> Result<Json<WithTucanUrl<Exam>>, MyError> {
    let binary_path = base64::decode_config(input.as_bytes(), base64::URL_SAFE_NO_PAD).unwrap();

    let tucan = tucan.continue_session(session.clone()).await.unwrap();

    let url = Examdetails {
        id: binary_path.clone(),
    };

    let result = tucan.exam_details(url.clone()).await?;

    Ok(Json(WithTucanUrl {
        tucan_url: Into::<TucanProgram>::into(url)
            .to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: result,
    }))
}
