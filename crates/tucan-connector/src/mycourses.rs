use std::str::FromStr;

use html_handler::{Root, parse_document};
use scraper::CaseSensitivity;
use time::{Duration, OffsetDateTime};
use tucant_types::{
    LoginResponse, RevalidationStrategy, SemesterId, Semesterauswahl, TucanError,
    coursedetails::CourseDetailsRequest,
    mycourses::{Course, MyCoursesResponse},
};

use crate::{
    COURSEDETAILS_REGEX, TucanConnector, authenticated_retryable_get,
    head::{footer, html_head, logged_in_head},
};

pub async fn mycourses(
    tucan: &TucanConnector,
    login_response: &LoginResponse,
    revalidation_strategy: RevalidationStrategy,
    semester: SemesterId,
) -> Result<MyCoursesResponse, TucanError> {
    let key = format!("unparsed_mycourses.{}", semester.inner());

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(&key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age)
            {
                return mycourses_internal(login_response, content);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!(
        "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N{:015},-N000274,{}",
        login_response.id,
        if semester == SemesterId::current() {
            "-N1337".to_owned() // DO NOT ASK
        } else if semester == SemesterId::all() {
            "-N999".to_owned()
        } else {
            format!("-N{}", semester.inner())
        }
    );
    let (content, date) =
        authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = mycourses_internal(login_response, &content)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
        // TODO FIXME don't remove from database to be able to do recursive invalidations. maybe set age to oldest possible value? or more complex set invalidated and then queries can allow to return invalidated. I think we should do the more complex thing.
    }

    tucan.database.put(&key, (content, date)).await;

    Ok(result)
}

#[expect(clippy::too_many_lines)]
fn mycourses_internal(
    login_response: &LoginResponse,
    content: &str,
) -> Result<MyCoursesResponse, TucanError> {
    // TODO Übertragungsveranstaltung
    let document = parse_document(content);
    let html_handler = Root::new(document.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
                <head>
                    use html_head(html_handler)?;
                    <style type="text/css">
                        "lbOQfuwTSH1NQfB9sjkC-_xOS0UGzyKBoNNl8bXs_FE"
                    </style>
                    <style type="text/css">
                        "vfjJ7t0pGE0RfQdNjo9wRt1ASaup6zKvYVfYYYWBt4I"
                    </style>
                </head>
                <body class="profcourses">
                    use logged_in_head(html_handler, login_response.id).0;
                    <script type="text/javascript">
                    </script>
                    <h1>
                        _veranstaltungen_von_name
                    </h1>
                    <br></br>
                    <div class="tb rw-table">
                        <form id="semesterchange" action="/scripts/mgrqispi.dll" method="post" class="pageElementTop">
                            <div>
                                <div class="tbhead">
                                    "Semesterauswahl"
                                </div>
                                <div class="tbsubhead">
                                    "Wählen Sie ein Semester"
                                </div>
                                <div class="formRow">
                                    <div class="inputFieldLabel">
                                        <label for="semester">
                                            "Semester:"
                                        </label>
                                        <select name="semester" id="semester" onchange=_onchange class="tabledata pageElementLeft">
                                            let semester = while html_handler.peek().is_some() {
                                                let option = if html_handler
                                                    .peek()
                                                    .unwrap()
                                                    .value()
                                                    .as_element()
                                                    .unwrap()
                                                    .attr("selected")
                                                    .is_some() {
                                                    <option value=value selected="selected">
                                                        name
                                                    </option>
                                                } => Semesterauswahl {
                                                    name,
                                                    value: SemesterId::from_str(&value).unwrap(),
                                                    selected: true
                                                } else {
                                                    <option value=value>
                                                        name
                                                    </option>
                                                } => Semesterauswahl {
                                                    name,
                                                    value: SemesterId::from_str(&value).unwrap(),
                                                    selected: false
                                                };
                                            } => option.either_into();
                                        </select>
                                        <input name="Refresh" type="submit" value="Aktualisieren" class="img img_arrowReload"></input>
                                    </div>
                                </div>
                                <input name="APPNAME" type="hidden" value="CampusNet"></input>
                                <input name="PRGNAME" type="hidden" value="PROFCOURSES"></input>
                                <input name="ARGUMENTS" type="hidden" value="sessionno,menuno,semester"></input>
                                <input name="sessionno" type="hidden" value=_session_id></input>
                                <input name="menuno" type="hidden" value="000274"></input>
                            </div>
                        </form>
                    </div>
                    <table class="tb rw-table rw-all">
                        <caption>
                            "Veranstaltungen"
                        </caption>
                        <thead>
                            <tr class="tbsubhead rw-hide">
                                <th scope="col">
                                </th>
                                <th scope="col">
                                    "Nr."
                                </th>
                                <th scope="col">
                                    "Name"
                                </th>
                                <th scope="col">
                                    "Zeitraum"
                                </th>
                                <th scope="col">
                                    "Credits"
                                </th>
                                <th scope="col">
                                    "Standort"
                                </th>
                                <th scope="col">
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            let sections = while html_handler.peek().is_some() {
                                <tr class="tbsubhead">
                                    <th colspan="100%">
                                        title
                                    </th>
                                </tr>
                                let courses = while html_handler.peek().is_some()
                                    && html_handler
                                        .peek()
                                        .unwrap()
                                        .value()
                                        .as_element()
                                        .unwrap()
                                        .has_class("tbdata", CaseSensitivity::CaseSensitive) {
                                    <tr class="tbdata ">
                                        <td class="rw rw-profc-logo">
                                        </td>
                                        <td class="rw rw-profc-courseno">
                                            course_no
                                        </td>
                                        <td class="rw rw-profc-coursename">
                                            <a href=coursedetails_url class="link" name="eventLink">
                                                name
                                            </a>
                                        </td>
                                        <td class="rw rw-profc-daterange">
                                            date_range
                                        </td>
                                        <td class="rw rw-profc-credits">
                                            let credits = if html_handler.peek().is_some() {
                                                credits
                                            } => credits;
                                        </td>
                                        <td class="rw rw-profc-location">
                                            location
                                        </td>
                                        <td class="rw rw-profc-audit">
                                        </td>
                                    </tr>
                                } => Course {
                                    nr: course_no,
                                    title: name,
                                    url: CourseDetailsRequest::parse(
                                        &COURSEDETAILS_REGEX.replace(&coursedetails_url, "")
                                    ),
                                    date_range,
                                    location,
                                    credits
                                };
                            } => (title, courses);
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
        use footer(html_handler, login_response.id, 326);
    }
    html_handler.end_document();
    Ok(MyCoursesResponse { semester, sections })
}
