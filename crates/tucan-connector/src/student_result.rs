use crate::{
    TucanConnector, authenticated_retryable_get,
    common::head::{footer, html_head, logged_in_head, logged_out_head},
};
use html_handler::{InElement, Root, parse_document};
use log::info;
use scraper::CaseSensitivity;
use time::{Duration, OffsetDateTime};
use tucant_types::{
    LoginResponse, RevalidationStrategy, TucanError,
    student_result::{CourseOfStudySelection, StudentResultEntry, StudentResultLevel, StudentResultResponse},
};

/// 0 is the default
pub async fn student_result(tucan: &TucanConnector, login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy, request: u64) -> Result<StudentResultResponse, TucanError> {
    let key = format!("unparsed_student_result.{request}");

    // TODO FIXME this can break as the normal tucan usage will remember which one you selected
    let request = format!("-N0,-N000000000000000,-N000000000000000,-N{request},-N0,-N000000000000000");

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
    println!("{url}");
    let (content, date) = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = student_result_internal(login_response, &content)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
    }

    tucan.database.put(&key, (content, date)).await;

    Ok(result)
}

fn part0<'a, T>(html_handler: InElement<'a, T>, level: &str) -> (InElement<'a, T>, (String, Vec<StudentResultEntry>)) {
    html_extractor::html! {
        <tr class={|l| assert_eq!(l, format!("subhead {level}"))}>
            <td colspan="2">
                level_i
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
                    let name_and_resultdetails_url = if html_handler.peek().unwrap().value().is_text() {
                        name
                    } => (name, None::<String>) else {
                        <a name=_name id=_result_id href=resultdetails_url onclick=_onclick>
                            name
                        </a>
                        <script type="text/javascript">
                            _popup_script
                        </script>
                    } => (name, Some(resultdetails_url));
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
                    <img src=_src alt=_alt title=state></img>
                </td>
            </tr>
        } => StudentResultEntry {
            id,
            name: name_and_resultdetails_url.clone().either_into::<(String, Option<String>)>().0,
            resultdetails_url: name_and_resultdetails_url.either_into::<(String, Option<String>)>().1,
            cp,
            used_cp,
            grade,
            state
        };
    }
    (html_handler, (level_i, entries))
}

fn part1<'a, T>(html_handler: InElement<'a, T>, level: &str, name: (String, Vec<StudentResultEntry>), children: Vec<StudentResultLevel>) -> (InElement<'a, T>, StudentResultLevel) {
    html_extractor::html! {
        let optional = if html_handler.peek().unwrap().value().as_element().unwrap().attrs.is_empty() {
            <tr>
                <td colspan="2" class={|v| assert_eq!(v, level)}>
                    _summe
                </td>
                let sum_cp_and_used_cp = if html_handler.peek().unwrap().value().as_element().unwrap().attr("colspan").is_some() {
                    <td colspan="4" class={|v| assert_eq!(v, level)} style="text-align:left;white-space:nowrap;">
                        _summe_wird_erst_berechnet_wenn_der_bereich_abgeschlossen_ist
                    </td>
                } => (None, None) else {
                    <td class={|v| assert_eq!(v, level)}>
                    </td>
                    <td class={|v| assert_eq!(v, level)} style="text-align:right;white-space:nowrap;">
                        let sum_cp = if html_handler.peek().is_some() {
                            sum_cp
                        } => sum_cp;
                    </td>
                    <td class={|v| assert_eq!(v, level)} style="text-align:right;white-space:nowrap;">
                        let sum_used_cp = if html_handler.peek().is_some() {
                            sum_used_cp
                        } => sum_used_cp;
                    </td>
                    <td class={|v| assert_eq!(v, level)} style="text-align:right;">
                    </td>
                } => (sum_cp, sum_used_cp);
                <td class={|v| assert_eq!(v, level)} style="text-align:center;">
                    <img src=_pass_or_open alt=_bestanden_or_offen title=state></img>
                </td>
            </tr>
            let rules = while html_handler.peek().is_some() && html_handler.peek().unwrap().first_child().unwrap().value().as_element().unwrap().has_class(level, CaseSensitivity::CaseSensitive) {
                <tr>
                    <td colspan="   7" class={|v| assert_eq!(v, level)}>
                        rules
                    </td>
                </tr>
            } => rules;
        } => {
            let (sum_cp, sum_used_cp) = sum_cp_and_used_cp.either_into();
            (sum_cp, sum_used_cp, state, rules)
        };
    }
    (
        html_handler,
        StudentResultLevel {
            name: name.0,
            entries: name.1,
            sum_cp: optional.clone().and_then(|o| o.0),
            sum_used_cp: optional.clone().and_then(|o| o.1),
            state: optional.clone().map(|o| o.2),
            rules: optional.map(|o| o.3).unwrap_or_default(),
            children,
        },
    )
}

