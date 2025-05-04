use std::str::FromStr;

use html_handler::{Root, parse_document};
use time::{Duration, OffsetDateTime};
use tucant_types::{
    LoginResponse, RevalidationStrategy, SemesterId, Semesterauswahl, TucanError,
    examresults::{ExamResult, ExamResultsResponse},
};

use crate::{
    TucanConnector, authenticated_retryable_get,
    common::head::{footer, html_head, logged_in_head},
};

pub async fn examresults(tucan: &TucanConnector, login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<ExamResultsResponse, TucanError> {
    let key = format!("unparsed_examresults.{}", semester.inner());

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(&key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age) {
                return examresults_internal(login_response, content);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!(
        "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N{:015},-N000325,{}",
        login_response.id,
        if semester == SemesterId::current() {
            String::new()
        } else if semester == SemesterId::all() {
            "-N999".to_owned()
        } else {
            format!("-N{}", semester.inner())
        }
    );
    let (content, date) = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = examresults_internal(login_response, &content)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
        // TODO FIXME don't remove from database to be able to do recursive invalidations. maybe set age to oldest possible value? or more complex set invalidated and then queries can allow to return invalidated. I think we should do the more complex thing.
    }

    tucan.database.put(&key, (content, date)).await;

    Ok(result)
}

#[expect(clippy::too_many_lines)]
fn examresults_internal(login_response: &LoginResponse, content: &str) -> Result<ExamResultsResponse, TucanError> {
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
                        "rg0QEneqjn5GhiURVuEXyYt07X4xXCeM1lgTu-0-5Ak"
                    </style>
                </head>
                <body class="exam_results">
                    use logged_in_head(html_handler, login_response.id).0;
                    <script type="text/javascript">
                    </script>
                    <h1>
                        _pruefungsergebnisse_semester_fuer_name
                    </h1>
                    <div class="tb rw-table">
                        <form id="semesterchange" action="/scripts/mgrqispi.dll" method="post" class="pageElementTop">
                            <div>
                                <div class="tbhead">
                                    "Prüfungsergebnisse"
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
                                                } => Semesterauswahl { name, value: SemesterId::from_str(&value).unwrap(), selected: true } else {
                                                    <option value=value>
                                                        name
                                                    </option>
                                                } => Semesterauswahl { name, value: SemesterId::from_str(&value).unwrap(), selected: false };
                                            } => option.either_into();
                                        </select>
                                        <input name="Refresh" type="submit" value="Aktualisieren" class="img img_arrowReload"></input>
                                    </div>
                                </div>
                                <input name="APPNAME" type="hidden" value="CampusNet"></input>
                                <input name="PRGNAME" type="hidden" value="EXAMRESULTS"></input>
                                <input name="ARGUMENTS" type="hidden" value="sessionno,menuno,semester"></input>
                                <input name="sessionno" type="hidden" value=_sessionno></input>
                                <input name="menuno" type="hidden" value="000325"></input>
                            </div>
                        </form>
                        <table class="nb list">
                            <thead>
                                <tr class="tbsubhead">
                                    <th>
                                        "Name"
                                    </th>
                                    <th>
                                        "Datum"
                                    </th>
                                    <th>
                                        "Note"
                                    </th>
                                    <th>
                                    </th>
                                    <th>
                                    </th>
                                </tr>
                            </thead>
                            <tbody>
                                let results = while html_handler.peek().is_some() {
                                    <tr class="tbdata">
                                        <td>
                                            id_and_name
                                            <br></br>
                                            exam_type
                                            <br></br>
                                            let _thesis = if html_handler.peek().is_some() {
                                                <br></br>
                                                <b>
                                                    "Thema:"
                                                </b>
                                                _topic
                                                <br></br>
                                                <b>
                                                    _topic_eng
                                                    _topic_second
                                                </b>
                                                <br></br>
                                            } => ();
                                        </td>
                                        <td style="vertical-align:top;">
                                            let date = if html_handler.peek().is_some() {
                                                date
                                            } => date;
                                        </td>
                                        <td style="vertical-align:top;">
                                            grade
                                        </td>
                                        <td style="vertical-align:top;">
                                            let grade_text = if grade != "Noch nicht erbracht" {
                                                grade_text
                                            } => grade_text;
                                        </td>
                                        <td style="vertical-align:top;">
                                            let average_url = if html_handler.peek().is_some() {
                                                let _abgegeben = if html_handler.peek().unwrap().value().is_text() {
                                                        "Abgegeben"
                                                    </td>
                                                    <td style="vertical-align:top;">
                                                } => ();
                                                <a id=_popup_id href=average_url class="link" title="Notenspiegel">
                                                    <b>
                                                        "Ø"
                                                    </b>
                                                </a>
                                                <script type="text/javascript">
                                                    _popup_script
                                                </script>
                                            } => average_url;
                                        </td>
                                    </tr>
                                } => {
                                    let (id, name) = id_and_name.split_once('\n').unwrap();
                                    ExamResult { id: id.trim().to_owned(), name: name.trim().to_owned(), exam_type, date, grade, grade_text, average_url }
                                };
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
        use footer(html_handler, login_response.id, 326);
    }
    html_handler.end_document();
    Ok(ExamResultsResponse { semester, results })
}
