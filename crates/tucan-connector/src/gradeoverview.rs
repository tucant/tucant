use std::sync::LazyLock;

use log::info;
use regex::Regex;
use time::{Duration, OffsetDateTime};
use tucan_types::{
    LoginResponse, RevalidationStrategy,
    gradeoverview::{GradeOverviewRequest, GradeOverviewResponse, Grades},
};

use crate::{
    TucanConnector, TucanError, authenticated_retryable_get,
    head::{footer, html_head, logged_in_head},
};
use html_handler::{Root, parse_document};

pub static GRADEOVERVIEW_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        "^/scripts/mgrqispi.dll\\?APPNAME=CampusNet&PRGNAME=GRADEOVERVIEW&ARGUMENTS=-N\\d+,-N\\d+,",
    )
    .unwrap()
});

pub async fn gradeoverview(
    tucan: &TucanConnector,
    login_response: &LoginResponse,
    revalidation_strategy: RevalidationStrategy,
    request: GradeOverviewRequest,
) -> Result<GradeOverviewResponse, TucanError> {
    let key = format!("unparsed_gradeoverview.{request}");

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(&key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            info!("{}", OffsetDateTime::now_utc() - *date);
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age)
            {
                return gradeoverview_internal(login_response, content);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!(
        "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=GRADEOVERVIEW&ARGUMENTS=-N{},-N000325,{request}",
        login_response.id
    );
    let (content, date) =
        authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = gradeoverview_internal(login_response, &content)?;
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
fn gradeoverview_internal(
    login_response: &LoginResponse,
    content: &str,
) -> Result<GradeOverviewResponse, TucanError> {
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
                <body class="students_grades_diagramm_BFW">
                    let _head = logged_in_head(html_handler, login_response.id);
                    <script type="text/javascript">
                    </script>
                    <h1>
                        _notenspiegel
                    </h1>
                    <h2>
                        module_and_semester
                    </h2>
                    let modulangebot = if html_handler
                        .peek()
                        .unwrap()
                        .value()
                        .as_element()
                        .unwrap()
                        .name()
                        == "table" {
                        <table class="tb">
                            <tbody>
                                <tr>
                                    <td class="tbhead">
                                        "Kontext"
                                    </td>
                                </tr>
                                <tr>
                                    <td class="tbdata">
                                        modulangebot
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    } => modulangebot;
                    <h2>
                        let studienleistung = if html_handler
                            .peek()
                            .is_some() {
                            studienleistung
                        } => studienleistung;
                    </h2>
                    <div class="tb">
                        <div class="tbhead">
                        </div>
                        <div class="tbcontrol">
                            <a href=_examresults_url class="img img_arrowLeft prev">
                                "Zurück"
                            </a>
                        </div>
                        let maybe_grades = if html_handler
                            .peek()
                            .unwrap()
                            .value()
                            .as_element()
                            .unwrap()
                            .name()
                            == "table" {
                            <table class="nb">
                                <tbody>
                                    <tr>
                                        <td class="tbsubhead">
                                            "Noten"
                                        </td>
                                        let names = while html_handler.peek().is_some() {
                                            <td class="tbsubhead">
                                                name
                                            </td>
                                        } => name;
                                    </tr>
                                    <tr>
                                        <td class="tbdata">
                                            "Anzahl"
                                        </td>
                                        let values = while html_handler.peek().is_some() {
                                            <td class="tbdata">
                                                value
                                            </td>
                                        } => if value == "---" {
                                            0
                                        } else {
                                            value.parse().expect(&value)
                                        };
                                    </tr>
                                </tbody>
                            </table>
                            let infos = while html_handler.peek().is_some() {
                                <div class="tbdata">
                                    info
                                </div>
                            } => info;
                        } => Grades {
                            columns: names.into_iter().zip(values).collect(),
                            infos
                        } else {
                            <div class="tbdata">
                                "noch nicht gesetzt"
                            </div>
                        } => ();
                    </div>
                </div>
            </div>
        </div>
    };
    let html_handler = footer(html_handler, login_response.id, 19);
    html_handler.end_document();
    Ok(GradeOverviewResponse {
        module_and_semester,
        modulangebot,
        studienleistung,
        maybe_grades: maybe_grades.left(),
    })
}
