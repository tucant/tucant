use dioxus::prelude::*;
use tucant_types::{LoginResponse, SemesterId, mlsstart::MlsStart, registration::AnmeldungRequest};

use crate::Route;

#[component]
pub fn Vorlesungsverzeichnisse(data: ReadOnlySignal<Option<MlsStart>>) -> Element {
    rsx! {
        {
            data().iter()
                .flat_map(|v| v.logged_in_head.vv.vvs.iter())
                .map(|(name, url)| {
                    let disabled = data().is_none().then_some("disabled").unwrap_or_default();
                    rsx! {
                        li {
                            Link {
                                to: Route::Vorlesungsverzeichnis { vv: url.clone() },
                                class: "dropdown-item bg-success-subtle {disabled}",
                                "{name}"
                                if data().is_none() {
                                    " "
                                    span { class: "spinner-grow spinner-grow-sm", "aria-hidden": "true" }
                                    span { class: "visually-hidden", role: "status",
                                        "Loading..."
                                    }
                                }
                            }
                        }
                    }
                })
        }
    }
}

#[component]
pub fn NavbarLoggedIn(current_session: ReadOnlySignal<LoginResponse>, data: ReadOnlySignal<Option<MlsStart>>) -> Element {
    let disabled = data().is_none().then_some("disabled").unwrap_or_default();
    rsx! {
            li { class: "nav-item dropdown",
                a { class: "nav-link dropdown-toggle", href: "#", role: "button", "data-bs-toggle": "dropdown", "aria-expanded": "false",
                    "Aktuelles"
                }
                ul { class: "dropdown-menu",
                    li {
                        Link { to: Route::Overview {}, class: "dropdown-item bg-success-subtle",
                            "Aktuelles"
                        }
                    }
                    li {
                        hr { class: "dropdown-divider" }
                    }
                    li {
                        a { class: "dropdown-item {disabled}", href: data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.messages_url)),
                            "Nachrichten"
                            if data().is_none() {
                                " "
                                span { class: "spinner-grow spinner-grow-sm", "aria-hidden": "true" }
                                span { class: "visually-hidden", role: "status",
                                    "Loading..."
                                }
                            }
                        }
                    }
                }
            }
            li { class:"nav-item dropdown",
                a { class:"nav-link dropdown-toggle", href:"#", role:"button", "data-bs-toggle": "dropdown", "aria-expanded": "false",
                    "VV"
                }
                ul { class: "dropdown-menu",
                    li {
                        Link { to: data.as_ref().map(|d| Route::Vorlesungsverzeichnis { vv: d.logged_in_head.vorlesungsverzeichnis_url.clone() }).unwrap_or(Route::NotFound { route: vec!["not-found".to_string()] }), class: "dropdown-item bg-success-subtle {disabled}",
                            "Vorlesungsverzeichnis"
                            if data().is_none() {
                                " "
                                span { class: "spinner-grow spinner-grow-sm", "aria-hidden": "true" }
                                span { class: "visually-hidden", role: "status",
                                    "Loading..."
                                }
                            }
                        }
                    }
                    li {
                        hr { class: "dropdown-divider" }
                    }
                    li {
                        a { class: "dropdown-item {disabled}", href: data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.vv.lehrveranstaltungssuche_url)),
                            "Lehrveranstaltungssuche"
                            if data().is_none() {
                                " "
                                span { class: "spinner-grow spinner-grow-sm", "aria-hidden": "true" }
                                span { class: "visually-hidden", role: "status",
                                    "Loading..."
                                }
                            }
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SEARCHROOM&ARGUMENTS=-N588840428781170,-N000387,",
                            "Raumsuche"
                        }
                    }
                    Vorlesungsverzeichnisse { data: data.clone() }
                    li {
                        a { class: "dropdown-item", href: "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N588840428781170,-N000464,-Avvarchivstart%2Ehtml",
                            "Archiv"
                        }
                    }
                }
            }
            li { class: "nav-item dropdown",
                a { class: "nav-link dropdown-toggle", href: "#", role: "button", "data-bs-toggle": "dropdown", "aria-expanded": "false",
                    "Stundenplan"
                }
                ul { class: "dropdown-menu",
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{:015},-N000268,-A,-A,-N1", current_session().id),
                            "Stundenplan"
                        }
                    }
                    li {
                        hr { class: "dropdown-divider" }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{:015},-N000269,-A,-A,-N0", current_session().id),
                            "Tagesansicht"
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{:015},-N000270,-A,-A,-N1", current_session().id),
                            "Wochenansicht"
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MONTH&ARGUMENTS=-N{:015},-N000271,-A", current_session().id),
                            "Monatsansicht"
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER_EXPORT&ARGUMENTS=-N{:015},-N000272,", current_session().id),
                            "Export"
                        }
                    }
                }
            }
            li { class: "nav-item dropdown",
                a { class: "nav-link dropdown-toggle", href: "#", role: "button", "data-bs-toggle": "dropdown", "aria-expanded": "false",
                    "Veranstaltungen"
                }
                ul { class: "dropdown-menu",
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000273,-Astudveranst%2Ehtml", current_session().id),
                            { "Veranstaltungen" }
                        }
                    }
                    li {
                        hr { class: "dropdown-divider"}
                    }
                    li {
                        Link { to: Route::MySemesterModules { semester: SemesterId::current() }, class: "dropdown-item bg-success-subtle",
                            { "Meine Semestermodule" }
                        }
                    }
                    li {
                        Link { to: Route::MyModules { semester: SemesterId::current() }, class: "dropdown-item bg-success-subtle",
                            { "Meine Module" }
                        }
                    }
                    li {
                        Link { to: Route::MyCourses { semester: SemesterId::current() }, class: "dropdown-item bg-success-subtle",
                            { "Meine Veranstaltungen" }
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENTCHOICECOURSES&ARGUMENTS=-N{:015},-N000307,", current_session().id),
                            { "Meine Wahlbereiche" }
                        }
                    }
                    li {
                        Link { to: Route::Registration { registration: AnmeldungRequest::default() }, class: "dropdown-item bg-success-subtle",
                            { "Anmeldung" }
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYREGISTRATIONS&ARGUMENTS=-N{:015},-N000308,-N000000000000000", current_session().id),
                            { "Mein aktueller Anmeldestatus" }
                        }
                    }
                }
            }
            li { class: "nav-item dropdown",
                a { class: "nav-link dropdown-toggle", href: "#", role: "button", "data-bs-toggle": "dropdown", "aria-expanded": "false",
                    { "Prüfungen" }
                }
                ul { class: "dropdown-menu",
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000280,-Astudpruefungen%2Ehtml", current_session().id),
                            { "Prüfungen" }
                        }
                    }
                    li {
                        hr { class: "dropdown-divider" }
                    }
                    li {
                        Link { to: Route::MyExams { semester: SemesterId::current() }, class: "dropdown-item bg-success-subtle",
                            { "Meine Prüfungen" }
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCPCHOICE&ARGUMENTS=-N{:015},-N000389,", current_session().id),
                            { "Mein Prüfungsplan" }
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000391,-Astudplan%2Ehtml", current_session().id),
                            { "Mein Prüfungsplan - Wichtige Hinweise" }
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000323,-Astudergebnis%2Ehtml", current_session().id),
                            { "Semesterergebnisse" }
                        }
                    }
                    li {
                        Link { to: Route::CourseResults { semester: SemesterId::current() }, class: "dropdown-item bg-success-subtle",
                            { "Modulergebnisse" }
                        }
                    }
                    li {
                        Link { to: Route::ExamResults { semester: SemesterId::current() }, class: "dropdown-item bg-success-subtle",
                            { "Prüfungsergebnisse" }
                        }
                    }
                    li {
                        Link { to: Route::StudentResult { course_of_study: "default".to_owned() }, class: "dropdown-item bg-success-subtle",
                            { "Leistungsspiegel" }
                        }
                    }
                }
            }
            li { class: "nav-item dropdown",
                a { class: "nav-link dropdown-toggle", href: "#", role: "button", "data-bs-toggle": "dropdown", "aria-expanded": "false",
                    { "Service" }
                }
                ul { class: "dropdown-menu",
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000337,-Aservice%2Ehtml", current_session().id),
                            { "Service" }
                        }
                    }
                    li {
                        hr { class: "dropdown-divider" }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PERSADDRESS&ARGUMENTS=-N{:015},-N000339,-A", current_session().id),
                            { "Persönliche Daten" }
                        }
                    }
                    li {
                        Link { to: Route::MyDocuments {}, class: "dropdown-item bg-success-subtle",
                            { "Meine Dokumente" }
                        }
                    }
                    a { class: "dropdown-item {disabled}", href: data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.antraege_url)),
                        "Anträge"
                        if data().is_none() {
                            " "
                            span { class: "spinner-grow spinner-grow-sm", "aria-hidden": "true" }
                            span { class: "visually-hidden", role: "status",
                                "Loading..."
                            }
                        }
                    }
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=HOLDINFO&ARGUMENTS=-N{:015},-N000652,", current_session().id),
                            { "Sperren" }
                        }
                    }
                }
            }
            li { class: "nav-item dropdown",
                a { class: "nav-link dropdown-toggle", href: "#", role: "button", "data-bs-toggle": "dropdown", "aria-expanded": "false",
                    { "Bewerbung" }
                }
                ul { class: "dropdown-menu",
                    li {
                        a { class: "dropdown-item", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000441,-Abewerbung", current_session().id),
                            { "Bewerbung" }
                        }
                    }
                    li {
                        hr { class: "dropdown-divider" }
                    }
                    a { class: "dropdown-item {disabled}", href: data.as_ref().map(|v| format!("https://www.tucan.tu-darmstadt.de{}", v.logged_in_head.meine_bewerbung_url)),
                        { "Meine Bewerbung" }
                        if data().is_none() {
                            " "
                            span { class: "spinner-grow spinner-grow-sm", "aria-hidden": "true" }
                            span { class: "visually-hidden", role: "status",
                                "Loading..."
                            }
                        }
                    }
                    li {
                        Link { to: Route::MyDocuments {}, class: "dropdown-item bg-success-subtle",
                            { "Meine Dokumente" }
                        }
                    }
                }
            }
            li { class: "nav-item",
                a { class: "nav-link", href: format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000340,-Ahilfe%2Ehtml", current_session().id),
                    { "Hilfe" }
                }
            }
    }
}
