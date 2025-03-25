use tucant_types::{LoginResponse, Tucan, coursedetails::CourseDetailsRequest};
use wasm_bindgen_futures::spawn_local;
use yew::{Html, HtmlResult, Properties, UseStateHandle, function_component, html, use_context, use_effect_with, use_state};

use crate::RcTucanType;

#[derive(Properties, PartialEq)]
pub struct CourseDetailsProps {
    pub course_details: CourseDetailsRequest,
}

#[function_component(CourseDetails)]
pub fn course_details<TucanType: Tucan + 'static>(CourseDetailsProps { course_details }: &CourseDetailsProps) -> HtmlResult {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let data = use_state(|| None);
    let loading = use_state(|| false);
    let current_session_handle = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        use_effect_with(course_details.to_owned(), move |request| {
            if let Some(current_session) = (*current_session_handle).to_owned() {
                loading.set(true);
                let request = request.clone();
                let data = data.clone();
                spawn_local(async move {
                    let response = tucan.0.course_details(&current_session, request).await.unwrap();
                    data.set(Some(response));
                    loading.set(false);
                })
            }
        });
    }

    Ok(html! {
        <div class="container">
            { data.as_ref().map(|course| {
                html!{
                    <div>

                    <h1>
                        { &course.name }
                        if let Some(credits) = course.credits {
                            {" "}<span class="badge text-bg-secondary">{ format!("{} CP", credits) }</span>
                        }
                    </h1>

                    <h2>{"Lehrende"}</h2>
                    <ul>
                    {
                        course.instructors.iter().map(|instructor| {
                            html!{
                                <li>{ &instructor.0 }</li>
                            }
                        }).collect::<Html>()
                    }
                    </ul>

                    <div>{ format!("Typ: {}", course.r#type) }</div>

                    <div>{ format!("Fachbereich: {}", course.fachbereich) }</div>

                    {
                        match (course.teilnehmer_min, course.teilnehmer_max) {
                            (None, None) => html! {
                            },
                            (None, Some(max)) => html! {
                                <div>{ format!("Maximal {max} Teilnehmende") }</div>
                            },
                            (Some(min), None) => html! {
                                <div>{ format!("Mindestens {min} Teilnehmende", ) }</div>
                            },
                            (Some(min), Some(max)) => html! {
                                <div>{ format!("{min} - {max} Teilnehmende", ) }</div>
                            }
                        }
                    }

                    <h2>{"Übungsgruppen"}</h2>
                    <table class="table">
                    <thead>
                    <tr>
                        <th scope="col">{"Name"}</th>
                        <th scope="col">{"Zeitraum"}</th>
                        <th scope="col">{"Uebungsleitende"}</th>
                    </tr>
                    </thead>
                    <tbody>
                    {
                        course.uebungsgruppen.iter().map(|uebungsgruppe| {
                            html!{
                                <tr>
                                    <th scope="row">{&uebungsgruppe.name}</th>
                                    <td>{uebungsgruppe.date_range.clone().unwrap_or_default()}</td>
                                    <td>{&uebungsgruppe.uebungsleiter}</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                    </tbody>
                    </table>

                    <h2>{"Anmeldefristen"}</h2>
                    <table class="table">
                    <thead>
                    <tr>
                        <th scope="col">{"Phase"}</th>
                        <th scope="col">{"Block"}</th>
                        <th scope="col">{"Start"}</th>
                        <th scope="col">{"Ende Anmeldung"}</th>
                        <th scope="col">{"Ende Abmeldung"}</th>
                        <th scope="col">{"Ende Hörer"}</th>
                    </tr>
                    </thead>
                    <tbody>
                    {
                        course.course_anmeldefristen.iter().map(|anmeldefrist| {
                            html!{
                                <tr>
                                    <td>{&anmeldefrist.zulassungstyp}</td>
                                    <td>{&anmeldefrist.block_type}</td>
                                    <td>{anmeldefrist.start.clone().unwrap_or_default()}</td>
                                    <td>{&anmeldefrist.ende_anmeldung.clone().unwrap_or_default()}</td>
                                    <td>{&anmeldefrist.ende_abmeldung.clone().unwrap_or_default()}</td>
                                    <td>{&anmeldefrist.ende_hoerer.clone().unwrap_or_default()}</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                    </tbody>
                    </table>

                    <h2>{"Termine"}</h2>
                    <table class="table">
                    <thead>
                    <tr>
                        <th scope="col">{"Datum"}</th>
                        <th scope="col">{"Start"}</th>
                        <th scope="col">{"Ende"}</th>
                        <th scope="col">{"Kursleitende"}</th>
                        <th scope="col">{"Räume"}</th>
                    </tr>
                    </thead>
                    <tbody>
                    {
                        course.termine.iter().map(|termin| {
                            html!{
                                <tr>
                                    <td>{&termin.date}</td>
                                    <td>{&termin.time_start}</td>
                                    <td>{&termin.time_end}</td>
                                    <td>{&termin.instructors.clone().unwrap_or_default()}</td>
                                    <td><ul>
                                    {
                                        termin.rooms.iter().map(|room| {
                                            html!{
                                                <li>{&room.name}</li>
                                            }
                                        }).collect::<Html>()
                                    }
                                    </ul></td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                    </tbody>
                    </table>

                    <h2>{"Beschreibung"}</h2>

                    // TODO FIXME this is dangerous
                    { Html::from_html_unchecked(course.description.join("\n").into()) }

                    <h2>{"Sonstige Informationen"}</h2>

                    <div>{ format!("Sprache: {}", course.language) }</div>

                    <div>{ format!("SWS: {}", course.sws.map(|v| v.to_string()).unwrap_or_default()) }</div>

                    if let Some(anzeige_im_stundenplan) = &course.anzeige_im_stundenplan {
                        <div>{ format!("Anzeige im Stundenplan: {}", anzeige_im_stundenplan) }</div>
                    }

                    <div>{ format!("Kurslevel: {}", course.courselevel) }</div>

                    <h2>{"Enhalten in Modulen"}</h2>
                    <ul>
                    {
                        course.enhalten_in_modulen.iter().map(|modul| {
                            html!{
                                <li>{modul}</li>
                            }
                        }).collect::<Html>()
                    }
                    </ul>

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
