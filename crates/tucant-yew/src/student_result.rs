use std::ops::Deref;

use crate::{RcTucanType, common::use_data_loader};
use tucant_types::{
    Tucan,
    coursedetails::{CourseDetailsRequest, CourseDetailsResponse},
    student_result::{StudentResultLevel, StudentResultResponse},
};
use yew::{Callback, Html, HtmlResult, MouseEvent, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct StudentResultProps {
    pub course_of_study: String,
}

#[function_component(StudentResult)]
pub fn student_result<TucanType: Tucan + 'static>(StudentResultProps { course_of_study }: &StudentResultProps) -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.student_result(&current_session, revalidation_strategy, additional).await;

    use_data_loader(handler, course_of_study.to_owned(), 14 * 24 * 60 * 60, 60 * 60, |student_result: StudentResultResponse, reload| {
        ::yew::html! {
            <>
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
