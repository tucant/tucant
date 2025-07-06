use crate::{common::use_authenticated_data_loader, RcTucanType, Route};
use dioxus::prelude::*;
use tucant_types::{
    student_result::{StudentResultLevel, StudentResultResponse},
    Tucan,
};

#[component]
pub fn StudentResult(course_of_study: ReadOnlySignal<String>) -> Element {
    let handler = async |tucan: RcTucanType, current_session, revalidation_strategy, additional| {
        tucan
            .student_result(&current_session, revalidation_strategy, additional)
            .await
    };

    let navigator = use_navigator();

    let memo = use_memo(move || {
        if course_of_study() == "default" {
            0
        } else {
            course_of_study().parse().unwrap()
        }
    });

    use_authenticated_data_loader(
        handler,
        memo.into(),
        14 * 24 * 60 * 60,
        60 * 60,
        |student_result: StudentResultResponse, reload| {
            let on_course_of_study_change = {
                Callback::new(move |e: Event<FormData>| {
                    let value = e.value();
                    navigator.push(Route::StudentResult {
                        course_of_study: value,
                    });
                })
            };
            rsx! {
                h1 {
                    {"Leistungsspiegel"}
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
                select {
                    onchange: on_course_of_study_change,
                    class: "form-select mb-1",
                    "aria-label": "Select course of study",
                    {
                        student_result
                            .course_of_study
                            .iter()
                            .map(|course_of_study| {
                                rsx! {
                                    option { selected: course_of_study.selected, value: course_of_study.value.clone(),
                                        {course_of_study.name.clone()}
                                    }
                                }
                            })
                    }
                }
                StudentResultLevelComponent { level: student_result.level0, path: Vec::new() }
                div { {format!("Gesamt-GPA: {}", student_result.total_gpa)} }
                div { {format!("Hauptfach-GPA: {}", student_result.main_gpa)} }
            }
        },
    )
}

#[component]
pub fn StudentResultLevelComponent(
    level: ReadOnlySignal<StudentResultLevel>,
    path: ReadOnlySignal<Vec<String>>,
) -> Element {
    rsx! {
        if !level().entries.is_empty() {
            h5 {
                nav { "aria-label": "breadcrumb",
                    ol { class: "breadcrumb",
                        {
                            path.iter()
                                .map(|item| {
                                    rsx! {
                                        li { class: "breadcrumb-item", {item.clone()} }
                                    }
                                })
                        }
                        li { class: "breadcrumb-item", {level().name.clone()} }
                    }
                }
            }
            table { class: "table table-sm",
                thead {
                    tr {
                        th { scope: "col", {"Name"} }
                        th { scope: "col", class: "col-1", {"CP"} }
                        th { scope: "col", class: "col-1", {"eCP"} }
                        th { scope: "col", class: "col-1", {"Note"} }
                        th { scope: "col", class: "col-1", {"Status"} }
                    }
                }
                tbody {
                    {
                        level()
                            .entries
                            .iter()
                            .map(|entry| {
                                rsx! {
                                    tr {
                                        td { {entry.name.clone()} }
                                        td { {entry.cp.clone().unwrap_or_default().to_string()} }
                                        td { {entry.used_cp.clone().unwrap_or_default().to_string()} }
                                        if let Some(grade) = &entry.grade {
                                            td { "{grade}" }
                                        }
                                        td { {entry.state.clone()} }
                                    }
                                }
                            })
                    }
                }
            }
        }
        {
            level()
                .children
                .iter()
                .map(|child| {
                    rsx! {
                        StudentResultLevelComponent {
                            level: child.clone(),
                            path: path()
                                .iter()
                                .cloned()
                                .chain(std::iter::once(level().name.clone()))
                                .collect::<Vec<_>>(),
                        }
                    }
                })
        }
    }
}
