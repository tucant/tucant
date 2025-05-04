// https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEPREP&ARGUMENTS=-N503526614753137,-N000268,-N0,-N391553606081258,-ACODA,-N393006535520104

use std::sync::LazyLock;

// just use the url of another course and change the last number to the last number in the coursedetails url
// seems like there is access control. if you are not in a course it does not work. though you could easily register and unregister again
use crate::{
    TucanConnector, authenticated_retryable_get,
    common::head::{html_head, logged_in_head, logged_out_head},
};
use html_handler::{Root, parse_document};
use log::info;
use regex::Regex;
use scraper::CaseSensitivity;
use time::{Duration, OffsetDateTime};
use tucant_types::{LoginResponse, RevalidationStrategy, TucanError, courseprep::CoursePrepRequest};

/// 04.2025
pub async fn month(tucan: &TucanConnector, login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy, request: String) -> Result<Vec<(String, CoursePrepRequest)>, TucanError> {
    println!("{request}");
    let key = format!("unparsed_month.{request}");

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(&key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            info!("{}", OffsetDateTime::now_utc() - *date);
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age) {
                return month_internal(login_response, content);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MONTH&ARGUMENTS=-N{:015},-N000271,-A01.{},-A,-N000000000000000", login_response.id, request);
    let (content, date) = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = month_internal(login_response, &content)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
    }

    tucan.database.put(&key, (content, date)).await;

    Ok(result)
}

#[expect(clippy::too_many_lines)]
fn month_internal(login_response: &LoginResponse, content: &str) -> Result<Vec<(String, CoursePrepRequest)>, TucanError> {
    static COURSEPREP_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^/scripts/mgrqispi.dll\\?APPNAME=CampusNet&PRGNAME=COURSEPREP&ARGUMENTS=-N\\d+,-N000271,").unwrap());

    let document = parse_document(content);
    //println!("{}", html(&document));
    let html_handler = Root::new(document.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
            <head>
                use html_head(html_handler)?;
                <style type="text/css">
                    "7eLX2t1Uo0IA6vL9eg7XnOUPvbqtOEwtcyPy3J7000g"
                </style>
                <style type="text/css">
                    "U5m9Zw5CdkJnZlkguH4sD5FpcyzEa_LKMFZ5gZtgC0s"
                </style>
            </head>
            <body class="month">
                use if login_response.id == 1 { logged_out_head(html_handler, 311).0 } else { logged_in_head(html_handler, login_response.id).0 };
                <script type="text/javascript">
                </script>
                <h1>
                    _stundenplan_month_year
                </h1>
                <div class="tb tbMonthContainer" id="tbmonthContainer">
                    <div class="tbhead">
                        _month_year
                    </div>
                    <div class="tbcontrol">
                        <div class="arrow_skipBtn">
                            <a href=_url title=_month class="img img_arrowLeft skipLeft" name="skipBack_btn">
                            </a>
                            <a href=_url class="link">
                                "Heute"
                            </a>
                            <a href=_url title=_month class="img img_arrowRight skipRight" name="skipForward_btn">
                            </a>
                        </div>
                        <a href=_url class="arrow">
                            "Monat"
                        </a>
                        <a href=_url class="arrow workdays-week">
                            "Arbeitswoche"
                        </a>
                        <a href=_url class="arrow">
                            "Woche"
                        </a>
                        <a href=_url class="arrow">
                            "Tag"
                        </a>
                    </div>
                    <table class="nb" summary="Month" id="tbMonth" border="0" cellspacing="0" cellpadding="0">
                        <tbody>
                            <tr>
                                <th id="KW" scope="col">
                                    "KW"
                                </th>
                                <th id="Montag" scope="col" class="monthColHead">
                                    "Montag"
                                </th>
                                <th id="Dienstag" scope="col" class="monthColHead">
                                    "Dienstag"
                                </th>
                                <th id="Mittwoch" scope="col" class="monthColHead">
                                    "Mittwoch"
                                </th>
                                <th id="Donnerstag" scope="col" class="monthColHead">
                                    "Donnerstag"
                                </th>
                                <th id="Freitag" scope="col" class="monthColHead">
                                    "Freitag"
                                </th>
                                <th id="Samstag" scope="col" class="monthColHead">
                                    "Samstag"
                                </th>
                                <th id="Sonntag" scope="col" class="monthColHead">
                                    "Sonntag"
                                </th>
                            </tr>
                            let appointments = while html_handler.peek().is_some() {
                                <tr>
                                    <th class="nb KW_month" scope="row">
                                        <a href=_url>
                                            _number
                                        </a>
                                    </th>
                                    let appointments = while html_handler.peek().is_some() {
                                        <td class="tbMonthDayCell">
                                            let appointments = if html_handler.peek().unwrap().value().as_element().unwrap().has_class("emptyDay", CaseSensitivity::CaseSensitive) {
                                                <div class="tbMonthDay nb emptyDay">
                                                    <img src="/gfx/_default/clear.gif" alt="empty"></img>
                                                </div>
                                            } => Vec::<(String, CoursePrepRequest)>::new() else {
                                                <div class="tbMonthDay" title=_day>
                                                    <div class="tbsubhead">
                                                        <a title=_date href=_url>
                                                            _number
                                                        </a>
                                                    </div>
                                                    let appointments = while html_handler.peek().is_some() {
                                                        <div class="appMonth">
                                                            <a title=name xss="href" href=url class="apmntLink" style="overflow:hidden;">
                                                                _title
                                                            </a>
                                                            let _optional_br = if html_handler.peek().is_some() {
                                                                <br></br>
                                                            } => ();
                                                        </div>
                                                    } => (name, CoursePrepRequest::parse(&COURSEPREP_REGEX.replace(&url, "")));
                                                </div>
                                            } => appointments;
                                        </td>
                                    } => appointments.either_into::<Vec<(String, CoursePrepRequest)>>();
                                </tr>
                            } => appointments.into_iter().flatten().collect::<Vec<_>>();
                        </tbody>
                    </table>
                </div>
    }
    let _ = html_handler;
    Ok(appointments.into_iter().flatten().collect::<Vec<_>>())
}
