use tucant_types::{LoginResponse, Tucan, moduledetails::ModuleDetailsRequest};
use wasm_bindgen_futures::spawn_local;
use yew::{Html, HtmlResult, Properties, UseStateHandle, function_component, html, use_context, use_effect_with, use_state};

use crate::RcTucanType;

#[derive(Properties, PartialEq)]
pub struct ModuleDetailsProps {
    pub module_details: ModuleDetailsRequest,
}

#[function_component(ModuleDetails)]
pub fn module_details<TucanType: Tucan + 'static>(ModuleDetailsProps { module_details }: &ModuleDetailsProps) -> HtmlResult {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let data = use_state(|| None);
    let loading = use_state(|| false);
    let current_session_handle = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        use_effect_with(module_details.to_owned(), move |request| {
            if let Some(current_session) = (*current_session_handle).to_owned() {
                loading.set(true);
                let request = request.clone();
                let data = data.clone();
                spawn_local(async move {
                    let response = tucan.0.module_details(&current_session, request).await.unwrap();
                    data.set(Some(response));
                    loading.set(false);
                })
            }
        });
    }

    Ok(html! {
        <div class="container">
            { data.as_ref().map(|module| {
                html!{
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
            }).unwrap_or_else(|| html! { if *loading {
                <div style="z-index: 10000" class="position-fixed top-50 start-50 translate-middle">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            } }) }
        </div>
    })
}
