use html_handler::{Root, parse_document};
use time::{Duration, OffsetDateTime};
use tucant_types::{
    LoginResponse, RevalidationStrategy, TucanError,
    mydocuments::{Document, MyDocumentsResponse},
};

use crate::{
    TucanConnector, authenticated_retryable_get,
    head::{footer, html_head, logged_in_head},
};

pub async fn my_documents(tucan: &TucanConnector, login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy) -> Result<MyDocumentsResponse, TucanError> {
    let key = "unparsed_mydocuments";

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age) {
                return my_documents_internal(login_response, content);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N{:015},-N000557,", login_response.id);
    let (content, date) = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = my_documents_internal(login_response, &content)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
        // TODO FIXME don't remove from database to be able to do recursive invalidations. maybe set age to oldest possible value? or more complex set invalidated and then queries can allow to return invalidated. I think we should do the more complex thing.
    }

    tucan.database.put(key, (content, date)).await;

    Ok(result)
}

fn my_documents_internal(login_response: &LoginResponse, content: &str) -> Result<MyDocumentsResponse, TucanError> {
    let document = parse_document(content);
    let html_handler = Root::new(document.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
                <head>
                    use html_head(html_handler)?;
                    <style type="text/css">
                        "41e2ICphaTflKCWDw0-D6hVTXfSC73XVLf9m4PcOeCc"
                    </style>
                    <style type="text/css">
                        "LV-UhM-gCRhIwiPhsdMIXFVizLpnmQ4ZWr5GVZR9YMU"
                    </style>
                </head>
                <body class="createdocument">
                    use logged_in_head(html_handler, login_response.id).0;
                    <script type="text/javascript">
                    </script>
                    <h1>
                        _dokumente_von_name
                    </h1>
                    <form action="/scripts/mgrqispi.dll" name="form1" id="form1" method="post">
                        <div>
                            <input type="hidden" name="templateid" value="1"></input>
                            <input type="hidden" name="status" value="0"></input>
                            <input type="hidden" name="date_from" value=""></input>
                            <input type="hidden" name="date_to" value=""></input>
                            <table class="tb">
                                <tbody>
                                    <tr>
                                        <td class="tbhead">
                                            "Name"
                                        </td>
                                        <td class="tbhead">
                                            "Datum"
                                        </td>
                                        <td class="tbhead">
                                            "Zeit"
                                        </td>
                                        <td class="tbhead">
                                            "Status"
                                        </td>
                                        <td class="tbhead">
                                        </td>
                                    </tr>
                                    let documents = while html_handler.peek().is_some() {
                                        <tr>
                                            <td class="tbdata">
                                                name
                                            </td>
                                            <td class="tbdata">
                                                date
                                            </td>
                                            <td class="tbdata">
                                                time
                                            </td>
                                            <td class="tbdata">
                                            </td>
                                            <td class="tbdata">
                                                <a class="img download" href=url>
                                                    "Download"
                                                </a>
                                            </td>
                                        </tr>
                                    } => Document { name, date, time, url };
                                </tbody>
                            </table>
                            <input name="APPNAME" type="hidden" value="CampusNet"></input>
                            <input name="PRGNAME" type="hidden" value="CREATEDOCUMENT"></input>
                            <input name="ARGUMENTS" type="hidden" value="sessionno,menuid,mode,templateid,status,date_from,date_to,documentid"></input>
                            <input name="sessionno" type="hidden" value=_session_id></input>
                            <input name="menuid" type="hidden" value="000557"></input>
                            <input name="mode" type="hidden" value="1"></input>
                        </div>
                    </form>
                </div>
            </div>
        </div>
        use footer(html_handler, login_response.id, 326);
    }
    html_handler.end_document();
    Ok(MyDocumentsResponse { documents })
}
