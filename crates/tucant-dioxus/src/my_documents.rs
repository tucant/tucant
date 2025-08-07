use dioxus::prelude::*;
use tucant_types::Tucan;

use crate::{common::use_authenticated_data_loader, RcTucanType};

#[component]
pub fn MyDocuments() -> Element {
    let handler =
        async |tucan: RcTucanType, current_session, revalidation_strategy, _additional| {
            tucan
                .my_documents(&current_session, revalidation_strategy)
                .await
        };

    use_authenticated_data_loader(
        handler,
        ReadSignal::new(Signal::new(())),
        14 * 24 * 60 * 60,
        60 * 60,
        |documents, reload| {
            rsx! {
                div {
                    h1 {
                        {"Meine Dokumente"}
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
                    div { class: "table-responsive",
                        table { class: "table",
                            thead {
                                tr {
                                    th { scope: "col", {"Name"} }
                                    th { scope: "col", {"Datum"} }
                                    th { scope: "col", {"URL"} }
                                }
                            }
                            tbody {
                                {
                                    documents
                                        .documents
                                        .iter()
                                        .map(|document| {
                                            rsx! {
                                                tr {
                                                    th { scope: "row", {document.name.clone()} }
                                                    td {
                                                        {document.date.clone()}
                                                        {" "}
                                                        {document.time.clone()}
                                                    }
                                                    td {
                                                        a { href: format!("https://www.tucan.tu-darmstadt.de{}", document.url), {"Download"} }
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
        },
    )
}
