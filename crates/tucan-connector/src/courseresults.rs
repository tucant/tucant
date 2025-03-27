use html_handler::{Root, parse_document};
use tucant_types::{
    LoginResponse, TucanError,
    vv::{ActionRequest, Vorlesungsverzeichnis},
};

use crate::{TucanConnector, authenticated_retryable_get, common::head::html_head};

pub async fn courseresults(tucan: &TucanConnector, login_response: &LoginResponse) -> Result<(), TucanError> {
    let key = format!("unparsed_courseresults");
    let content = if let Some(content) = tucan.database.get(&key).await {
        content
    } else {
        let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N{:015},-N000324,", login_response.id);
        let content = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
        tucan.database.put(&key, &content).await;
        content
    };
    let document = parse_document(&content);
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
    }
    Ok(())
}
