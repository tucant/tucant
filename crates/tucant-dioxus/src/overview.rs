use dioxus::prelude::*;
use tucant_types::{Tucan, mlsstart::MlsStart};

use crate::{RcTucanType, Route, common::use_authenticated_data_loader};

#[component]
pub fn Overview() -> Element {
    let handler = async |tucan: RcTucanType, current_session, revalidation_strategy, _additional| tucan.after_login(&current_session, revalidation_strategy).await;

    use_authenticated_data_loader(handler, ReadSignal::new(Signal::new(())), 14 * 24 * 60 * 60, 60 * 60, |mlsstart: MlsStart, reload| {
        rsx! {
            div {
                h1 {
                    {"Übersicht"}
                    {" "}
                    button {
                        onclick: reload,
                        r#type: "button",
                        class: "btn btn-secondary",
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            fill: "currentColor",
                            class: "bi bi-arrow-clockwise",
                            view_box: "0 0 16 16",
                            path {
                                "fill-rule": "evenodd",
                                d: "M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z",
                            }
                            path { d: "M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" }
                        }
                    }
                }
                h2 { {"Stundenplan"} }
                div { class: "table-responsive",
                    table { class: "table",
                        thead {
                            tr {
                                th { scope: "col", {"Kurs"} }
                                th { scope: "col", {"Von"} }
                                th { scope: "col", {"Bis"} }
                            }
                        }
                        tbody {
                            {
                                mlsstart
                                    .stundenplan
                                    .iter()
                                    .map(|stundenplaneintrag| {
                                        rsx! {
                                            tr {
                                                th { scope: "row",
                                                    Link {
                                                        to: Route::CourseDetails {
                                                            course: stundenplaneintrag.coursedetails_url.clone(),
                                                        },
                                                        {stundenplaneintrag.course_name.clone()}
                                                    }
                                                }
                                                td {
                                                    a { href: format!("https://www.tucan.tu-darmstadt.de{}", stundenplaneintrag.courseprep_url),
                                                        {stundenplaneintrag.from.clone()}
                                                    }
                                                }
                                                td {
                                                    a { href: format!("https://www.tucan.tu-darmstadt.de{}", stundenplaneintrag.courseprep_url),
                                                        {stundenplaneintrag.to.clone()}
                                                    }
                                                }
                                            }
                                        }
                                    })
                            }
                        }
                    }
                }
                h2 { {"Nachrichten"} }
                div { class: "table-responsive",
                    table { class: "table",
                        thead {
                            tr {
                                th { scope: "col", {"Datum"} }
                                th { scope: "col", {"Absender"} }
                                th { scope: "col", {"Nachricht"} }
                                th { scope: "col", {"Löschen"} }
                            }
                        }
                        tbody {
                            {
                                mlsstart
                                    .messages
                                    .iter()
                                    .map(|nachricht| {
                                        rsx! {
                                            tr {
                                                th { scope: "row", {nachricht.date.clone()} }
                                                td { {nachricht.source.clone()} }
                                                td {
                                                    a { href: format!("https://www.tucan.tu-darmstadt.de{}", nachricht.url),
                                                        {nachricht.message.clone()}
                                                    }
                                                }
                                                td {
                                                    a { href: format!("https://www.tucan.tu-darmstadt.de{}", nachricht.delete_url),
                                                        {"Löschen"}
                                                    }
                                                }
                                            }
                                        }
                                    })
                            }
                        }
                    }
                }
            }
        }
    })
}
