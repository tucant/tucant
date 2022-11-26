// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::MyError;
use crate::WithTucanUrl;

use tucant::models::Course;
use tucant::models::TucanSession;
use tucant::tucan::Tucan;
use tucant::url::Profcourses;
use tucant::url::TucanProgram;
use tucant_derive::ts;

#[ts]
pub async fn my_courses(
    session: TucanSession,
    tucan: Data<Tucan>,
    _input: Json<()>,
) -> Result<Json<WithTucanUrl<Vec<Course>>>, MyError> {
    let tucan = tucan.continue_session(session.clone()).await.unwrap();

    let result = tucan.my_courses().await?;

    Ok(Json(WithTucanUrl {
        tucan_url: Into::<TucanProgram>::into(Profcourses)
            .to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: result,
    }))
}
