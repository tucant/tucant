use tucant_types::{Tucan, mlsstart::MlsStart};
use yew::{Html, function_component};
use yew_router::prelude::Link;

use crate::{RcTucanType, Route, common::use_authenticated_data_loader};

#[function_component(Mlsstart)]
pub fn mlsstart<TucanType: Tucan + 'static>() -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, _additional| tucan.0.after_login(&current_session, revalidation_strategy).await;

    use_authenticated_data_loader(handler, (), 14 * 24 * 60 * 60, 60 * 60, |mlsstart: MlsStart, reload| {
        ::yew::html! {
            <div>
                <h1>
                    { "Übersicht" }
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
                <h2>
                    { "Stundenplan" }
                </h2>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">
                                { "Kurs" }
                            </th>
                            <th scope="col">
                                { "Von" }
                            </th>
                            <th scope="col">
                                { "Bis" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            mlsstart
                                .stundenplan
                                .iter()
                                .map(|stundenplaneintrag| {
                                    ::yew::html! {
                                        <tr>
                                            <th scope="row">
                                                <Link<Route> to={Route::CourseDetails { course: stundenplaneintrag.coursedetails_url.clone() }}>
                                                    { &stundenplaneintrag.course_name }
                                                </Link<Route>>
                                            </th>
                                            <td>
                                                <a href={format!("https://www.tucan.tu-darmstadt.de{}", stundenplaneintrag.courseprep_url)}>
                                                    { &stundenplaneintrag.from }
                                                </a>
                                            </td>
                                            <td>
                                                <a href={format!("https://www.tucan.tu-darmstadt.de{}", stundenplaneintrag.courseprep_url)}>
                                                    { &stundenplaneintrag.to }
                                                </a>
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </tbody>
                </table>
                <h2>
                    { "Nachrichten" }
                </h2>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">
                                { "Datum" }
                            </th>
                            <th scope="col">
                                { "Absender" }
                            </th>
                            <th scope="col">
                                { "Nachricht" }
                            </th>
                            <th scope="col">
                                { "Löschen" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            mlsstart
                                .messages
                                .iter()
                                .map(|nachricht| {
                                    ::yew::html! {
                                        <tr>
                                            <th scope="row">
                                                { &nachricht.date }
                                            </th>
                                            <td>
                                                { &nachricht.source }
                                            </td>
                                            <td>
                                                <a href={format!("https://www.tucan.tu-darmstadt.de{}", nachricht.url)}>
                                                    { &nachricht.message }
                                                </a>
                                            </td>
                                            <td>
                                                <a href={format!("https://www.tucan.tu-darmstadt.de{}", nachricht.delete_url)}>
                                                    { "Löschen" }
                                                </a>
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
