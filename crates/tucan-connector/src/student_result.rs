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
                        <table class="nb list students_results">
                            <thead>
                                <tr class="tbsubhead">
                                    <th colspan="2">
                                    </th>
                                    <th style="text-align:center;">
                                        "Datum"
                                    </th>
                                    <th style="text-align:right;">
                                        "Credits"
                                    </th>
                                    <th style="text-align:right;">
                                        "Angerechnet"
                                    </th>
                                    <th class="tbsubhead" style="text-align:right;">
                                        "Note"
                                    </th>
                                    <th class="tbsubhead" style="text-align:center;">
                                        "Status"
                                    </th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr class="subhead level00">
                                    <td colspan="2">
                                        course_of_study
                                    </td>
                                    <td style="text-align:center;">
                                    </td>
                                    <td>
                                    </td>
                                    <td>
                                    </td>
                                    <td>
                                    </td>
                                    <td>
                                    </td>
                                </tr>
                                let level1 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level01", CaseSensitivity::CaseSensitive) {
                                    <tr class="subhead level01">
                                        <td colspan="2">
                                            level_1
                                        </td>
                                        <td style="text-align:center;">
                                        </td>
                                        <td>
                                        </td>
                                        <td>
                                        </td>
                                        <td>
                                        </td>
                                        <td>
                                        </td>
                                    </tr>
                                    let level2 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level02", CaseSensitivity::CaseSensitive) {
                                        <tr class="subhead level02">
                                            <td colspan="2">
                                                level_2
                                            </td>
                                            <td style="text-align:center;">
                                            </td>
                                            <td>
                                            </td>
                                            <td>
                                            </td>
                                            <td>
                                            </td>
                                            <td>
                                            </td>
                                        </tr>
                                        let level3 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level03", CaseSensitivity::CaseSensitive) {
                                            <tr class="subhead level03">
                                                <td colspan="2">
                                                    level_3
                                                </td>
                                                <td style="text-align:center;">
                                                </td>
                                                <td>
                                                </td>
                                                <td>
                                                </td>
                                                <td>
                                                </td>
                                                <td>
                                                </td>
                                            </tr>
                                            let level4 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level04", CaseSensitivity::CaseSensitive) {
                                                <tr class="subhead level04">
                                                    <td colspan="2">
                                                        level_4
                                                    </td>
                                                    <td style="text-align:center;">
                                                    </td>
                                                    <td>
                                                    </td>
                                                    <td>
                                                    </td>
                                                    <td>
                                                    </td>
                                                    <td>
                                                    </td>
                                                </tr>
                                                let level5 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level05", CaseSensitivity::CaseSensitive) {
                                                    <tr class="subhead level05">
                                                        <td colspan="2">
                                                            level_5
                                                        </td>
                                                        <td style="text-align:center;">
                                                        </td>
                                                        <td>
                                                        </td>
                                                        <td>
                                                        </td>
                                                        <td>
                                                        </td>
                                                        <td>
                                                        </td>
                                                    </tr>
                                                    let level6 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level06", CaseSensitivity::CaseSensitive) {
                                                        <tr class="subhead level06">
                                                            <td colspan="2">
                                                                level_6
                                                            </td>
                                                            <td style="text-align:center;">
                                                            </td>
                                                            <td>
                                                            </td>
                                                            <td>
                                                            </td>
                                                            <td>
                                                            </td>
                                                            <td>
                                                            </td>
                                                        </tr>
                                                        let entries = while html_handler.peek().unwrap().first_child().unwrap().value().as_element().unwrap().has_class("tbdata", CaseSensitivity::CaseSensitive) {
                                                            <tr>
                                                                <td class="tbdata">
                                                                    id
                                                                </td>
                                                                <td class="tbdata">
                                                                    let name = if html_handler.peek().unwrap().value().is_text() {
                                                                        name
                                                                    } => name else {
                                                                        <a name=_name id=_result_id href=resultdetails_href onclick=_onclick>
                                                                            name
                                                                        </a>
                                                                        <script type="text/javascript">
                                                                            _popup_script
                                                                        </script>
                                                                    } => name;
                                                                </td>
                                                                <td class="tbdata" style="text-align:right;">
                                                                </td>
                                                                <td class="tbdata" style="text-align:right;">
                                                                    let cp = if html_handler.peek().is_some() {
                                                                        cp
                                                                    } => cp;
                                                                </td>
                                                                <td class="tbdata" style="text-align:right;">
                                                                    let used_cp = if html_handler.peek().is_some() {
                                                                        used_cp
                                                                    } => used_cp;
                                                                </td>
                                                                <td class="tbdata" style="text-align:right;">
                                                                    let grade = if html_handler.peek().is_some() {
                                                                        grade
                                                                    } => grade;
                                                                </td>
                                                                <td class="tbdata" style="text-align:center;">
                                                                    <img src=_src alt=_alt title=_title></img>
                                                                </td>
                                                            </tr>
                                                        } => ();
                                                        <tr>
                                                            <td colspan="2" class="level06">
                                                                summe
                                                            </td>
                                                            <td class="level06">
                                                            </td>
                                                            <td class="level06" style="text-align:right;white-space:nowrap;">
                                                                let cp = if html_handler.peek().is_some() {
                                                                    cp
                                                                } => cp;
                                                            </td>
                                                            <td class="level06" style="text-align:right;white-space:nowrap;">
                                                                let used_cp = if html_handler.peek().is_some() {
                                                                    used_cp
                                                                } => used_cp;
                                                            </td>
                                                            <td class="level06" style="text-align:right;">
                                                            </td>
                                                            <td class="level06" style="text-align:center;">
                                                                <img src=pass_or_open alt=bestanden_or_offen title=bestanden_or_offen></img>
                                                            </td>
                                                        </tr>
                                                        <tr>
                                                            <td colspan="   7" class="level06">
                                                                rules
                                                            </td>
                                                        </tr>
                                                    } => ();
                                                    let entries = while html_handler.peek().unwrap().first_child().unwrap().value().as_element().unwrap().has_class("tbdata", CaseSensitivity::CaseSensitive) {
                                                        <tr>
                                                            <td class="tbdata">
                                                                id
                                                            </td>
                                                            <td class="tbdata">
                                                                let name = if html_handler.peek().unwrap().value().is_text() {
                                                                    name
                                                                } => name else {
                                                                    <a name=_name id=_result_id href=resultdetails_href onclick=_onclick>
                                                                        name
                                                                    </a>
                                                                    <script type="text/javascript">
                                                                        _popup_script
                                                                    </script>
                                                                } => name;
                                                            </td>
                                                            <td class="tbdata" style="text-align:right;">
                                                            </td>
                                                            <td class="tbdata" style="text-align:right;">
                                                                let cp = if html_handler.peek().is_some() {
                                                                    cp
                                                                } => cp;
                                                            </td>
                                                            <td class="tbdata" style="text-align:right;">
                                                                let used_cp = if html_handler.peek().is_some() {
                                                                    used_cp
                                                                } => used_cp;
                                                            </td>
                                                            <td class="tbdata" style="text-align:right;">
                                                                let grade = if html_handler.peek().is_some() {
                                                                    grade
                                                                } => grade;
                                                            </td>
                                                            <td class="tbdata" style="text-align:center;">
                                                                <img src=_src alt=_alt title=_title></img>
                                                            </td>
                                                        </tr>
                                                    } => ();
                                                    <tr>
                                                        <td colspan="2" class="level05">
                                                            summe
                                                        </td>
                                                        <td class="level05">
                                                        </td>
                                                        <td class="level05" style="text-align:right;white-space:nowrap;">
                                                            let cp = if html_handler.peek().is_some() {
                                                                cp
                                                            } => cp;
                                                        </td>
                                                        <td class="level05" style="text-align:right;white-space:nowrap;">
                                                            let used_cp = if html_handler.peek().is_some() {
                                                                used_cp
                                                            } => used_cp;
                                                        </td>
                                                        <td class="level05" style="text-align:right;">
                                                        </td>
                                                        <td class="level05" style="text-align:center;">
                                                            <img src=pass_or_open alt=bestanden_or_offen title=bestanden_or_offen></img>
                                                        </td>
                                                    </tr>
                                                    <tr>
                                                        <td colspan="   7" class="level05">
                                                            rules
                                                        </td>
                                                    </tr>
                                                } => ();
                                                let entries = while html_handler.peek().unwrap().first_child().unwrap().value().as_element().unwrap().has_class("tbdata", CaseSensitivity::CaseSensitive) {
                                                    <tr>
                                                        <td class="tbdata">
                                                            id
                                                        </td>
                                                        <td class="tbdata">
                                                            let name = if html_handler.peek().unwrap().value().is_text() {
                                                                name
                                                            } => name else {
                                                                <a name=_name id=_result_id href=resultdetails_href onclick=_onclick>
                                                                    name
                                                                </a>
                                                                <script type="text/javascript">
                                                                    _popup_script
                                                                </script>
                                                            } => name;
                                                        </td>
                                                        <td class="tbdata" style="text-align:right;">
                                                        </td>
                                                        <td class="tbdata" style="text-align:right;">
                                                            let cp = if html_handler.peek().is_some() {
                                                                cp
                                                            } => cp;
                                                        </td>
                                                        <td class="tbdata" style="text-align:right;">
                                                            let used_cp = if html_handler.peek().is_some() {
                                                                used_cp
                                                            } => used_cp;
                                                        </td>
                                                        <td class="tbdata" style="text-align:right;">
                                                            let grade = if html_handler.peek().is_some() {
                                                                grade
                                                            } => grade;
                                                        </td>
                                                        <td class="tbdata" style="text-align:center;">
                                                            <img src=_src alt=_alt title=_title></img>
                                                        </td>
                                                    </tr>
                                                } => ();
                                                <tr>
                                                    <td colspan="2" class="level04">
                                                        summe
                                                    </td>
                                                    <td class="level04">
                                                    </td>
                                                    <td class="level04" style="text-align:right;white-space:nowrap;">
                                                        let cp = if html_handler.peek().is_some() {
                                                            cp
                                                        } => cp;
                                                    </td>
                                                    <td class="level04" style="text-align:right;white-space:nowrap;">
                                                        let used_cp = if html_handler.peek().is_some() {
                                                            used_cp
                                                        } => used_cp;
                                                    </td>
                                                    <td class="level04" style="text-align:right;">
                                                    </td>
                                                    <td class="level04" style="text-align:center;">
                                                        <img src=pass_or_open alt=bestanden_or_offen title=bestanden_or_offen></img>
                                                    </td>
                                                </tr>
                                                <tr>
                                                    <td colspan="   7" class="level04">
                                                        rules
                                                    </td>
                                                </tr>
                                            } => ();
                                            let entries = while html_handler.peek().unwrap().first_child().unwrap().value().as_element().unwrap().has_class("tbdata", CaseSensitivity::CaseSensitive) {
                                                <tr>
                                                    <td class="tbdata">
                                                        id
                                                    </td>
                                                    <td class="tbdata">
                                                        let name = if html_handler.peek().unwrap().value().is_text() {
                                                            name
                                                        } => name else {
                                                            <a name=_name id=_result_id href=resultdetails_href onclick=_onclick>
                                                                name
                                                            </a>
                                                            <script type="text/javascript">
                                                                _popup_script
                                                            </script>
                                                        } => name;
                                                    </td>
                                                    <td class="tbdata" style="text-align:right;">
                                                    </td>
                                                    <td class="tbdata" style="text-align:right;">
                                                        let cp = if html_handler.peek().is_some() {
                                                            cp
                                                        } => cp;
                                                    </td>
                                                    <td class="tbdata" style="text-align:right;">
                                                        let used_cp = if html_handler.peek().is_some() {
                                                            used_cp
                                                        } => used_cp;
                                                    </td>
                                                    <td class="tbdata" style="text-align:right;">
                                                        let grade = if html_handler.peek().is_some() {
                                                            grade
                                                        } => grade;
                                                    </td>
                                                    <td class="tbdata" style="text-align:center;">
                                                        <img src=_src alt=_alt title=_title></img>
                                                    </td>
                                                </tr>
                                            } => ();
                                            <tr>
                                                <td colspan="2" class="level03">
                                                    summe
                                                </td>
                                                <td class="level03">
                                                </td>
                                                <td class="level03" style="text-align:right;white-space:nowrap;">
                                                    let cp = if html_handler.peek().is_some() {
                                                        cp
                                                    } => cp;
                                                </td>
                                                <td class="level03" style="text-align:right;white-space:nowrap;">
                                                    let used_cp = if html_handler.peek().is_some() {
                                                        used_cp
                                                    } => used_cp;
                                                </td>
                                                <td class="level03" style="text-align:right;">
                                                </td>
                                                <td class="level03" style="text-align:center;">
                                                    <img src=pass_or_open alt=bestanden_or_offen title=bestanden_or_offen></img>
                                                </td>
                                            </tr>
                                            <tr>
                                                <td colspan="   7" class="level03">
                                                    rules
                                                </td>
                                            </tr>
                                        } => ();
                                        let entries = while html_handler.peek().unwrap().first_child().unwrap().value().as_element().unwrap().has_class("tbdata", CaseSensitivity::CaseSensitive) {
                                            <tr>
                                                <td class="tbdata">
                                                    id
                                                </td>
                                                <td class="tbdata">
                                                    let name = if html_handler.peek().unwrap().value().is_text() {
                                                        name
                                                    } => name else {
                                                        <a name=_name id=_result_id href=resultdetails_href onclick=_onclick>
                                                            name
                                                        </a>
                                                        <script type="text/javascript">
                                                            _popup_script
                                                        </script>
                                                    } => name;
                                                </td>
                                                <td class="tbdata" style="text-align:right;">
                                                </td>
                                                <td class="tbdata" style="text-align:right;">
                                                    let cp = if html_handler.peek().is_some() {
                                                        cp
                                                    } => cp;
                                                </td>
                                                <td class="tbdata" style="text-align:right;">
                                                    let used_cp = if html_handler.peek().is_some() {
                                                        used_cp
                                                    } => used_cp;
                                                </td>
                                                <td class="tbdata" style="text-align:right;">
                                                    let grade = if html_handler.peek().is_some() {
                                                        grade
                                                    } => grade;
                                                </td>
                                                <td class="tbdata" style="text-align:center;">
                                                    <img src=_src alt=_alt title=_title></img>
                                                </td>
                                            </tr>
                                        } => ();
                                        <tr>
                                            <td colspan="2" class="level02">
                                                summe
                                            </td>
                                            <td class="level02">
                                            </td>
                                            <td class="level02" style="text-align:right;white-space:nowrap;">
                                                let cp = if html_handler.peek().is_some() {
                                                    cp
                                                } => cp;
                                            </td>
                                            <td class="level02" style="text-align:right;white-space:nowrap;">
                                                let used_cp = if html_handler.peek().is_some() {
                                                    used_cp
                                                } => used_cp;
                                            </td>
                                            <td class="level02" style="text-align:right;">
                                            </td>
                                            <td class="level02" style="text-align:center;">
                                                <img src=pass_or_open alt=bestanden_or_offen title=bestanden_or_offen></img>
                                            </td>
                                        </tr>
                                        <tr>
                                            <td colspan="   7" class="level02">
                                                rules
                                            </td>
                                        </tr>
                                    } => ();
                                    let entries = while html_handler.peek().unwrap().first_child().unwrap().value().as_element().unwrap().has_class("tbdata", CaseSensitivity::CaseSensitive) {
                                        <tr>
                                            <td class="tbdata">
                                                id
                                            </td>
                                            <td class="tbdata">
                                                let name = if html_handler.peek().unwrap().value().is_text() {
                                                    name
                                                } => name else {
                                                    <a name=_name id=_result_id href=resultdetails_href onclick=_onclick>
                                                        name
                                                    </a>
                                                    <script type="text/javascript">
                                                        _popup_script
                                                    </script>
                                                } => name;
                                            </td>
                                            <td class="tbdata" style="text-align:right;">
                                            </td>
                                            <td class="tbdata" style="text-align:right;">
                                                let cp = if html_handler.peek().is_some() {
                                                    cp
                                                } => cp;
                                            </td>
                                            <td class="tbdata" style="text-align:right;">
                                                let used_cp = if html_handler.peek().is_some() {
                                                    used_cp
                                                } => used_cp;
                                            </td>
                                            <td class="tbdata" style="text-align:right;">
                                                let grade = if html_handler.peek().is_some() {
                                                    grade
                                                } => grade;
                                            </td>
                                            <td class="tbdata" style="text-align:center;">
                                                <img src=_src alt=_alt title=_title></img>
                                            </td>
                                        </tr>
                                    } => ();
                                    <tr>
                                        <td colspan="2" class="level01">
                                            summe
                                        </td>
                                        <td class="level01">
                                        </td>
                                        <td class="level01" style="text-align:right;white-space:nowrap;">
                                            let cp = if html_handler.peek().is_some() {
                                                cp
                                            } => cp;
                                        </td>
                                        <td class="level01" style="text-align:right;white-space:nowrap;">
                                            let used_cp = if html_handler.peek().is_some() {
                                                used_cp
                                            } => used_cp;
                                        </td>
                                        <td class="level01" style="text-align:right;">
                                        </td>
                                        <td class="level01" style="text-align:center;">
                                            <img src=pass_or_open alt=bestanden_or_offen title=bestanden_or_offen></img>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td colspan="   7" class="level01">
                                            rules
                                        </td>
                                    </tr>
                                } => ();
                                let entries = while html_handler.peek().unwrap().first_child().unwrap().value().as_element().unwrap().has_class("tbdata", CaseSensitivity::CaseSensitive) {
                                    <tr>
                                        <td class="tbdata">
                                            id
                                        </td>
                                        <td class="tbdata">
                                            let name = if html_handler.peek().unwrap().value().is_text() {
                                                name
                                            } => name else {
                                                <a name=_name id=_result_id href=resultdetails_href onclick=_onclick>
                                                    name
                                                </a>
                                                <script type="text/javascript">
                                                    _popup_script
                                                </script>
                                            } => name;
                                        </td>
                                        <td class="tbdata" style="text-align:right;">
                                        </td>
                                        <td class="tbdata" style="text-align:right;">
                                            let cp = if html_handler.peek().is_some() {
                                                cp
                                            } => cp;
                                        </td>
                                        <td class="tbdata" style="text-align:right;">
                                            let used_cp = if html_handler.peek().is_some() {
                                                used_cp
                                            } => used_cp;
                                        </td>
                                        <td class="tbdata" style="text-align:right;">
                                            let grade = if html_handler.peek().is_some() {
                                                grade
                                            } => grade;
                                        </td>
                                        <td class="tbdata" style="text-align:center;">
                                            <img src=_src alt=_alt title=_title></img>
                                        </td>
                                    </tr>
                                } => ();
                                <tr>
                                    <td colspan="2" class="level00">
                                        summe
                                    </td>
                                    <td class="level00">
                                    </td>
                                    <td class="level00" style="text-align:right;white-space:nowrap;">
                                        let cp = if html_handler.peek().is_some() {
                                            cp
                                        } => cp;
                                    </td>
                                    <td class="level00" style="text-align:right;white-space:nowrap;">
                                        let used_cp = if html_handler.peek().is_some() {
                                            used_cp
                                        } => used_cp;
                                    </td>
                                    <td class="level00" style="text-align:right;">
                                    </td>
                                    <td class="level00" style="text-align:center;">
                                        <img src=pass_or_open alt=bestanden_or_offen title=bestanden_or_offen></img>
                                    </td>
                                </tr>
                                <tr>
                                    <td colspan="   7" class="level00">
                                        rules
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                        <table class="nb list students_results">
                            <tbody>
                                <tr>
                                    <th class="tbsubhead" style="text-align:left;">
                                        "Gesamt-GPA"
                                    </th>
                                    <th class="tbsubhead" style="text-align:right;">
                                        grade
                                    </th>
                                </tr>
                                <tr>
                                    <th class="tbsubhead" style="text-align:left;">
                                        "Hauptfach-GPA"
                                    </th>
                                    <th class="tbsubhead" style="text-align:right;">
                                        grade
                                    </th>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
    let html_handler = footer(html_handler, login_response.id, 311);
    html_handler.end_document();

    Ok(todo!())
}
