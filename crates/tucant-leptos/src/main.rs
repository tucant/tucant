pub mod api_server;
pub mod common;
pub mod course_details;
pub mod course_results;
pub mod exam_results;
pub mod login_component;
pub mod logout_component;
pub mod mlsstart;
pub mod module_details;
pub mod my_courses;
pub mod my_documents;
pub mod my_exams;
pub mod my_modules;
pub mod my_semester_modules;
pub mod navbar;
pub mod navbar_logged_in;
pub mod navbar_logged_out;
pub mod rc_tucan_type;
pub mod registration;
pub mod root;
pub mod student_result;
pub mod tauri;
pub mod vv;

use std::sync::Arc;

use api_server::ApiServerTucan;
use course_details::CourseDetails;
use course_results::CourseResults;
use exam_results::ExamResults;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use log::Level;
use mlsstart::Mlsstart;
use module_details::ModuleDetails;
use my_courses::MyCourses;
use my_documents::MyDocuments;
use my_exams::MyExams;
use my_modules::MyModules;
use my_semester_modules::MySemesterModules;
use navbar::Navbar;
use navbar_logged_out::NavbarLoggedOut;
use registration::Registration;
use root::Root;
use tucant_types::{LoginResponse, SemesterId};
use vv::VorlesungsverzeichnisComponent;
use wasm_bindgen::prelude::wasm_bindgen;

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

async fn get_login_response() -> Option<LoginResponse> {
    #[cfg(feature = "direct")]
    if js_sys::Reflect::get(&js_sys::global(), &wasm_bindgen::JsValue::from_str("chrome")).is_ok() {
        return direct_login_response().await;
    }
    #[cfg(feature = "api")]
    {
        return api_login_response().await;
    }
    #[cfg(not(any(feature = "direct", feature = "api")))]
    panic!("must activate at least feature `direct` or `api`");
}

#[component]
fn App(login_response: Option<LoginResponse>) -> impl IntoView {
    provide_context(Arc::new(ApiServerTucan::new()));

    let (session, set_session) = signal(login_response);
    provide_context(session);

    view! {
        <Router>
            <Navbar set_session=set_session />
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=Root />
                <Route path=path!("/my-modules/:semester") view=|| view! { <MyModules /> } />
                <Route path=path!("/my-semester-modules/:semester") view=|| view! { <MySemesterModules /> } />
                <Route path=path!("/my-courses/:semester") view=|| view! { <MyCourses /> } />
                <Route path=path!("/module-details/:module-details") view=|| view! { <ModuleDetails /> } />
                <Route path=path!("/course-details/:course-details") view=|| view! { <CourseDetails /> } />
                <Route path=path!("/vv/:vv") view=|| view! { <VorlesungsverzeichnisComponent /> } />
                <Route path=path!("/registration/") view=|| view! { <Registration /> } />
                <Route path=path!("/registration/:registration") view=|| view! { <Registration /> } />
                <Route path=path!("/my-exams/:semester") view=|| view! { <MyExams /> } />
                <Route path=path!("/course-results/:semester") view=|| view! { <CourseResults /> } />
                <Route path=path!("/exam-results/:semester") view=|| view! { <ExamResults /> } />
                <Route path=path!("/my-documents") view=|| view! { <MyDocuments /> } />
                <Route path=path!("/overview") view=|| view! { <Mlsstart /> } />
                <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> } />
            </Routes>
        </Router>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);

    fn alert(s: &str);

    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}

#[wasm_bindgen(main)]
async fn main() {
    console_log::init_with_level(Level::Debug);
    // From https://github.com/rustwasm/console_error_panic_hook, licensed under MIT and Apache 2.0
    std::panic::set_hook(Box::new(|info| {
        let mut msg = info.to_string();
        msg.push_str("\n\nStack:\n\n");
        let e = Error::new();
        let stack = e.stack();
        msg.push_str(&stack);
        msg.push_str("\n\n");
        error(msg.clone());
        alert(msg.as_str());
    }));
    let login_response = get_login_response().await;
    leptos::mount::mount_to_body(|| view! { <App login_response=login_response /> })
}
