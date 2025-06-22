pub mod navbar;
pub mod navbar_logged_out;
pub mod login_component;
#[cfg(feature = "api")]
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
pub mod logout_component;

use std::ops::Deref;
use std::rc::Rc;

use dioxus::prelude::*;
use tucant_types::DynTucan;
use tucant_types::{coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest, registration::AnmeldungRequest, vv::ActionRequest, SemesterId, Tucan};

use crate::navbar::Navbar;
use crate::overview::Overview;

#[cfg(feature = "direct")]
pub async fn direct_login_response() -> Option<tucant_types::LoginResponse> {
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

    Some(tucant_types::LoginResponse { id: session_id.parse().unwrap(), cookie_cnsc: cnsc })
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
use crate::module_details::ModuleDetails;
use crate::course_details::CourseDetails;
use crate::registration::Registration;
use crate::my_modules::MyModules;
use crate::my_semester_modules::MySemesterModules;
use crate::my_courses::MyCourses;
use crate::my_exams::MyExams;
use crate::exam_results::ExamResults;
use crate::course_results::CourseResults;
use crate::my_documents::MyDocuments;
use crate::student_result::StudentResult;
use crate::vv::Vorlesungsverzeichnis;

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
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
    }
}

#[component]
pub fn Root() -> Element {
    rsx! {
        div { class: "container",
            h1 {
                { "Willkommen bei TUCaN't!" }
            }
            p {
                { "Du kannst gerne die " }
                a { href: "https://tucant.github.io/tucant/", target: "_blank",
                    { "Browsererweiterung herunterladen" }
                }
                { ", falls Du diese noch nicht verwendest." }
            }
            p {
                { "Der Quellcode dieses Projekts ist unter der AGPL-3.0 Lizenz auf " }
                a { href: "https://github.com/tucant/tucant/", target: "_blank",
                    { "GitHub" }
                }
                { " verfügbar." }
            }
            p {
                { "Du kannst Dir deine " }
                a { href: "#/registration/",
                    { "anmeldbaren Module ansehen" }
                }
                { "." }
            }
        }
    }
}

pub struct RcTucanType(pub Rc<DynTucan<'static>>);

impl Clone for RcTucanType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for RcTucanType {
    fn eq(&self, other: &RcTucanType) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Deref for RcTucanType {
    type Target = Rc<DynTucan<'static>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}