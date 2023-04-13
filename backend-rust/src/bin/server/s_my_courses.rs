// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::AppState;
use crate::WithTucanUrl;
use tucant::MyError;

use axum::extract::State;
use axum::Json;

use tucant::models::TucanSession;

use tucant::tucan::Tucan;
use tucant::tucan_user::CourseOrCourseGroup;
use tucant::url::Profcourses;
use tucant::url::TucanProgram;
use tucant_derive::ts;

#[ts]
#[axum::debug_handler(state=AppState)]
pub async fn my_courses(
    session: TucanSession,
    tucan: State<Tucan>,
    _input: Json<()>,
) -> Result<Json<WithTucanUrl<Vec<CourseOrCourseGroup>>>, MyError> {
    let tucan = tucan.continue_session(session.clone()).await?;

    let result = tucan.my_courses().await?;

    Ok(Json(WithTucanUrl {
        tucan_url: Into::<TucanProgram>::into(Profcourses)
            .to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: result,
    }))
}
