// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io::ErrorKind;

use crate::MyError;
use crate::WithTucanUrl;

use tucant::models::TucanSession;
use tucant::tucan_user::CourseOrCourseGroup;
use tucant::url::Coursedetails;
use tucant::url::TucanProgram;
use tucant::{models::Course, tucan::Tucan};
use tucant_derive::ts;

#[ts]
pub async fn course(
    session: TucanSession,
    tucan: Data<Tucan>,
    input: Json<String>,
) -> Result<Json<WithTucanUrl<Course>>, MyError> {
    let binary_path = base64::decode_config(input.as_bytes(), base64::URL_SAFE_NO_PAD).unwrap();

    let tucan = tucan.continue_session(session.clone()).await.unwrap();

    let url = Coursedetails {
        id: binary_path.clone(),
    };

    let result = tucan.course_or_course_group(url.clone()).await?;

    match result {
        CourseOrCourseGroup::Course(result) => Ok(Json(WithTucanUrl {
            tucan_url: Into::<TucanProgram>::into(url)
                .to_tucan_url(Some(session.session_nr.try_into().unwrap())),
            inner: result,
        })),
        CourseOrCourseGroup::CourseGroup(_) => Err(std::io::Error::new(
            ErrorKind::Other,
            "this is a course group, not a course",
        )
        .into()),
    }
}
