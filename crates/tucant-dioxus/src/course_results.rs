use std::str::FromStr;

use dioxus::prelude::*;
use tucant_types::{SemesterId, Tucan, courseresults::ModuleResultsResponse};

use crate::{RcTucanType, Route, common::use_authenticated_data_loader};

#[component]
pub fn CourseResults(semester: ReadOnlySignal<SemesterId>) -> Element {
    let handler = async |tucan: RcTucanType, current_session, revalidation_strategy, additional| tucan.course_results(&current_session, revalidation_strategy, additional).await;

    let navigator = use_navigator();

    use_authenticated_data_loader(handler, semester, 14 * 24 * 60 * 60, 60 * 60, |course_results: ModuleResultsResponse, reload| {
        let on_semester_change = {
            Callback::new(move |e: Event<FormData>| {
                let value = e.value();
                navigator.push(Route::CourseResults { semester: SemesterId::from_str(&value).unwrap() });
            })
        };
        rsx! {
            div {
                h1 {
                    { "Modulergebnisse" }
                    { " " }
                    button { onclick: reload, type: "button", class: "btn btn-light",
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        svg { xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", fill: "currentColor", class: "bi bi-arrow-clockwise", view_box: "0 0 16 16",
                            path { "fill-rule": "evenodd", d: "M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" }
                            path { d: "M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" }
                        }
                    }
                }
                select { onchange: on_semester_change, class: "form-select mb-1", "aria-label": "Select semester",
                    {
                        course_results
                            .semester
                            .iter()
                            .map(|semester| {
                                rsx! {
                                    option { selected: semester.selected, value: semester.value.inner().clone(),
                                        { semester.name.clone() }
                                    }
                                }
                            })

                    }
                }
                table { class: "table",
                    thead {
                        tr {
                            th { scope: "col",
                                { "Nr" }
                            }
                            th { scope: "col",
                                { "Name" }
                            }
                            th { scope: "col",
                                { "Credits" }
                            }
                            th { scope: "col",
                                { "Note" }
                            }
                            th { scope: "col",
                                { "Status" }
                            }
                            th { scope: "col",
                                { "Prüfungen" }
                            }
                            th { scope: "col",
                                { "Ø" }
                            }
                        }
                    }
                    tbody {
                        {
                            course_results
                                .results
                                .iter()
                                .map(|exam| {
                                    rsx! {
                                        tr {
                                            th { scope: "row",
                                                { exam.nr.clone() }
                                            }
                                            td {
                                                { exam.name.clone() }
                                            }
                                            td {
                                                { exam.credits.clone() }
                                            }
                                            td {
                                                { exam.grade.clone().unwrap_or_else(|| "-".to_owned()) }
                                            }
                                            td {
                                                { exam.status.clone().unwrap_or_default().clone() }
                                            }
                                            td {
                                                if let Some(pruefungen_url) = &exam.pruefungen_url {
                                                    a { href: format!("https://www.tucan.tu-darmstadt.de{}", pruefungen_url),
                                                        { "Prüfungen" }
                                                    }
                                                }
                                            }
                                            td {
                                                if let Some(average_url) = &exam.average_url {
                                                    a { href: format!("https://www.tucan.tu-darmstadt.de{}", average_url),
                                                        { "Ø" }
                                                    }
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
    })
}
