use html_extractor::html;
use scraper::Html;
use serde::{Deserialize, Serialize};

use crate::{
    common::head::{html_head, logged_in_head},
    html_handler::Root,
    login::LoginResponse,
    registration::index::{AnmeldungRequest, AnmeldungResponse},
    Tucan, TucanError,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleDetailsRequest {
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDetailsResponse {}

pub async fn moduledetails(
    tucan: &Tucan,
    login_response: &LoginResponse,
    args: ModuleDetailsRequest,
) -> Result<ModuleDetailsResponse, TucanError> {
    let id = login_response.id;
    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N{:015}{}", id, args.arguments);
    let response = tucan
        .client
        .get(url)
        .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
        .send()
        .await?
        .error_for_status()?;
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html!(
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
        <head>_
    );
    let mut html_handler = html_head(html_handler);
    if html_handler.peek().is_none() {
        html!(
            </head>_
        <body class="timeout">
        );
        let _html_handler = html_handler;
        return Err(TucanError::Timeout);
    }
    html!(
        <style type="text/css">
            "Z8Nk5s0HqiFiRYeqc3zP-bPxIN31ePraM-bbLg_KfNQ"
        </style>_
        <style type="text/css">
            "3CC0xpJgjHprYY59D1krvfwrI2LSV2-OtaN3CviYnG8"
        </style>_
        </head>_
        <body class="moduledetails">_
    );
    let html_handler = logged_in_head(html_handler, login_response.id);
    html!(
            <!--"-h_LWY1o6IWQvq6DnWxWgp2Zp06F4JZitgy9Jh20j3s"-->_
            <script type="text/javascript"></script>_
            <h1>module_id</h1>_
    <form name="moduleform" id="moduleform" action="/scripts/mgrqispi.dll" method="post">_
    <div class="contentlayoutleft" id="contentlayoutleft">_
        <table class="tb">_
            <caption>"Moduldetails"</caption>_
            <tbody>
                <tr class="tbsubhead">_
                    <td colspan="3">
                        "\n\t\t\t\t\tSie sind angemeldet!\n\t\t\t\t"
                    </td>_
                </tr>_
                <tr class="tbcontrol">_
                    <td>_
                        //<a href=url class="arrow">"Schließen"</a>_
                    </td>_
                </tr>_
                <tr class="tbdata">_
                    <td colspan="3">_
                        <b>"Modulverantwortliche: "</b>_
                        <span id="dozenten">"N.N."</span>_
                        <br></br><br></br>_
                        <b>"Anzeige im Stundenplan: "</b>
                        display_in_timetable
                        <br></br><br></br>_
                        <b>"Dauer: "</b>
                        length
                        <br></br><br></br>_
                        <b>"Anzahl Wahlkurse: "</b>
                        count_elective_courses
                        <br></br><br></br>_
                        <b>"Credits: "</b>
                        credits
                        <br></br>"Hinweis: In Ihrer Prüfungsordnung können abweichende Credits festgelegt sein.\n                                                             "
                        <br></br><br></br>_
                        <b>"Startsemester: "</b>
                        start_semester
                        <br></br><br></br>
                        <!-- "Start participant selection" -->
                        <!-- "End participant selection" -->
                        <!-- "Start Descriptions" -->
    );

    Ok(todo!())
}
