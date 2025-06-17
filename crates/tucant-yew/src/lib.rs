use course_details::CourseDetails;
use course_results::CourseResults;
use exam_results::ExamResults;
use mlsstart::Mlsstart;
use module_details::ModuleDetails;
use my_courses::MyCourses;
use my_documents::MyDocuments;
use my_exams::MyExams;
use my_modules::MyModules;
use my_semester_modules::MySemesterModules;
use navbar::Navbar;
use registration::Registration;
use std::rc::Rc;
use student_result::StudentResult;
use tucant_types::{LoginRequest, LoginResponse, SemesterId, Tucan, coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest, registration::AnmeldungRequest, vv::ActionRequest};
use vv::VorlesungsverzeichnisComponent;

use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{HashRouter, Routable, Switch};

pub mod navbar;
pub mod navbar_logged_in;
pub mod navbar_logged_out;

pub mod api_server;
pub mod common;
pub mod course_details;
pub mod course_results;
pub mod exam_results;
pub mod mlsstart;
pub mod module_details;
pub mod my_courses;
pub mod my_documents;
pub mod my_exams;
pub mod my_modules;
pub mod my_semester_modules;
pub mod registration;
pub mod student_result;
pub mod tauri;
pub mod vv;

#[cfg(feature = "direct")]
pub async fn direct_login_response() -> Option<LoginResponse> {
    let session_id = web_extensions_sys::chrome()
        .cookies()
        .get(web_extensions_sys::CookieDetails {
            name: "id".to_owned(),
            partition_key: None,
            store_id: None,
            url: "https://www.tucan.tu-darmstadt.de/scripts/".to_owned(),
        })
        .await?
        .value;

    let cnsc = web_extensions_sys::chrome()
        .cookies()
        .get(web_extensions_sys::CookieDetails {
            name: "cnsc".to_owned(),
            url: "https://www.tucan.tu-darmstadt.de/scripts/".to_owned(),
            partition_key: None,
            store_id: None,
        })
        .await?
        .value;

    Some(LoginResponse { id: session_id.parse().unwrap(), cookie_cnsc: cnsc })
}

#[cfg(feature = "api")]
pub async fn api_login_response() -> Option<LoginResponse> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

    Some(LoginResponse {
        id: cookie::Cookie::split_parse(&cookie)
            .find_map(|cookie| {
                let cookie = cookie.unwrap();
                if cookie.name() == "id" { Some(cookie.value().to_string()) } else { None }
            })?
            .parse()
            .unwrap(),
        cookie_cnsc: cookie::Cookie::split_parse(&cookie).find_map(|cookie| {
            let cookie = cookie.unwrap();
            if cookie.name() == "cnsc" { Some(cookie.value().to_string()) } else { None }
        })?,
    })
}

#[function_component(LoginComponent)]
fn login<TucanType: Tucan + 'static>() -> HtmlResult {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let username_value_handle = use_state(String::default);

    let on_username_change = {
        let username_value_handle = username_value_handle.clone();

        Callback::from(move |e: Event| {
            username_value_handle.set(e.target_dyn_into::<HtmlInputElement>().unwrap().value());
        })
    };

    let password_value_handle = use_state(String::default);

    let on_password_change = {
        let password_value_handle = password_value_handle.clone();

        Callback::from(move |e: Event| {
            password_value_handle.set(e.target_dyn_into::<HtmlInputElement>().unwrap().value());
        })
    };

    let current_session = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");

    let on_submit = {
        let username_value_handle = username_value_handle.clone();
        let password_value_handle = password_value_handle.clone();
        let current_session = current_session.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username_value_handle).clone();
            let password = (*password_value_handle).clone();
            let current_session = current_session.clone();
            password_value_handle.set("".to_owned());

            let tucan = tucan.clone();

            spawn_local(async move {
                let response = tucan.0.login(LoginRequest { username, password }).await.unwrap();

                #[cfg(feature = "direct")]
                web_extensions_sys::chrome()
                    .cookies()
                    .set(web_extensions_sys::SetCookieDetails {
                        name: Some("id".to_owned()),
                        partition_key: None,
                        store_id: None,
                        url: "https://www.tucan.tu-darmstadt.de/scripts/".to_owned(),
                        domain: None,
                        path: None,
                        value: Some(response.id.to_string()),
                        expiration_date: None,
                        http_only: None,
                        secure: Some(true),
                        same_site: None,
                    })
                    .await;

                current_session.set(Some(response.clone()));
            })
        })
    };

    Ok(html! {
        <form onsubmit={on_submit} class="d-flex">
            <input id="login-username" onchange={on_username_change} value={(*username_value_handle).clone()} required=true class="form-control me-2" type="username" placeholder="TU-ID" aria-label="TU-ID" autocomplete="current-username" />
            <input id="login-password" onchange={on_password_change} value={(*password_value_handle).clone()} required=true class="form-control me-2" type="password" placeholder="Password" aria-label="Password" autocomplete="current-password" />
            <button class="btn btn-outline-success" type="submit" id="login-button">{ "Login" }</button>
        </form>
    })
}

