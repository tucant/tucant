use std::ops::Deref;

use tucant_types::{Tucan, moduledetails::ModuleDetailsRequest};
use yew::{Html, HtmlResult, Properties, function_component, html};

use crate::{RcTucanType, common::use_data_loader};

#[derive(Properties, PartialEq)]
pub struct ModuleDetailsProps {
    pub module_details: ModuleDetailsRequest,
}

#[function_component(ModuleDetails)]
pub fn module_details<TucanType: Tucan + 'static>(ModuleDetailsProps { module_details }: &ModuleDetailsProps) -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.module_details(&current_session, revalidation_strategy, additional).await;

    use_data_loader(handler, module_details.clone(), 14 * 24 * 60 * 60, 60 * 60, |module, reload| {
        html! {
            <div>
                <h1>
                    { &module.module_id }
                    if let Some(credits) = &module.credits {
                        {" "}<span class="badge text-bg-secondary">{ format!("{credits} CP", ) }</span>
                    }
                    if module.registered {
                        {" "}<span class="badge text-bg-secondary">{ "Angemeldet" }</span>
                    }{" "}<button onclick={reload} type="button" class="btn btn-light">
                    // https://github.com/twbs/icons
                    // The MIT License (MIT)
                    // Copyright (c) 2019-2024 The Bootstrap Authors
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                        <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z"/>
                        <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466"/>
                    </svg>
                </button>
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
    })
}
