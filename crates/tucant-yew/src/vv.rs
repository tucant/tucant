use std::ops::Deref as _;

use tucant_types::{
    Tucan,
    vv::{ActionRequest, Vorlesungsverzeichnis},
};
use yew::{Html, HtmlResult, Properties, function_component, html};
use yew_router::prelude::Link;

use crate::{RcTucanType, Route, common::use_data_loader};

#[derive(Properties, PartialEq)]
pub struct VorlesungsverzeichnisProps {
    pub vv: ActionRequest,
}

#[function_component(VorlesungsverzeichnisComponent)]
pub fn vorlesungsverzeichnis<TucanType: Tucan + 'static>(VorlesungsverzeichnisProps { vv }: &VorlesungsverzeichnisProps) -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.vv(Some(&current_session), revalidation_strategy, additional).await;

    use_data_loader(handler, vv.to_owned(), 28 * 24 * 60 * 60, 24 * 60 * 60, |data, reload| {
        ::yew::html! {
            <div class="container">
                <h2 class="text-center">
                    { &data.title }
                    <button onclick={reload} type="button" class="btn btn-light">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" />
                                <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" />
                                </svg>
                            </button>
                        </h2>
                        <nav style="min-height: 5.5rem" aria-label="breadcrumb">
                            <ol class="breadcrumb">
                                {
                data.path
                    .iter()
                    .map(|entry| {
                        yew::html!{<li class="breadcrumb-item"><Link<Route> to={Route::Vorlesungsverzeichnis { vv: entry.1.clone()}}>{entry.0.clone()}</Link<Route>></li>}
                    })
                    .collect::<Html>()
            }
                            </ol>
                        </nav>
                        // TODO FIXME this is dangerous

                        { Html::from_html_unchecked(data.description.join("\n").into()) }
                        <h2 class="text-center">
                            { "Submenus" }
                        </h2>
                        <ul class="list-group">
                            {
                data.entries
                    .iter()
                    .map(|entry| {
                        yew::html!{<Link<Route> to={Route::Vorlesungsverzeichnis { vv: entry.1.clone()}} classes="list-group-item list-group-item-action">{ format!("{}", entry.0) }</Link<Route>>}
                    })
                    .collect::<Html>()
            }
                        </ul>
                        <h2 class="text-center">
                            { "Modules and courses" }
                        </h2>
                        <ul class="list-group">
                            {
                data.veranstaltungen_or_module
                    .iter()
                    .map(|entry| {
                        yew::html!{
                            <li class="list-group-item">
                                <div class="d-flex w-100 justify-content-between">
                                    <h5 class="mb-1"><Link<Route> to={Route::CourseDetails { course: entry.coursedetails_url.clone() }}>{ format!("Kurs {}", entry.title) }</Link<Route>></h5>
                                </div>

                                <div class="d-flex w-100 justify-content-between">
                                    <h6 class="mb-1">{ format!("{}", entry.lecturer_name.clone().unwrap_or_default()) }</h6>
                                </div>

                                <h6 class="mb-1">{ format!("{}", entry.date_range.clone().unwrap_or_default()) }</h6>
                            </li>
                        }
                    })
                    .collect::<Html>()
            }
                        </ul>
                    </div>
        }
    })
}
