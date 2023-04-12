// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::AppState;
use crate::WithTucanUrl;

use axum::extract::State;
use axum::Json;

use chrono::Duration;
use icalendar::Alarm;
use icalendar::Calendar;
use icalendar::Class;
use icalendar::Component;
use icalendar::Event;
use icalendar::EventLike;
use icalendar::EventStatus;
use tucant::models::CourseEvent;
use tucant::models::CourseGroup;
use tucant::models::Module;
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
) -> Result<
    Json<
        WithTucanUrl<(
            Course,
            Vec<CourseGroup>,
            Vec<CourseEvent>,
            Vec<Module>,
            String,
        )>,
    >,
    MyError,
> {
    let binary_path = BASE64_URL_SAFE_NO_PAD.decode(input.as_bytes()).unwrap();

    let tucan = tucan.continue_session(session.clone());

    let url = Coursedetails {
        id: binary_path.clone(),
    };

    let result = tucan.course(url.clone()).await?;

    let events = result.2.clone();

    let mut my_calendar = Calendar::new();
    my_calendar.name(&result.0.title);
    for event in events {
        my_calendar.push(
            Event::new()
                .class(Class::Public)
                .status(EventStatus::Confirmed)
                .starts(event.timestamp_start)
                .ends(event.timestamp_end)
                .location(&event.room)
                .alarm(Alarm::display(
                    &format!("{} beginnt gleich", result.0.title),
                    -Duration::minutes(15),
                ))
                .summary(&result.0.title)
                .done(),
        );
    }
    let my_calendar = my_calendar.done();

    Ok(Json(WithTucanUrl {
        tucan_url: Into::<TucanProgram>::into(url)
            .to_tucan_url(Some(session.session_nr.try_into().unwrap())),
        inner: (
            result.0,
            result.1,
            result.2,
            result.3,
            format!("{}", my_calendar),
        ),
    }))
}
