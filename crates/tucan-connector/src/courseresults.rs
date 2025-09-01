use std::str::FromStr;

use html_handler::{Root, parse_document};
use time::{Duration, OffsetDateTime};
use tucant_types::{
    LoginResponse, ModuleGrade, RevalidationStrategy, SemesterId, Semesterauswahl, TucanError,
    courseresults::{ModuleResult, ModuleResultsResponse},
    gradeoverview::GradeOverviewRequest,
};

use crate::{
    TucanConnector, authenticated_retryable_get,
    gradeoverview::GRADEOVERVIEW_REGEX,
    head::{footer, html_head, logged_in_head},
};

pub async fn courseresults(
    tucan: &TucanConnector,
    login_response: &LoginResponse,
    revalidation_strategy: RevalidationStrategy,
    semester: SemesterId,
) -> Result<ModuleResultsResponse, TucanError> {
    let key = format!("unparsed_courseresults.{}", semester.inner());

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(&key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age)
            {
                return courseresults_internal(login_response, content);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!(
        "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N{:015},-N000324,{}",
        login_response.id,
        if semester == SemesterId::current() {
            String::new()
        } else if semester == SemesterId::all() {
            panic!("not supported")
        } else {
            format!("-N{}", semester.inner())
        }
    );
    let (content, date) =
        authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = courseresults_internal(login_response, &content)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
        // TODO FIXME don't remove from database to be able to do recursive
        // invalidations. maybe set age to oldest possible value? or
        // more complex set invalidated and then queries can allow to return
        // invalidated. I think we should do the more complex thing.
    }

    tucan.database.put(&key, (content, date)).await;

    Ok(result)
}

#[expect(clippy::too_many_lines)]
fn courseresults_internal(
    login_response: &LoginResponse,
    content: &str,
) -> Result<ModuleResultsResponse, TucanError> {
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
                        "Fh9QXGwM_sXrM0QKenGB9RZNLE7wpBMHS188Im7J1M4"
                    </style>
                    <style type="text/css">
                        "9_MbCqyCOkddcwSY33R_7gqnd6subPQ-km7hoA0s4xo"
                    </style>
                </head>
                <body class="course_results">
                    use logged_in_head(html_handler, login_response.id).0;
                    <script type="text/javascript">
                    </script>
                    <h1>
                        _modulnoten_semester_fuer_name
                    </h1>
                    <div class="tb">
                        <form id="semesterchange" action="/scripts/mgrqispi.dll" method="post" class="pageElementTop">
                            <div>
                                <div class="tbhead">
                                </div>
                                <div class="tbsubhead">
                                    "Wählen Sie ein Semester"
                                </div>
                                <div class="formRow">
                                    <div class="inputFieldLabel long">
                                        <label for="semester">
                                            "Semester:"
                                        </label>
                                        <select id="semester" name="semester" onchange=_onchange class="tabledata">
                                            let semester = while html_handler.peek().is_some() {
                                                let option = if html_handler.peek().unwrap().value().as_element().unwrap().attr("selected").is_some() {
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
                                <input name="PRGNAME" type="hidden" value="COURSERESULTS"></input>
                                <input name="ARGUMENTS" type="hidden" value="sessionno,menuno,semester"></input>
                                <input name="sessionno" type="hidden" value=_sessionno></input>
                                <input name="menuno" type="hidden" value="000324"></input>
                            </div>
                        </form>
                        <table class="nb list">
                            <thead>
                                <tr>
                                    <td class="tbsubhead">
                                        "Nr."
                                    </td>
                                    <td class="tbsubhead">
                                        "Kursname"
                                    </td>
                                    <td class="tbsubhead">
                                        "Endnote"
                                    </td>
                                    <td class="tbsubhead">
                                        "Credits"
                                    </td>
                                    <td class="tbsubhead">
                                        "Status"
                                    </td>
                                    <td class="tbsubhead" colspan="2">
                                    </td>
                                </tr>
                            </thead>
                            <tbody>
                                let results = while html_handler.peek().unwrap().next_sibling().is_some() {
                                    <tr>
                                        <td class="tbdata">
                                            nr
                                        </td>
                                        <td class="tbdata">
                                            name
                                        </td>
                                        <td class="tbdata_numeric" style="vertical-align:top;">
                                            let grade = if html_handler.peek().is_some() {
                                                grade
                                            } => grade;
                                        </td>
                                        <td class="tbdata_numeric">
                                            credits
                                        </td>
                                        <td class="tbdata">
                                            let status = if html_handler.peek().is_some() {
                                                status
                                            } => status;
                                        </td>
                                        <td class="tbdata" style="vertical-align:top;">
                                            let pruefungen_url = if html_handler.peek().is_some() {
                                                <a id=_some_id href=pruefungen_url>
                                                    "Prüfungen"
                                                </a>
                                                <script type="text/javascript">
                                                    _script
                                                </script>
                                            } => pruefungen_url;
                                        </td>
                                        <td class="tbdata">
                                            let average_url = if html_handler.peek().is_some() {
                                                <a id=_some_id href=average_url class="link" title="Notenspiegel">
                                                    <b>
                                                        "Ø"
                                                    </b>
                                                </a>
                                                <script type="text/javascript">
                                                    _script
                                                </script>
                                            } => GradeOverviewRequest::parse(&GRADEOVERVIEW_REGEX.replace(&average_url, ""));
                                        </td>
                                    </tr>
                                } => ModuleResult {
                                    nr,
                                    name,
                                    grade: ModuleGrade::from((grade.as_deref(), status.as_deref())),
                                    credits,
                                    pruefungen_url,
                                    average_url
                                };
                                <tr>
                                    <th colspan="2">
                                        "Semester-GPA"
                                    </th>
                                    <th class="tbdata">
                                        average_grade
                                    </th>
                                    <th>
                                        sum_credits
                                    </th>
                                    <th class="tbdata" colspan="4">
                                    </th>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
        use footer(html_handler, login_response.id, 326);
    }
    html_handler.end_document();
    Ok(ModuleResultsResponse {
        semester,
        results,
        average_grade,
        sum_credits,
    })
}
