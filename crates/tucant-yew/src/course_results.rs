use std::str::FromStr;

use tucant_types::{SemesterId, Tucan, courseresults::ModuleResultsResponse};
use web_sys::HtmlSelectElement;
use yew::{Callback, Event, Html, Properties, TargetCast, function_component};
use yew_router::hooks::use_navigator;

use crate::{
    RcTucanType, Route,
    common::{use_authenticated_data_loader, use_data_loader},
};

#[derive(Properties, PartialEq)]
pub struct CourseResultsProps {
    pub semester: SemesterId,
}

#[function_component(CourseResults)]
pub fn course_results<TucanType: Tucan + 'static>(CourseResultsProps { semester }: &CourseResultsProps) -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.course_results(&current_session, revalidation_strategy, additional).await;

    let navigator = use_navigator().unwrap();

    use_authenticated_data_loader(handler, semester.clone(), 14 * 24 * 60 * 60, 60 * 60, |course_results: ModuleResultsResponse, reload| {
        let on_semester_change = {
            let navigator = navigator.clone();
            Callback::from(move |e: Event| {
                let value = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
                navigator.push(&Route::CourseResults { semester: SemesterId::from_str(&value).unwrap() });
            })
        };
        ::yew::html! {
            <div>
                <h1>
                    { "Modulergebnisse" }
                    { " " }
                    <button onclick={reload} type="button" class="btn btn-light">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" />
                            <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" />
                        </svg>
                    </button>
                </h1>
                <select onchange={on_semester_change} class="form-select mb-1" aria-label="Select semester">
                    {
                        course_results
                            .semester
                            .iter()
                            .map(|semester| {
                                ::yew::html! {
                                    <option selected={semester.selected} value={semester.value.inner().clone()}>
                                        { &semester.name }
                                    </option>
                                }
                            })
                            .collect::<Html>()
                    }
                </select>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">
                                { "Nr" }
                            </th>
                            <th scope="col">
                                { "Name" }
                            </th>
                            <th scope="col">
                                { "Credits" }
                            </th>
                            <th scope="col">
                                { "Note" }
                            </th>
                            <th scope="col">
                                { "Status" }
                            </th>
                            <th scope="col">
                                { "Prüfungen" }
                            </th>
                            <th scope="col">
                                { "Ø" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            course_results
                                .results
                                .iter()
                                .map(|exam| {
                                    ::yew::html! {
                                        <tr>
                                            <th scope="row">
                                                { &exam.nr }
                                            </th>
                                            <td>
                                                { &exam.name }
                                            </td>
                                            <td>
                                                { &exam.credits }
                                            </td>
                                            <td>
                                                { exam.grade.clone().unwrap_or_else(|| "-".to_owned()) }
                                            </td>
                                            <td>
                                                { &exam.status.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                if let Some(pruefungen_url) = &exam.pruefungen_url {
                                                    <a href={format!("https://www.tucan.tu-darmstadt.de{}", pruefungen_url)}>
                                                        { "Prüfungen" }
                                                    </a>
                                                }
                                            </td>
                                            <td>
                                                if let Some(average_url) = &exam.average_url {
                                                    <a href={format!("https://www.tucan.tu-darmstadt.de{}", average_url)}>
                                                        { "Ø" }
                                                    </a>
                                                }
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </tbody>
                </table>
            </div>
        }
    })
}
