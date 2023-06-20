// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::AppState;
use crate::WithTucanUrl;
use tucant_core::MyError;

use axum::extract::State;
use axum::Json;
use tucant_core::models::Exam;
use tucant_core::models::MaybeCompleteCourse;
use tucant_core::models::MaybeCompleteModule;
use tucant_core::models::TucanSession;
use tucant_core::tucan::Tucan;
use tucant_core::url::Myexams;
use tucant_core::url::Semester;
use tucant_core::url::TucanProgram;
use tucant_derive::ts;

#[ts]
#[axum::debug_handler(state=AppState)]
pub async fn my_exams(
    session: TucanSession,
    tucan: State<Tucan>,
    input: Json<Option<u64>>,
) -> Result<
    Json<
        WithTucanUrl<(
            Vec<(MaybeCompleteModule, Exam)>,
            Vec<(MaybeCompleteCourse, Exam)>,
        )>,
    >,
    MyError,
> {
    let tucan = tucan.continue_session(session.clone()).await?;

    let result = tucan.my_exams_semester(input.0.into()).await?;

    Ok(Json(WithTucanUrl {
        tucan_url: Into::<TucanProgram>::into(Myexams {
            semester: Semester::CurrentSemester,
        })
        .to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: result,
    }))
}
