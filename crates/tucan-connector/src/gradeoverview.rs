use log::info;
use time::{Duration, OffsetDateTime};
use tucant_types::{
    coursedetails::CourseDetailsRequest, gradeoverview::GradeOverviewRequest, mlsstart::{MlsStart, Nachricht, StundenplanEintrag}, LoginResponse, RevalidationStrategy
};

use crate::{
    COURSEDETAILS_REGEX, TucanConnector, TucanError, authenticated_retryable_get,
    head::{footer, html_head, logged_in_head},
};
use html_handler::{MyElementRef, MyNode, Root, parse_document};

// ARGUMENTS=-N352196045346277,-N000325,-AEXEV,-N391263798646423,-N0,-N,-N,-A,-N,-A,-N,-N,-N0,-N391263798681424
//                                              exam-details-id                                 some-id


// PRGNAME=GRADEOVERVIEW&ARGUMENTS=-N700694270951401,-N000325,-AEXEV,-N391263798646423,-N0,-N,-N000000015166000,-A,-N,-A,-N,-N,-N2,-N391263798681424
//                                                                                                                             full site?

pub async fn gradeoverview(tucan: &TucanConnector, login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy, request: GradeOverviewRequest) -> Result<MlsStart, TucanError> {
    let key = format!("unparsed_gradeoverview.{}", login_response.id);

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(&key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            info!("{}", OffsetDateTime::now_utc() - *date);
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age) {
                return gradeoverview_internal(login_response, content);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=GRADEOVERVIEW&ARGUMENTS=-N{},-N000325,{request}", login_response.id);
    let (content, date) = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = gradeoverview_internal(login_response, &content)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
        // TODO FIXME don't remove from database to be able to do recursive invalidations. maybe set age to oldest possible value? or more complex set invalidated and then queries can allow to return invalidated. I think we should do the more complex thing.
    }

    tucan.database.put(&key, (content, date)).await;

    Ok(result)
}

#[expect(clippy::too_many_lines)]
fn gradeoverview_internal(login_response: &LoginResponse, content: &str) -> Result<MlsStart, TucanError> {
    let document = parse_document(content);
    let html_handler = Root::new(document.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
                <head>
                    use html_head(html_handler)?;
                    <style type="text/css">
                        "gpvwXW4pD3VWGZ6fZ_lq3YrpGn430u3_UuuzX97r2rg"
                    </style>
                </head>
                <body class="currentevents">
                    let head = logged_in_head(html_handler, login_response.id);
                    <script type="text/javascript">
                    </script>
                    <h1>
                        _welcome_message
                    </h1>
                    <h2>
                    </h2>
                    <h2>
                        _text
                    </h2>
                    <div class="tb rw-table">
                        <div class="tbhead">
                            "Heutige Veranstaltungen:"
                        </div>
                        <div class="tbcontrol">
                            <a href=_ class="img" name="schedulerLink">
                                "Stundenplan"
                            </a>
                        </div>
                        let stundenplan = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "table" {
                            <table class="nb rw-table" summary="Studium Generale">
                                <tbody>
                                    <tr class="tbsubhead">
                                        <th id="Veranstaltung">
                                            "Veranstaltung"
                                        </th>
                                        <th id="Name">
                                            "Name"
                                        </th>
                                        <th id="von">
                                            "von"
                                        </th>
                                        <th id="bis">
                                            "bis"
                                        </th>
                                    </tr>
                                    let stundenplan = while html_handler.peek().is_some() {
                                        <tr class="tbdata">
                                            <td headers="Veranstaltung">
                                                "Kurse"
                                            </td>
                                            <td headers="Name">
                                                <a class="link" href=coursedetails_url name="eventLink">
                                                    course_name
                                                </a>
                                            </td>
                                            <td headers="von">
                                                <a class="link" href=courseprep_url>
                                                    from
                                                </a>
                                            </td>
                                            <td headers="bis">
                                                <a class="link" href={|v| assert_eq!(v, courseprep_url)}>
                                                    to
                                                </a>
                                            </td>
                                        </tr>
                                    } => StundenplanEintrag {
                                        course_name,
                                        coursedetails_url: CourseDetailsRequest::parse(&COURSEDETAILS_REGEX.replace(&coursedetails_url, "")),
                                        courseprep_url,
                                        from,
                                        to
                                    };
                                </tbody>
                            </table>
                        } => stundenplan else {
                            <div class="tbsubhead">
                                "Für heute sind keine Termine angesetzt!"
                            </div>
                        } => Vec::<StundenplanEintrag>::new();
                    </div>
                    <div class="tb rw-table">
                        <div class="tbhead">
                            "Eingegangene Nachrichten:"
                        </div>
                        <div class="tbcontrol">
                            <a href=_archive class="img">
                                "Archiv"
                            </a>
                        </div>
                        let messages = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "table" {
                            <table class="nb rw-table rw-all" summary="Eingegangene Nachrichten">
                                <tbody>
                                    <tr class="tbsubhead rw-hide">
                                        <th id="Datum">
                                            "Datum"
                                        </th>
                                        <th id="Uhrzeit">
                                            "Uhrzeit"
                                        </th>
                                        <th id="Absender">
                                            "Absender"
                                        </th>
                                        <th id="Betreff">
                                            "Betreff"
                                        </th>
                                        <th id="Aktion">
                                            "Aktion"
                                        </th>
                                    </tr>
                                    let messages = while html_handler.peek().is_some() {
                                        <tr class="tbdata">
                                            <td headers="Datum" class="rw rw-maildate">
                                                <a class="link" href=url>
                                                    date
                                                </a>
                                            </td>
                                            <td headers="Uhrzeit" class="rw rw-mailtime">
                                                <a class="link" href={|u| assert_eq!(url, u)}>
                                                    hour
                                                </a>
                                            </td>
                                            <td headers="Absender" class="rw rw-mailpers">
                                                <a class="link" href={|u| assert_eq!(url, u)}>
                                                    source
                                                </a>
                                            </td>
                                            <td headers="Betreff" class="rw rw-mailsubject">
                                                <a class="link" href={|u| assert_eq!(url, u)}>
                                                    let message = html_handler.next_any_child();
                                                </a>
                                            </td>
                                            <td headers="Aktion" class="rw rw-maildel">
                                                <a class="link" href=delete_url>
                                                    "Löschen"
                                                </a>
                                            </td>
                                        </tr>
                                    } => Nachricht {
                                        url,
                                        date,
                                        hour,
                                        source,
                                        message: match message.value() {
                                            MyNode::Text(text) => text.to_string(),
                                            MyNode::Element(_element) => MyElementRef::wrap(message).unwrap().html(),
                                            _ => panic!(),
                                        },
                                        delete_url
                                    };
                                </tbody>
                            </table>
                        } => messages else {
                            <div class="tbsubhead">
                                "Sie haben keine neuen Nachrichten!"
                            </div>
                        } => Vec::<Nachricht>::new();
                    </div>
                </div>
            </div>
        </div>
    };
    let html_handler = footer(html_handler, login_response.id, 19);
    html_handler.end_document();
    Ok(MlsStart { logged_in_head: head, stundenplan: stundenplan.either_into(), messages: messages.either_into() })
}
