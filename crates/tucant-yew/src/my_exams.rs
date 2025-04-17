use std::ops::Deref;

use tucant_types::{SemesterId, Tucan, myexams::MyExamsResponse};
use web_sys::HtmlSelectElement;
use yew::{Callback, Event, Html, HtmlResult, Properties, TargetCast, function_component, html};
use yew_router::hooks::use_navigator;

use crate::{RcTucanType, Route, common::use_data_loader};

#[derive(Properties, PartialEq)]
pub struct MyExamsProps {
    pub semester: SemesterId,
}

#[function_component(MyExams)]
pub fn my_exams<TucanType: Tucan + 'static>(MyExamsProps { semester }: &MyExamsProps) -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.my_exams(&current_session, revalidation_strategy, additional).await;

    let navigator = use_navigator().unwrap();

    use_data_loader(handler, semester.clone(), 14 * 24 * 60 * 60, 60 * 60, |exams: MyExamsResponse, reload| {
        let on_semester_change = {
            let navigator = navigator.clone();
            Callback::from(move |e: Event| {
                let value = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
                navigator.push(&Route::MyExams { semester: SemesterId(value) });
            })
        };
        ::yew::html! {
            <div>
                <h1>
                    { "Prüfungen" }
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
                        exams
                            .semester
                            .iter()
                            .map(|semester| {
                                ::yew::html! {
                                    <option selected={semester.selected} value={semester.value.0.clone()}>
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
                                { "NR" }
                            </th>
                            <th scope="col">
                                { "Name" }
                            </th>
                            <th scope="col">
                                { "Prüfungsart" }
                            </th>
                            <th scope="col">
                                { "Termin" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            exams
                                .exams
                                .iter()
                                .map(|exam| {
                                    ::yew::html! {
                                        <tr>
                                            <th scope="row">
                                                { &exam.id }
                                            </th>
                                            <td>
                                                { &exam.name }
                                            </td>
                                            <td>
                                                { &exam.pruefungsart }
                                            </td>
                                            <td>
                                                { &exam.date }
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