#[function_component(LogoutComponent)]
fn logout<TucanType: Tucan + 'static>() -> HtmlResult {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let current_session_handle = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");

    let on_submit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let current_session_handle = current_session_handle.clone();
            let tucan = tucan.clone();

            if let Some(current_session) = (*current_session_handle).to_owned() {
                spawn_local(async move {
                    tucan.0.logout(&current_session).await.unwrap();

                    current_session_handle.set(None);
                });
            }
        })
    };

    Ok(html! {
        <form onsubmit={on_submit} class="d-flex">
            <button id="logout-button" class="btn btn-outline-success" type="submit">{ "Logout" }</button>
        </form>
    })
}

fn switch<TucanType: Tucan + 'static>(routes: Route) -> Html {
    match routes {
        Route::Registration { registration } => {
            ::yew::html! {
                <Registration<TucanType> registration={registration} />
            }
        }
        Route::RootRegistration => {
            ::yew::html! {
                <Registration<TucanType> registration={AnmeldungRequest::default()} />
            }
        }
        Route::NotFound => ::yew::html! {
            <div>
                { "404" }
            </div>
        },
        Route::Root => ::yew::html! {
            <div class="container">
                <h1>
                    { "Willkommen bei TUCaN't!" }
                </h1>
                <p>
                    { "Du kannst gerne die " }
                    <a href="https://tucant.github.io/tucant/" target="_blank">
                        { "Browsererweiterung herunterladen" }
                    </a>
                    { ", falls Du diese noch nicht verwendest." }
                </p>
                <p>
                    { "Der Quellcode dieses Projekts ist unter der AGPL-3.0 Lizenz auf " }
                    <a href="https://github.com/tucant/tucant/" target="_blank">
                        { "GitHub" }
                    </a>
                    { " verfügbar." }
                </p>
                <p>
                    { "Du kannst Dir deine " }
                    <a href="#/registration/">
                        { "anmeldbaren Module ansehen" }
                    </a>
                    { "." }
                </p>
            </div>
        },
        Route::ModuleDetails { module } => {
            ::yew::html! {
                <ModuleDetails<TucanType> module_details={module} />
            }
        }
        Route::CourseDetails { course } => {
            ::yew::html! {
                <CourseDetails<TucanType> course_details={course} />
            }
        }
        Route::Overview => {
            ::yew::html! {
                <Mlsstart<TucanType> />
            }
        }
        Route::Vorlesungsverzeichnis { vv } => {
            ::yew::html! {
                <VorlesungsverzeichnisComponent<TucanType> vv={vv} />
            }
        }
        Route::MyModules { semester } => {
            ::yew::html! {
                <MyModules<TucanType> semester={semester} />
            }
        }
        Route::MySemesterModules { semester } => {
            ::yew::html! {
                <MySemesterModules<TucanType> semester={semester} />
            }
        }
        Route::MyCourses { semester } => {
            ::yew::html! {
                <MyCourses<TucanType> semester={semester} />
            }
        }
        Route::MyExams { semester } => {
            ::yew::html! {
                <MyExams<TucanType> semester={semester} />
            }
        }
        Route::ExamResults { semester } => {
            ::yew::html! {
                <ExamResults<TucanType> semester={semester} />
            }
        }
        Route::CourseResults { semester } => {
            ::yew::html! {
                <CourseResults<TucanType> semester={semester} />
            }
        }
        Route::MyDocuments => {
            ::yew::html! {
                <MyDocuments<TucanType> />
            }
        }
        Route::StudentResult { course_of_study } => {
            ::yew::html! {
                <StudentResult<TucanType> course_of_study={course_of_study} />
            }
        }
    }
}

#[derive(Properties)]
pub struct AppProps<TucanType: Tucan + 'static> {
    pub initial_session: Option<LoginResponse>,
    pub tucan: RcTucanType<TucanType>,
}

impl<TucanType: Tucan + 'static> PartialEq for AppProps<TucanType> {
    fn eq(&self, other: &Self) -> bool {
        self.initial_session == other.initial_session && self.tucan == other.tucan
    }
}

#[function_component(App)]
pub fn app<TucanType: Tucan + 'static>(AppProps { initial_session, tucan }: &AppProps<TucanType>) -> HtmlResult {
    let ctx = use_state(|| initial_session.clone());

    Ok(html! {
        <>
            <ContextProvider<RcTucanType<TucanType>> context={tucan.clone()}>
                <ContextProvider<UseStateHandle<Option<LoginResponse>>> context={ctx.clone()}>
                    <HashRouter>
                        <Navbar<TucanType> />
                        <Switch<Route> render={switch::<TucanType>} />
                    </HashRouter>
                </ContextProvider<UseStateHandle<Option<LoginResponse>>>>
            </ContextProvider<RcTucanType<TucanType>>>
        </>
    })
}

pub struct RcTucanType<TucanType: Tucan + 'static>(pub Rc<TucanType>);

impl<TucanType: Tucan + 'static> Clone for RcTucanType<TucanType> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<TucanType: Tucan + 'static> PartialEq for RcTucanType<TucanType> {
    fn eq(&self, other: &RcTucanType<TucanType>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
