use std::{rc::Rc, str::FromStr};

use tucant_types::{myexams::MyExamsResponse, DynTucan, SemesterId, Tucan};
use web_sys::HtmlSelectElement;
use dioxus::prelude::*;


use crate::{
    Route, common::use_authenticated_data_loader};


#[component]
pub fn MyExams(semester: SemesterId) -> Element {
    let handler = async |tucan: Rc<DynTucan>, current_session, revalidation_strategy, additional| tucan.my_exams(&current_session, revalidation_strategy, additional).await;

    let navigator = use_navigator();

    use_authenticated_data_loader(handler, semester.clone(), 14 * 24 * 60 * 60, 60 * 60, |exams: MyExamsResponse, reload| {
        let on_semester_change = {
            let navigator = navigator.clone();
            Callback::new(move |e: Event| {
                let value = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
                navigator.push(&Route::MyExams { semester: SemesterId::from_str(&value).unwrap() });
            })
        };
        rsx! {
            div {
                h1 {
                    { "Prüfungen" }
                    { " " }
                    button { onclick: reload, type: "button" class: "btn btn-light",
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        svg { xmlns: "http://www.w3.org/2000/svg" width: "16" height: "16" fill: "currentColor" class: "bi bi-arrow-clockwise" viewBox: "0 0 16 16",
                            path { fill-rule: "evenodd" d: "M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" }
                            path { d: "M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" }
                        }
                    }
                }
                select { onchange: on_semester_change, class: "form-select mb-1" aria-label: "Select semester",
                    {
                        exams
                            .semester
                            .iter()
                            .map(|semester| {
                                rsx! {
                                    option { selected: semester.selected, value: semester.value.inner().clone(),
                                        { &semester.name }
                                    }
                                }
                            })
                            
                    }
                }
                table { class: "table",
                    thead {
                        tr {
                            th { scope: "col",
                                { "NR" }
                            }
                            th { scope: "col",
                                { "Name" }
                            }
                            th { scope: "col",
                                { "Prüfungsart" }
                            }
                            th { scope: "col",
                                { "Termin" }
                            }
                        }
                    }
                    tbody {
                        {
                            exams
                                .exams
                                .iter()
                                .map(|exam| {
                                    rsx! {
                                        tr {
                                            th { scope: "row",
                                                { &exam.id }
                                            }
                                            td {
                                                if let Some(coursedetails_url) = &exam.coursedetails_url {
                                                    Link { to: Route::CourseDetails { course: coursedetails_url.clone() },
                                                        { &exam.name }
                                                    }
                                                }
                                                if let Some(moduledetails_url) = &exam.moduledetails_url {
                                                    Link { to: Route::ModuleDetails { module: moduledetails_url.clone() },
                                                        { &exam.name }
                                                    }
                                                }
                                            }
                                            td {
                                                a { href: format!("https://www.tucan.tu-darmstadt.de{}", exam.examdetail_url),
                                                    { &exam.pruefungsart }
                                                }
                                            }
                                            td {
                                                if let Some(courseprep_url) = &exam.courseprep_url {
                                                    a { href: format!("https://www.tucan.tu-darmstadt.de{}", courseprep_url),
                                                        { &exam.date }
                                                    }
                                                } else {
                                                    { &exam.date }
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
