use std::ops::Deref;

use crate::{RcTucanType, Route, common::use_data_loader};
use tucant_types::{
    Tucan,
    coursedetails::{CourseDetailsRequest, CourseDetailsResponse},
    student_result::{StudentResultLevel, StudentResultResponse},
};
use web_sys::HtmlSelectElement;
use yew::{Callback, Event, Html, HtmlResult, MouseEvent, Properties, TargetCast, function_component, html};
use yew_router::hooks::use_navigator;

#[derive(Properties, PartialEq)]
pub struct StudentResultProps {
    pub course_of_study: String,
}

#[function_component(StudentResult)]
pub fn student_result<TucanType: Tucan + 'static>(StudentResultProps { course_of_study }: &StudentResultProps) -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.student_result(&current_session, revalidation_strategy, additional).await;

    let navigator = use_navigator().unwrap();

    use_data_loader(handler, if course_of_study == "default" { 0 } else { course_of_study.parse().unwrap() }, 14 * 24 * 60 * 60, 60 * 60, |student_result: StudentResultResponse, reload| {
        let on_course_of_study_change = {
            let navigator = navigator.clone();
            Callback::from(move |e: Event| {
                let value = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
                navigator.push(&Route::StudentResult { course_of_study: value });
            })
        };
        ::yew::html! {
            <>
                <select onchange={on_course_of_study_change} class="form-select mb-1" aria-label="Select course of study">
                    {
                        student_result
                            .course_of_study
                            .iter()
                            .map(|course_of_study| {
                                ::yew::html! {
                                    <option selected={course_of_study.selected} value={course_of_study.value.clone()}>
                                        { &course_of_study.name }
                                    </option>
                                }
                            })
                            .collect::<Html>()
                    }
                </select>
                <StudentResultLevelComponent<TucanType> level={student_result.level0} path={Vec::new()} />
                <div>
                    { format!("Gesamt-GPA: {}", student_result.total_gpa) }
                </div>
                <div>
                    { format!("Hauptfach-GPA: {}", student_result.main_gpa) }
                </div></>
        }
    })
}

#[derive(Properties, PartialEq)]
pub struct StudentResultLevelProps {
    pub level: StudentResultLevel,
    pub path: Vec<String>,
}

#[function_component(StudentResultLevelComponent)]
pub fn student_result_level<TucanType: Tucan + 'static>(StudentResultLevelProps { level, path }: &StudentResultLevelProps) -> Html {
    ::yew::html! {
        <>
            if !level.entries.is_empty() {
                <h5>
                    <nav aria-label="breadcrumb">
                        <ol class="breadcrumb">
                            {
                                path.iter()
                                    .map(|item| {
                                        ::yew::html! {
                                            <li class="breadcrumb-item">
                                                { item }
                                            </li>
                                        }
                                    })
                                    .collect::<Html>()
                            }
                        </ol>
                    </nav>
                </h5>
                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th scope="col">
                                { "Name" }
                            </th>
                            <th scope="col">
                                { "Note" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            level
                                .entries
                                .iter()
                                .map(|entry| {
                                    ::yew::html! {
                                        <tr>
                                            <td>
                                                { &entry.name }
                                            </td>
                                            <td>
                                                { &entry.grade.clone().unwrap_or_default() }
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </tbody>
                </table>
            }
            {
                level
                    .children
                    .iter()
                    .map(|child| {
                        ::yew::html! {
                            <StudentResultLevelComponent<TucanType> level={child.clone()} path={path.iter().cloned().chain(std::iter::once(level.name.clone())).collect::<Vec<_>>()} />
                        }
                    })
                    .collect::<Html>()
            }</>
    }
}
