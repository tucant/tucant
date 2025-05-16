use crate::Route;
use leptos::prelude::*;
use tucant_types::{LoginResponse, SemesterId, mlsstart::MlsStart, registration::AnmeldungRequest};

#[component]
pub fn Vorlesungsverzeichnisse(data: Option<MlsStart>) -> impl IntoView {
    view! {
        {
            data.iter()
                .flat_map(|v| v.logged_in_head.vv.vvs.iter())
                .map(|(name, url)| {
                    view! {
                        <li>
                            <a
                                href=format!("/vv/{}", url.clone()) class="dropdown-item bg-success-subtle" class:disabled=data.is_none()>
                                name
                                {
                                    if data.is_none() {
                                        view! {
                                                " "
                                            <span class="spinner-grow spinner-grow-sm" aria-hidden="true" />
                                            <span class="visually-hidden" role="status">
                                                { "Loading..." }
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                            </a>
                        </li>
                    }
                })
                .collect::<Vec<_>>()
        }
    }
}

#[component]
pub fn navbar_logged_in(current_session: LoginResponse, data: Option<MlsStart>) -> impl IntoView {
    let loading = if data.is_none() {
        view! {
                " "
            <span class="spinner-grow spinner-grow-sm" aria-hidden="true" />
            <span class="visually-hidden" role="status">
                { "Loading..." }
            </span>
        }
        .into_any()
    } else {
        view! {}.into_any()
    };
    view! {
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { "Aktuelles" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a href="/overview" class="dropdown-item bg-success-subtle">
                            { "Aktuelles" }
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" class:disabled=data.is_none() href={data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.messages_url))}>
                            { "Nachrichten" }
                            loading
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { "VV" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a href=data.as_ref().map(|d| format!("/vv/{}", d.logged_in_head.vorlesungsverzeichnis_url)) class="dropdown-item bg-success-subtle" class:disabled=data.is_none()>
                            { "Vorlesungsverzeichnis" }
                            loading
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" class:disabled=data.is_none() href={data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.vv.lehrveranstaltungssuche_url))}>
                            { "Lehrveranstaltungssuche" }
                            loading
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SEARCHROOM&ARGUMENTS=-N588840428781170,-N000387,">
                            { "Raumsuche" }
                        </a>
                    </li>
                    <Vorlesungsverzeichnisse data={data.clone()} />
                    <li>
                        <a class="dropdown-item" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N588840428781170,-N000464,-Avvarchivstart%2Ehtml">
                            { "Archiv" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { "Stundenplan" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{:015},-N000268,-A,-A,-N1", current_session.id)}>
                            { "Stundenplan" }
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{:015},-N000269,-A,-A,-N0", current_session.id)}>
                            { "Tagesansicht" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{:015},-N000270,-A,-A,-N1", current_session.id)}>
                            { "Wochenansicht" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MONTH&ARGUMENTS=-N{:015},-N000271,-A", current_session.id)}>
                            { "Monatsansicht" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER_EXPORT&ARGUMENTS=-N{:015},-N000272,", current_session.id)}>
                            { "Export" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { "Veranstaltungen" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000273,-Astudveranst%2Ehtml", current_session.id)}>
                            { "Veranstaltungen" }
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a href="/my-semester-modules/current" class="dropdown-item bg-success-subtle">
                            { "Meine Semestermodule" }
                        </a>
                    </li>
                    <li>
                        <a href="/my-modules/current" class="dropdown-item bg-success-subtle">
                            { "Meine Module" }
                        </a>
                    </li>
                    <li>
                        <a href="/my-courses/current" class="dropdown-item bg-success-subtle">
                            { "Meine Veranstaltungen" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENTCHOICECOURSES&ARGUMENTS=-N{:015},-N000307,", current_session.id)}>
                            { "Meine Wahlbereiche" }
                        </a>
                    </li>
                    <li>
                        <a href="/registration/" class="dropdown-item bg-success-subtle">
                            { "Anmeldung" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYREGISTRATIONS&ARGUMENTS=-N{:015},-N000308,-N000000000000000", current_session.id)}>
                            { "Mein aktueller Anmeldestatus" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { "Prüfungen" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000280,-Astudpruefungen%2Ehtml", current_session.id)}>
                            { "Prüfungen" }
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a href="/my-exams/current" class="dropdown-item bg-success-subtle">
                            { "Meine Prüfungen" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCPCHOICE&ARGUMENTS=-N{:015},-N000389,", current_session.id)}>
                            { "Mein Prüfungsplan" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000391,-Astudplan%2Ehtml", current_session.id)}>
                            { "Mein Prüfungsplan - Wichtige Hinweise" }
                        </a>
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000323,-Astudergebnis%2Ehtml", current_session.id)}>
                            { "Semesterergebnisse" }
                        </a>
                    </li>
                    <li>
                        <a href="/course-results/current" class="dropdown-item bg-success-subtle">
                            { "Modulergebnisse" }
                        </a>
                    </li>
                    <li>
                        <a href="/exam-results/current" class="dropdown-item bg-success-subtle">
                            { "Prüfungsergebnisse" }
                        </a>
                    </li>
                    <li>
                        <a href="/student-result/default" class="dropdown-item bg-success-subtle">
                            { "Leistungsspiegel" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { "Service" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000337,-Aservice%2Ehtml", current_session.id)}>
                            { "Service" }
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PERSADDRESS&ARGUMENTS=-N{:015},-N000339,-A", current_session.id)}>
                            { "Persönliche Daten" }
                        </a>
                    </li>
                    <li>
                        <a href="/my-documents" class="dropdown-item bg-success-subtle">
                            { "Meine Dokumente" }
                        </a>
                    </li>
                    <a class="dropdown-item" class:disabled=data.is_none() href={data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.antraege_url))}>
                        { "Anträge" }
                        loading
                    </a>
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=HOLDINFO&ARGUMENTS=-N{:015},-N000652,", current_session.id)}>
                            { "Sperren" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                    { "Bewerbung" }
                </a>
                <ul class="dropdown-menu">
                    <li>
                        <a class="dropdown-item" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000441,-Abewerbung", current_session.id)}>
                            { "Bewerbung" }
                        </a>
                    </li>
                    <li>
                        <hr class="dropdown-divider" />
                    </li>
                    <a class="dropdown-item" class:disabled=data.is_none() href={data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.meine_bewerbung_url))}>
                        { "Meine Bewerbung" }
                        loading
                    </a>
                    <li>
                        <a href="/my-documents" class="dropdown-item bg-success-subtle">
                            { "Meine Dokumente" }
                        </a>
                    </li>
                </ul>
            </li>
            <li class="nav-item">
                <a class="nav-link" href={format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000340,-Ahilfe%2Ehtml", current_session.id)}>
                    { "Hilfe" }
                </a>
            </li>
    }
}
