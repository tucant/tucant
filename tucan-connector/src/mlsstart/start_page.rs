use html_extractor::html;
use reqwest::Client;
use scraper::Html;

use crate::{
    common::head::{html_head, html_head_2, page_start},
    html_handler::Root,
    login::LoginResponse,
    TucanError,
};

pub async fn after_login(client: &Client, login_response: LoginResponse) -> Result<(), TucanError> {
    let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N{},-N000019,", login_response.id))
                .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
                .send()
                .await?
                .error_for_status()?;
    println!("{response:#?}");
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    println!("{}", document.root_element().html());
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html!(
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
        <head>_
    );
    let html_handler = html_head(html_handler);
    html!(
        <style type="text/css">
            "XN0jaYaHLeXpiJk0Z7v_FOEBkC5jmdtJaIxqyRqEMj4"
        </style>_
        </head>_
        <body class="currentevents">_
    );
    let html_handler = page_start(html_handler);
    html!(
                            // this is different depending on the page

                     </ul>_
                  </div>_
                  <div id="pageHeadBottom_3" class="pageElementTop">_
                        <div id="pageHeadSwitchLang" class="pageElementRight">_
                            <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CHANGELANGUAGE&ARGUMENTS=-N000000000000002,-N002" class="img img_LangEnglish pageElementLeft" title="English">"English"</a>_
                        </div>_
                        <form name="cn_loginForm" id="cn_loginForm" action="/scripts/mgrqispi.dll" method="post" class="pageElementRight">_
                            <div>_
                                <fieldset id="fieldSet_login">_
                                    <legend>"Anmeldung"</legend>_
                                    <div class="formRow nb">_
                                        <div class="inputFieldLabel">_
                                            <label for="field_user">"TU-ID:"</label>_
                                            <input type="text" id="field_user" name="usrname" size="15" class="login" maxlength="255" accesskey="n" autofocus=""></input>_
                                        </div>_
                                        <div class="inputFieldLabel">_
                                            <label for="field_pass">"Passwort:"</label>_
                                            <input type="password" id="field_pass" name="pass" value="" size="15" class="login" maxlength="255" accesskey="p"></input>_
                                        </div>_
                                    </div>_
                                </fieldset>_
                                <input class="img img_arrowSubmit login_btn" type="submit" id="logIn_btn" value="Anmelden" onclick="return checkform('cn_loginForm','usrname:TU-ID,pass:Passwort','000000000000001');"></input>_
                                <!--"416mrhkWvn83zXJacA3wOy6ZHvHNbAfVlkkb_PMmkEg"-->_
                                <input name="APPNAME" type="hidden" value="CampusNet"></input>_
                                <input name="PRGNAME" type="hidden" value="LOGINCHECK"></input>_
                                <input name="ARGUMENTS" type="hidden" value="clino,usrname,pass,menuno,menu_type,browser,platform"></input>_
                                <input name="clino" type="hidden" value="000000000000001"></input>_
                                <input name="menuno" type="hidden" value="000344"></input>_
                                <input name="menu_type" type="hidden" value="classic"></input>_
                                <input name="browser" type="hidden" value=""></input>_
                                <input name="platform" type="hidden" value=""></input>_
                            </div>_
                        </form>_
                    </div>_
              </div>_
    );
    Ok(())
}
