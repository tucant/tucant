pub mod navbar;
pub mod navbar_logged_out;
pub mod login_component;
pub mod api_server;
pub mod navbar_logged_in;
pub mod overview;
pub mod common;
pub mod course_details;
pub mod course_results;
pub mod exam_results;
pub mod module_details;
pub mod my_courses;
pub mod my_documents;
pub mod my_exams;
pub mod my_modules;
pub mod my_semester_modules;
pub mod registration;
pub mod student_result;
pub mod vv;

use dioxus::prelude::*;
use tucant_types::{coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest, registration::AnmeldungRequest, vv::ActionRequest, SemesterId, Tucan};

use crate::navbar::Navbar;
use crate::overview::Overview;

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
pub async fn api_login_response() -> Option<tucant_types::LoginResponse> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

    Some(tucant_types::LoginResponse {
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

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Root {},
    #[route("/:..route")]
    NotFound { route: Vec<String> },
    #[route("/module-details/:module")]
    ModuleDetails { module: ModuleDetailsRequest },
    #[route("/course-details/:course")]
    CourseDetails { course: CourseDetailsRequest },
    #[route("/registration/:registration")]
    Registration { registration: AnmeldungRequest },
    #[route("/registration/")]
    RootRegistration {},
    #[route("/overview")]
    Overview {},
    #[route("/vv/:vv")]
    Vorlesungsverzeichnis { vv: ActionRequest },
    #[route("/my-modules/:semester")]
    MyModules { semester: SemesterId },
    #[route("/my-semester-modules/:semester")]
    MySemesterModules { semester: SemesterId },
    #[route("/my-courses/:semester")]
    MyCourses { semester: SemesterId },
    #[route("/my-exams/:semester")]
    MyExams { semester: SemesterId },
    #[route("/exam-results/:semester")]
    ExamResults { semester: SemesterId },
    #[route("/course-results/:semester")]
    CourseResults { semester: SemesterId },
    #[route("/my-documents")]
    MyDocuments {},
    #[route("/student-result/:course_of_study")]
    StudentResult { course_of_study: String },
}

#[component]
fn Root() -> Element {
    rsx! { }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! { }
}

#[component]
fn ModuleDetails(module: ModuleDetailsRequest) -> Element {
    rsx! { }
}

#[component]
fn CourseDetails(course: CourseDetailsRequest) -> Element {
    rsx! { }
}

#[component]
fn Registration(registration: AnmeldungRequest) -> Element {
    rsx! { }
}

#[component]
fn RootRegistration() -> Element {
    rsx! { }
}

#[component]
fn Vorlesungsverzeichnis(vv: ActionRequest) -> Element {
    rsx! { }
}

#[component]
fn MyModules(semester: SemesterId) -> Element {
    rsx! { }
}

#[component]
fn MySemesterModules(semester: SemesterId) -> Element {
    rsx! { }
}

#[component]
fn MyCourses(semester: SemesterId) -> Element {
    rsx! { }
}

#[component]
fn MyExams(semester: SemesterId) -> Element {
    rsx! { }
}

#[component]
fn ExamResults(semester: SemesterId) -> Element {
    rsx! { }
}

#[component]
fn CourseResults(semester: SemesterId) -> Element {
    rsx! { }
}

#[component]
fn MyDocuments() -> Element {
    rsx! { }
}

#[component]
fn StudentResult(course_of_study: String) -> Element {
    rsx! { }
}
