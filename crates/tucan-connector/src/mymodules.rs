use html_handler::{Root, parse_document};
use time::{Duration, OffsetDateTime};
use tucant_types::{
    LoginResponse, RevalidationStrategy, SemesterId, Semesterauswahl, TucanError,
    moduledetails::ModuleDetailsRequest,
    mymodules::{Module, MyModulesResponse},
};

use crate::{
    TucanConnector, authenticated_retryable_get,
    common::head::{footer, html_head, logged_in_head},
    registration::index::MODULEDETAILS_REGEX,
};

pub async fn mymodules(tucan: &TucanConnector, login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<MyModulesResponse, TucanError> {
    let key = format!("unparsed_mymodules.{}", semester.0);

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(&key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age) {
                return mymodules_internal(login_response, content);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N{:015},-N000275,{}", login_response.id, if semester == SemesterId::current() { String::new() } else { format!("-N{}", semester.0) });
    let (content, date) = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = mymodules_internal(login_response, &content)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
        // TODO FIXME don't remove from database to be able to do recursive invalidations. maybe set age to oldest possible value? or more complex set invalidated and then queries can allow to return invalidated. I think we should do the more complex thing.
    }

    tucan.database.put(&key, (content, date)).await;

    Ok(result)
}

#[expect(clippy::too_many_lines)]
fn mymodules_internal(login_response: &LoginResponse, content: &str) -> Result<MyModulesResponse, TucanError> {
    let document = parse_document(content);
    let html_handler = Root::new(document.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
                <head>
                    use html_head(html_handler)?;
                    <style type="text/css">
                        "lbOQfuwTSH1NQfB9sjkC-_xOS0UGzyKBoNNl8bXs_FE"
                    </style>
                    <style type="text/css">
                        "ez4igVpcXJnoZyie_yy-b7wKrGd2q4L-BvRmEYSDi2k"
                    </style>
                </head>
                <body class="mymodules">
                    use logged_in_head(html_handler, login_response.id).0;
                    <script type="text/javascript">
                    </script>
                    <h1>
                        _module_von_name
                    </h1>
                    <div class="tb">
                        <form id="semesterchange" action="/scripts/mgrqispi.dll" method="post" class="pageElementTop">
                            <div>
                                <div class="tbhead">
                                    "Modul"
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
                                            <option value="999">
                                                "<Alle>"
                                            </option>
                                            let semester = while html_handler.peek().is_some() {
                                                let option = if html_handler.peek().unwrap().value().as_element().unwrap().attr("selected").is_some() {
                                                    <option value=value selected="selected">
                                                        name
                                                    </option>
                                                } => Semesterauswahl { name, value: SemesterId(value), selected: true } else {
                                                    <option value=value>
                                                        name
                                                    </option>
                                                } => Semesterauswahl { name, value: SemesterId(value), selected: false };
                                            } => option.either_into();
                                        </select>
                                        <input name="Refresh" type="submit" value="Aktualisieren" class="img img_arrowReload refresh"></input>
                                    </div>
                                </div>
                                <input name="APPNAME" type="hidden" value="CampusNet"></input>
                                <input name="PRGNAME" type="hidden" value="MYMODULES"></input>
                                <input name="ARGUMENTS" type="hidden" value="sessionno,menuno,semester"></input>
                                <input name="sessionno" type="hidden" value=session_id></input>
                                <input name="menuno" type="hidden" value="000275"></input>
                            </div>
                        </form>
                        <table class="nb list rw-table rw-all">
                            <thead>
                                <tr class="tbsubhead rw-hide">
                                    <th>
                                    </th>
                                    <th id="Nr.">
                                        "Nr."
                                    </th>
                                    <th id="Name">
                                        "Name"
                                    </th>
                                    <th id="Modulverantwortliche">
                                        "Modulverantwortliche"
                                    </th>
                                    <th id="Credits">
                                        "Credits"
                                    </th>
                                </tr>
                            </thead>
                            <tbody>
                                let modules = while html_handler.peek().is_some() {
                                    <tr class="tbdata ">
                                        <td class="rw rw-mod-logo">
                                        </td>
                                        <td headers="Nr." class="rw rw-mod-no">
                                            module_nr
                                        </td>
                                        <td headers="Name" class="rw rw-mod-name">
                                            <a class="link" href=moduledetails_url>
                                                module_title
                                            </a>
                                        </td>
                                        <td headers="Modulverantwortliche" class="rw rw-mod-prof">
                                            lecturer
                                        </td>
                                        <td headers="Credits" class="rw rw-mod-credits" style="text-align:left">
                                            credits
                                        </td>
                                    </tr>
                                } => Module {
                                    nr: module_nr,
                                    title: module_title,
                                    url: ModuleDetailsRequest::parse(MODULEDETAILS_REGEX.replace(&moduledetails_url, "").split_once(",-A").unwrap().0),
                                    lecturer,
                                    credits
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
    semester.insert(0, Semesterauswahl { name: "<Alle>".to_owned(), value: SemesterId("all".to_owned()), selected: false });
    Ok(MyModulesResponse { semester, modules })
}
