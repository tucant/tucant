use std::ops::Deref;

use log::info;
use time::Duration;
use tucant_types::{LoginResponse, RevalidationStrategy, Tucan, moduledetails::ModuleDetailsRequest};
use wasm_bindgen_futures::spawn_local;
use yew::{Callback, Html, HtmlResult, MouseEvent, Properties, UseStateHandle, function_component, html, use_context, use_effect_with, use_state};

use crate::RcTucanType;

#[derive(Properties, PartialEq)]
pub struct ModuleDetailsProps {
    pub module_details: ModuleDetailsRequest,
}

#[function_component(ModuleDetails)]
pub fn module_details<TucanType: Tucan + 'static>(ModuleDetailsProps { module_details }: &ModuleDetailsProps) -> HtmlResult {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let data = use_state(|| Ok(None));
    let loading = use_state(|| false);
    let current_session_handle = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        let current_session_handle = current_session_handle.clone();
        let tucan = tucan.clone();
        use_effect_with(module_details.clone(), move |request| {
            if let Some(current_session) = (*current_session_handle).to_owned() {
                loading.set(true);
                let anmeldung_request = request.clone();
                let data = data.clone();
                spawn_local(async move {
                    match tucan.0.module_details(&current_session, RevalidationStrategy { max_age: Duration::days(14).whole_seconds(), invalidate_dependents: Some(true) }, anmeldung_request.clone()).await {
                        Ok(response) => {
                            data.set(Ok(Some(response)));
                            loading.set(false);

                            match tucan.0.module_details(&current_session, RevalidationStrategy { max_age: Duration::days(3).whole_seconds(), invalidate_dependents: Some(true) }, anmeldung_request).await {
                                Ok(response) => data.set(Ok(Some(response))),
                                Err(error) => {
                                    info!("ignoring error when refetching: {}", error)
                                }
                            }
                        }
                        Err(error) => {
                            data.set(Err(error.to_string()));
                            loading.set(false);
                        }
                    }
                })
            } else {
                data.set(Err("Not logged in".to_owned()));
            }
        });
    }

    let reload = {
        let current_session = current_session_handle.clone();
        let module_details = module_details.clone();
        let data = data.clone();
        let loading = loading.clone();
        let current_session = current_session.clone();
        let tucan = tucan.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(current_session) = (*current_session).to_owned() {
                loading.set(true);
                let module_details = module_details.clone();
                let data = data.clone();
                let tucan = tucan.clone();
                let loading = loading.clone();
                spawn_local(async move {
                    match tucan.0.module_details(&current_session, RevalidationStrategy { max_age: 0, invalidate_dependents: Some(true) }, module_details.clone()).await {
                        Ok(response) => {
                            data.set(Ok(Some(response)));
                            loading.set(false);
                        }
                        Err(error) => {
                            data.set(Err(error.to_string()));
                            loading.set(false);
                        }
                    }
                })
            } else {
                data.set(Err("Not logged in".to_owned()));
            }
        })
    };

    let data = match data.deref() {
        Ok(data) => data,
        Err(error) => {
            return Ok(html! {
                <div class="container">
                    <div class="alert alert-danger d-flex align-items-center mt-2" role="alert">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors
                        <svg xmlns="http://www.w3.org/2000/svg" class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2" width="16" height="16" viewBox="0 0 16 16" role="img" aria-label="Error:">
                            <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" />
                        </svg>
                        <div>{ error }</div>
                    </div>
                </div>
            });
        }
    };

    Ok(html! {
        <div class="container">
            if *loading {
                <div style="z-index: 10000" class="position-fixed top-50 start-50 translate-middle">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            }

            if let Some(module) = data {
                    <div>
                        <h1>
                            { &module.module_id }
                            if let Some(credits) = &module.credits {
                                {" "}<span class="badge text-bg-secondary">{ format!("{credits} CP", ) }</span>
                            }
                            if module.registered {
                                {" "}<span class="badge text-bg-secondary">{ "Angemeldet" }</span>
                            }
                        </h1>

                        <h2>{"Modulverantwortliche"}</h2>
                        <ul>
                        {
                            module.modulverantwortliche.iter().map(|modulverantwortliche| {
                                html!{
                                    <li>{ &modulverantwortliche.0 }</li>
                                }
                            }).collect::<Html>()
                        }
                        </ul>

                        <h2>{"Kurse"}</h2>

                        {
                            module.kurskategorien.iter().map(|kurskategorie| {
                                html!{
                                    <>
                                        <h3>
                                            {& kurskategorie.course_no}
                                            {" "}{ &kurskategorie.name }
                                            if kurskategorie.credits != 0.0 {
                                                {" "}<span class="badge text-bg-secondary">{ format!("{} CP", kurskategorie.credits) }</span>
                                            }
                                            if kurskategorie.mandatory {
                                                {" "}<span class="badge text-bg-secondary">{ "Pflicht" }</span>
                                            }
                                            if let Some(semester) = &kurskategorie.semester {
                                                if *semester != 1 {
                                                    {" "}<span class="badge text-bg-secondary">{ format!("{semester} Semester") }</span>
                                                }
                                            }
                                        </h3>
                                        <table class="table">
                                          <thead>
                                          <tr>
                                              <th scope="col">{"Nummer"}</th>
                                              <th scope="col">{"Name"}</th>
                                              <th scope="col">{"Semester"}</th>
                                          </tr>
                                          </thead>
                                          <tbody>
                                        {
                                            kurskategorie.kurse.iter().map(|kurs| {
                                            html! {
                                                <tr>
                                                    <th scope="row">{&kurs.course_id}</th>
                                                    <td>{&kurs.name}</td>
                                                    <td>{&kurs.semester}</td>
                                                </tr>
                                            }
                                        }).collect::<Html>()
                                        }
                                            </tbody>
                                        </table>
                                    </>
                                }
                            }).collect::<Html>()
                        }

                        <h2>{"Leistungen"}</h2>
                        {
                            module.leistungen.iter().map(|leistung| {
                                html!{
                                    <>
                                    <h3>
                                        { &leistung.name }
                                        if leistung.compulsory {
                                            {" "}<span class="badge text-bg-secondary">{ "Pflicht" }</span>
                                        }
                                        {" "}<span class="badge text-bg-secondary">{ format!("{} Gewichtung", leistung.weight) }</span>
                                        if let Some(weight_more) = &leistung.weight_more {
                                            {" "}<span class="badge text-bg-secondary">{ format!("Zusatzinfo {weight_more}") }</span>
                                        }
                                    </h3>
                                    </>
                                }
                            }).collect::<Html>()
                        }

                        <h2>{"Pruefungen"}</h2>
                        {
                            module.pruefungen.iter().map(|pruefung| {
                                html!{
                                    <>
                                    <h3>
                                        { &pruefung.name }
                                        if pruefung.compulsory {
                                            {" "}<span class="badge text-bg-secondary">{ "Pflicht" }</span>
                                        }
                                    </h3>
                                    <table class="table">
                                          <thead>
                                          <tr>
                                              <th scope="col">{"Name"}</th>
                                              <th scope="col">{"Datum"}</th>
                                              <th scope="col">{"Prüfende"}</th>
                                          </tr>
                                          </thead>
                                          <tbody>
                                    {
                                        pruefung.termine.iter().map(|termin| {
                                        html! {
                                            <tr>
                                                <th scope="row">{& termin.subname}</th>
                                                <td>{  &termin.date }</td>
                                                <td>{&termin.examiner}</td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                    }
                                    </tbody>
                                    </table>
                                    </>
                                }
                            }).collect::<Html>()
                        }

                        <h2>{"Beschreibung"}</h2>

                        // TODO FIXME this is dangerous
                        { Html::from_html_unchecked(module.description.join("\n").into()) }

                        <h2>{"Sonstige Informationen"}</h2>

                        if module.abweichende_credits {
                            <div class="alert alert-warning" role="alert">
                              {"Hinweis: In Ihrer Prüfungsordnung können abweichende Credits festgelegt sein."}
                            </div>
                        }

                        if let Some(anmeldefristen) = &module.anmeldefristen {
                            <div>{ format!("Anmeldefrist: {}", anmeldefristen.registration_range) }</div>
                            <div>{ format!("Abmeldefrist: {}", anmeldefristen.unregistration_range) }</div>
                        }

                        <div>{ format!("Startsemester: {}", module.start_semester) }</div>

                        if let Some(display_in_timetable) = &module.display_in_timetable {
                            <div>{ format!("Display in timetable: {}", display_in_timetable) }</div>
                        }

                        <div>{ format!("Dauer: {}", module.duration) }</div>

                        <div>{ format!("Anzahl Wahlkurse: {}", module.count_elective_courses) }</div>

                        <br />
                    </div>
                }
        </div>
    })
}