#[expect(clippy::too_many_lines)]
fn student_result_internal(login_response: &LoginResponse, content: &str) -> Result<StudentResultResponse, TucanError> {
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
                        _leistungsspiegel_von
                    </h1>
                    <div class="tb">
                        <form id="students_results" action="/scripts/mgrqispi.dll" method="post">
                            <div>
                                <div class="tbhead">
                                    selected_course_of_study
                                </div>
                                let course_of_study = if html_handler.peek().is_some() {
                                    <div class="tbcontrol">
                                        <div class="inputFieldLabel">
                                            <label for="study">
                                                "Studium:"
                                            </label>
                                            <select name="study" id="study" onchange="reloadpage.submitForm(this.form.id);" class="tabledata pageElementLeft">
                                                let course_of_study = while html_handler.peek().is_some() {
                                                    let course_of_study = if html_handler.peek().unwrap().value().as_element().unwrap().attr("selected").is_some() {
                                                        <option value=value selected="selected">
                                                            name
                                                        </option>
                                                    } => CourseOfStudySelection { name, value, selected: true } else {
                                                        <option value=value>
                                                            name
                                                        </option>
                                                    } => CourseOfStudySelection { name, value, selected: false };
                                                } => course_of_study.either_into::<CourseOfStudySelection>();
                                            </select>
                                            <input id="Refresh" name="Refresh" type="submit" value="Aktualisieren" class="img img_arrowReload pageElementLeft update"></input>
                                        </div>
                                    </div>
                                    <input name="APPNAME" type="hidden" value="CampusNet"></input>
                                    <input name="PRGNAME" type="hidden" value="STUDENT_RESULT"></input>
                                    <input name="ARGUMENTS" type="hidden" value="sessionno,menuno,mode, semester,student,study,changestudy,section"></input>
                                    <input name="sessionno" type="hidden" value=_session_id></input>
                                    <input name="menuno" type="hidden" value="000316"></input>
                                    <input name="resulttype" type="hidden" value="0"></input>
                                    <input name="semester" type="hidden" value="0"></input>
                                    <input name="student" type="hidden" value="000000000000000"></input>
                                    <input name="changestudy" type="hidden" value="1"></input>
                                    <input name="section" type="hidden" value="000000000000000"></input>
                                } => course_of_study;
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
                                let level0_title = part0(html_handler, "level00");
                                let level1 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level01", CaseSensitivity::CaseSensitive) {
                                    let level1_title = part0(html_handler, "level01");
                                    let level2 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level02", CaseSensitivity::CaseSensitive) {
                                        let level2_title = part0(html_handler, "level02");
                                        let level3 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level03", CaseSensitivity::CaseSensitive) {
                                            let level3_title = part0(html_handler, "level03");
                                            let level4 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level04", CaseSensitivity::CaseSensitive) {
                                                let level4_title = part0(html_handler, "level04");
                                                let level5 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level05", CaseSensitivity::CaseSensitive) {
                                                    let level5_title = part0(html_handler, "level05");
                                                    let level6 = while html_handler.peek().unwrap().value().as_element().unwrap().has_class("level06", CaseSensitivity::CaseSensitive) {
                                                        let level6_title = part0(html_handler, "level06");
                                                        let level6_contents = part1(html_handler, "level06", level6_title, Vec::new());
                                                    } => level6_contents;
                                                    let level5_contents = part1(html_handler, "level05", level5_title, level6);
                                                } => level5_contents;
                                                let level4_contents = part1(html_handler, "level04", level4_title, level5);
                                            } => level4_contents;
                                            let level3_contents = part1(html_handler, "level03", level3_title, level4);
                                        } => level3_contents;
                                        let level2_contents = part1(html_handler, "level02", level2_title, level3);
                                    } => level2_contents;
                                    let level1_contents = part1(html_handler, "level01", level1_title, level2);
                                } => level1_contents;
                                let level0_contents = part1(html_handler, "level00", level0_title, level1);
                            </tbody>
                        </table>
                        <table class="nb list students_results">
                            <tbody>
                                <tr>
                                    <th class="tbsubhead" style="text-align:left;">
                                        "Gesamt-GPA"
                                    </th>
                                    <th class="tbsubhead" style="text-align:right;">
                                        total_gpa
                                    </th>
                                </tr>
                                <tr>
                                    <th class="tbsubhead" style="text-align:left;">
                                        "Hauptfach-GPA"
                                    </th>
                                    <th class="tbsubhead" style="text-align:right;">
                                        main_gpa
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

    Ok(StudentResultResponse {
        course_of_study: course_of_study.unwrap_or_else(|| vec![CourseOfStudySelection { name: selected_course_of_study, selected: true, value: "default".to_owned() }]),
        level0: level0_contents,
        total_gpa,
        main_gpa,
    })
}
