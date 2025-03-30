use tucant_types::{LoginResponse, mlsstart::MlsStart, registration::AnmeldungRequest};
use yew::{Html, Properties, classes, function_component, html};
use yew_router::prelude::Link;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct VorlesungsverzeichnisseProps {
    pub data: Option<MlsStart>,
}

#[function_component(Vorlesungsverzeichnisse)]
pub fn vorlesungsverzeichnisse(VorlesungsverzeichnisseProps { data }: &VorlesungsverzeichnisseProps) -> Html {
    html! {
        { data.iter().flat_map(|v| v.logged_in_head.vv.vvs.iter()).map(|(name, url)| {
            html!{
                <li>
                <a
                    class={classes!("dropdown-item", Some(data.is_none().then_some("disabled")))}
                    href={ format!("https://www.tucan.tu-darmstadt.de{}", url)}
                >
                    { name }
                    if data.is_none() {
                        { " " }
                        <span class="spinner-grow spinner-grow-sm" aria-hidden="true" />
                        <span class="visually-hidden" role="status">{ "Loading..." }</span>
                    }
                </a>
            </li>

            }
        }).collect::<Html>() }
    }
}

#[derive(Properties, PartialEq)]
pub struct NavbarLoggedInProps {
    pub current_session: LoginResponse,
    pub data: Option<MlsStart>,
}

#[function_component(NavbarLoggedIn)]
pub fn navbar_logged_in(NavbarLoggedInProps { current_session, data }: &NavbarLoggedInProps) -> Html {
    html! {
        <>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">{ "Aktuelles" }</a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N{:015},-N000019,", current_session.id)}>{ "Aktuelles" }</a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class={classes!("dropdown-item", Some(data.is_none().then_some("disabled")))} href={data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.messages_url))}>
                            { "Nachrichten" }
                            if data.is_none() {
                                { " " }
                                <span class="spinner-grow spinner-grow-sm" aria-hidden="true" />
                                <span class="visually-hidden" role="status">{ "Loading..." }</span>
                            }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">{ "VV" }</a>
                <ul class="dropdown-menu">
                    <li>
                        <Link<Route> to={data.as_ref().map(|d| Route::Vorlesungsverzeichnis { vv: d.logged_in_head.vorlesungsverzeichnis_url.clone() }).unwrap_or(Route::NotFound)} classes={classes!("dropdown-item", Some(data.is_none().then_some("disabled")))}>
                            { "Vorlesungsverzeichnis" }
                            if data.is_none() {
                                { " " }
                                <span class="spinner-grow spinner-grow-sm" aria-hidden="true" />
                                <span class="visually-hidden" role="status">{ "Loading..." }</span>
                            }
                        </Link<Route>>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class={classes!("dropdown-item", Some(data.is_none().then_some("disabled")))} href={data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.vv.lehrveranstaltungssuche_url))}>
                            { "Lehrveranstaltungssuche" }
                            if data.is_none() {
                                { " " }
                                <span class="spinner-grow spinner-grow-sm" aria-hidden="true" />
                                <span class="visually-hidden" role="status">{ "Loading..." }</span>
                            }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SEARCHROOM&ARGUMENTS=-N588840428781170,-N000387,">{ "Raumsuche" }</a>
                    </li>
                    <Vorlesungsverzeichnisse data={data.clone()} />
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N588840428781170,-N000464,-Avvarchivstart%2Ehtml">{ "Archiv" }</a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">{ "Stundenplan" }</a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{:015},-N000268,-A,-A,-N1", current_session.id)}>{ "Stundenplan" }</a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{:015},-N000269,-A,-A,-N0", current_session.id)}>{ "Tagesansicht" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{:015},-N000270,-A,-A,-N1", current_session.id)}>{ "Wochenansicht" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MONTH&ARGUMENTS=-N{:015},-N000271,-A", current_session.id)}>{ "Monatsansicht" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER_EXPORT&ARGUMENTS=-N{:015},-N000272,", current_session.id)}>{ "Export" }</a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">{ "Veranstaltungen" }</a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000273,-Astudveranst%2Ehtml", current_session.id)}>{ "Veranstaltungen" }</a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N{:015},-N000275,", current_session.id)}>{ "Meine Module" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N{:015},-N000274,", current_session.id)}>{ "Meine Veranstaltungen" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENTCHOICECOURSES&ARGUMENTS=-N{:015},-N000307,", current_session.id)}>{ "Meine Wahlbereiche" }</a>
                    </li>
                    <li>
                        <Link<Route> to={Route::Registration { registration: AnmeldungRequest::default() }} classes="dropdown-item">{ "Anmeldung" }</Link<Route>>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYREGISTRATIONS&ARGUMENTS=-N{:015},-N000308,-N000000000000000", current_session.id)}>{ "Mein aktueller Anmeldestatus" }</a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">{ "Prüfungen" }</a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000280,-Astudpruefungen%2Ehtml", current_session.id)}>{ "Prüfungen" }</a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=-N{:015},-N000318,", current_session.id)}>{ "Meine Prüfungen" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCPCHOICE&ARGUMENTS=-N{:015},-N000389,", current_session.id)}>{ "Mein Prüfungsplan" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000391,-Astudplan%2Ehtml", current_session.id)}>{ "Mein Prüfungsplan - Wichtige Hinweise" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000323,-Astudergebnis%2Ehtml", current_session.id)}>{ "Semesterergebnisse" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N{:015},-N000324,", current_session.id)}>{ "Modulergebnisse" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N{:015},-N000325,", current_session.id)}>{ "Prüfungsergebnisse" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N{:015},-N000316,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000", current_session.id)}>{ "Leistungsspiegel" }</a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">{ "Service" }</a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000337,-Aservice%2Ehtml", current_session.id)}>{ "Service" }</a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PERSADDRESS&ARGUMENTS=-N{:015},-N000339,-A", current_session.id)}>{ "Persönliche Daten" }</a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N{:015},-N000557,", current_session.id)}>{ "Meine Dokumente" }</a>
                    </li>
                    <a class={classes!("dropdown-item", Some(data.is_none().then_some("disabled")))} href={data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.antraege_url))}>
                        { "Anträge" }
                        if data.is_none() {
                            { " " }
                            <span class="spinner-grow spinner-grow-sm" aria-hidden="true" />
                            <span class="visually-hidden" role="status">{ "Loading..." }</span>
                        }
                    </a>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=HOLDINFO&ARGUMENTS=-N{:015},-N000652,", current_session.id)}>{ "Sperren" }</a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">{ "Bewerbung" }</a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000441,-Abewerbung", current_session.id)}>{ "Bewerbung" }</a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <a class={classes!("dropdown-item", Some(data.is_none().then_some("disabled")))} href={data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.meine_bewerbung_url))}>
                        { "Meine Bewerbung" }
                        if data.is_none() {
                            { " " }
                            <span class="spinner-grow spinner-grow-sm" aria-hidden="true" />
                            <span class="visually-hidden" role="status">{ "Loading..." }</span>
                        }
                    </a>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N{:015},-N000444,", current_session.id)}>{ "Meine Dokumente" }</a>
                    </li>
                </ul>
            </li>
            <li class="nav-item">
                <a class="nav-link" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000340,-Ahilfe%2Ehtml", current_session.id)}>{ "Hilfe" }</a>
            </li>
        </>
    }
}
