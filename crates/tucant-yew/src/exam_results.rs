use std::{ops::Deref, str::FromStr};

use tucant_types::{SemesterId, Tucan, examresults::ExamResultsResponse};
use web_sys::HtmlSelectElement;
use yew::{Callback, Event, Html, HtmlResult, Properties, TargetCast, function_component, html};
use yew_router::hooks::use_navigator;

use crate::{RcTucanType, Route, common::use_data_loader};

#[derive(Properties, PartialEq)]
pub struct ExamResultsProps {
    pub semester: SemesterId,
}

#[function_component(ExamResults)]
pub fn exam_results<TucanType: Tucan + 'static>(ExamResultsProps { semester }: &ExamResultsProps) -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.exam_results(&current_session, revalidation_strategy, additional).await;

    let navigator = use_navigator().unwrap();

    use_data_loader(handler, semester.clone(), 14 * 24 * 60 * 60, 60 * 60, |exam_results: ExamResultsResponse, reload| {
        let on_semester_change = {
            let navigator = navigator.clone();
            Callback::from(move |e: Event| {
                let value = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
                navigator.push(&Route::ExamResults { semester: SemesterId::from_str(&value).unwrap() });
            })
        };
        ::yew::html! {
            <div>
                <h1>
                    { "Prüfungsergebnisse" }
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
                        exam_results
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
                                { "Name" }
                            </th>
                            <th scope="col">
                                { "Art" }
                            </th>
                            <th scope="col">
                                { "Datum" }
                            </th>
                            <th scope="col">
                                { "Note" }
                            </th>
                            <th scope="col">
                                { "Ø" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            exam_results
                                .results
                                .iter()
                                .map(|exam| {
                                    ::yew::html! {
                                        <tr>
                                            <th scope="row">
                                                { &exam.name }
                                            </th>
                                            <td>
                                                { &exam.exam_type }
                                            </td>
                                            <td>
                                                { exam.date.clone().unwrap_or_else(|| "-".to_owned()) }
                                            </td>
                                            <td>
                                                { &exam.grade }
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
