use std::str::FromStr;

use crate::{
    TucanConnector, authenticated_retryable_get,
    common::head::{footer, html_head, logged_in_head, logged_out_head},
};
use data_encoding::BASE64URL_NOPAD;
use html_handler::{MyElementRef, MyNode, Root, parse_document};
use itertools::Itertools;
use log::info;
use scraper::CaseSensitivity;
use sha3::{Digest, Sha3_256};
use time::{Duration, OffsetDateTime};
use tucant_types::{
    InstructorImage, LoginResponse, RevalidationStrategy, SemesterId, Semesterauswahl, TucanError,
    coursedetails::{CourseAnmeldefrist, CourseDetailsRequest, CourseDetailsResponse, CourseUebungsGruppe, InstructorImageWithLink, Room, Termin},
};

pub async fn student_result(tucan: &TucanConnector, login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy, request: String) -> Result<CourseDetailsResponse, TucanError> {
    let key = format!("unparsed_student_result.{}", request);

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(&key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            info!("{}", OffsetDateTime::now_utc() - *date);
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age) {
                return student_result_internal(login_response, content);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N{:015},-N000316,{}", login_response.id, request);
    let (content, date) = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = student_result_internal(login_response, &content)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
    }

    tucan.database.put(&key, (content, date)).await;

    Ok(result)
}

fn h(input: &str) -> String {
    BASE64URL_NOPAD.encode(&Sha3_256::digest(input))
}

#[expect(clippy::similar_names, clippy::too_many_lines, clippy::cognitive_complexity)]
fn student_result_internal(login_response: &LoginResponse, content: &str) -> Result<CourseDetailsResponse, TucanError> {
    let document = parse_document(content);
    let html_handler = Root::new(document.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
            <head>
                use html_head(html_handler)?;
                <style type="text/css">
                    "hmeJiQNKqsf_yG6nmm6z0mPHuZmNXFlumNxu52NwnGY"
                </style>
                <style type="text/css">
                    "UfX2y86jUlyyH7pO6ezOiKqPWGK7fDWbsbA23Rfw7Ak"
                </style>
            </head>
            <body class="students_results">
                use if login_response.id == 1 { logged_out_head(html_handler, 311).0 } else { logged_in_head(html_handler, login_response.id).0 };
                <script type="text/javascript">
                </script>
                <h1>
                    leistungsspiegel_von
                </h1>
                <div class="tb">
                    <form id="students_results" action="/scripts/mgrqispi.dll" method="post">
                        <div>
                            <div class="tbhead">
                                selected_course_of_study
                            </div>
                            <div class="tbcontrol">
                                <div class="inputFieldLabel">
                                    <label for="study">
                                        "Studium:"
                                    </label>
                                    <select name="study" id="study" onchange="reloadpage.submitForm(this.form.id);" class="tabledata pageElementLeft">
                                        let semester = while html_handler.peek().is_some() {
                                            let option = if html_handler.peek().unwrap().value().as_element().unwrap().attr("selected").is_some() {
                                                <option value=value selected="selected">
                                                    name
                                                </option>
                                            } => Semesterauswahl { name, value: SemesterId::from_str(&value).unwrap(), selected: true } else {
                                                <option value=value>
                                                    name
                                                </option>
                                            } => Semesterauswahl { name, value: SemesterId::from_str(&value).unwrap(), selected: false };
                                        } => option.either_into::<Semesterauswahl>();
                                    </select>
                                    <input id="Refresh" name="Refresh" type="submit" value="Aktualisieren" class="img img_arrowReload pageElementLeft update"></input>
                                </div>
                            </div>
                            <input name="APPNAME" type="hidden" value="CampusNet"></input>
                            <input name="PRGNAME" type="hidden" value="STUDENT_RESULT"></input>
                            <input name="ARGUMENTS" type="hidden" value="sessionno,menuno,mode, semester,student,study,changestudy,section"></input>
                            <input name="sessionno" type="hidden" value=session_id></input>
                            <input name="menuno" type="hidden" value="000316"></input>
                            <input name="resulttype" type="hidden" value="0"></input>
                            <input name="semester" type="hidden" value="0"></input>
                            <input name="student" type="hidden" value="000000000000000"></input>
                            <input name="changestudy" type="hidden" value="1"></input>
                            <input name="section" type="hidden" value="000000000000000"></input>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
    //let html_handler = footer(html_handler, login_response.id, 311);
    //html_handler.end_document();

    Ok(todo!())
}
