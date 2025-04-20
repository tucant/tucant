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
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">
                                { "Modul" }
                            </th>
                            <th scope="col">
                                { "Note" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <StudentResultLevelComponent<TucanType> level={student_result.level0} depth={1} />
                    </tbody>
                </table>
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
    pub depth: u8,
}

#[function_component(StudentResultLevelComponent)]
pub fn student_result_level<TucanType: Tucan + 'static>(StudentResultLevelProps { level, depth }: &StudentResultLevelProps) -> Html {
    ::yew::html! {
        <>
            <tr>
                <td class={format!("ps-{}", depth - 1)} colspan="2">
                    { format!("{}. {}", depth, level.name) }
                </td>
            </tr>
            if !level.entries.is_empty() {
                {
                    level
                        .entries
                        .iter()
                        .map(|entry| {
                            ::yew::html! {
                                <tr>
                                    <td class={format!("ps-{}", depth)}>
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
            }
            {
                level
                    .children
                    .iter()
                    .map(|child| {
                        ::yew::html! {
                            <StudentResultLevelComponent<TucanType> level={child.clone()} depth={depth + 1} />
                        }
                    })
                    .collect::<Html>()
            }</>
    }
}
