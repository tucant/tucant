#[cfg(feature = "api")]
pub mod api_server;
pub mod common;
pub mod course_details;
pub mod course_results;
pub mod exam_results;
pub mod gradeoverview;
pub mod login_component;
pub mod logout_component;
pub mod module_details;
pub mod my_courses;
pub mod my_documents;
pub mod my_exams;
pub mod my_modules;
pub mod my_semester_modules;
pub mod navbar;
pub mod navbar_logged_in;
pub mod navbar_logged_out;
pub mod overview;
pub mod registration;
pub mod student_result;
pub mod vv;

use std::ops::Deref;
use std::sync::Arc;

use dioxus::prelude::*;
use tucant_types::gradeoverview::GradeOverviewRequest;
use tucant_types::DynTucan;
use tucant_types::{
    coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest,
    registration::AnmeldungRequest, vv::ActionRequest, SemesterId,
};

use crate::navbar::Navbar;
use crate::overview::Overview;

#[cfg(not(any(feature = "desktop", feature = "mobile", feature = "direct", feature = "api")))]
pub async fn login_response() -> Option<tucant_types::LoginResponse> {
    None
}

#[cfg(any(feature = "desktop", feature = "mobile"))]
pub async fn login_response() -> Option<tucant_types::LoginResponse> {
    #[cfg(feature = "mobile")]
    android_keyring::set_android_keyring_credential_builder().unwrap();

    let entry = keyring::Entry::new("tucant", "session").ok()?;
    Some(serde_json::from_str(&entry.get_password().ok()?).unwrap())
    //println!("My password is '{}'", password);
    //entry.set_password("topS3cr3tP4$$w0rd").ok()?;
    //println!("could set password");
    //None
}

#[cfg(feature = "direct")]
pub async fn login_response() -> Option<tucant_types::LoginResponse> {
    let session_id = web_extensions_sys::chrome()
        .cookies()
        .get(web_extensions_sys::CookieDetails {
            name: "id".to_owned(),
            partition_key: None,
            store_id: None,
            url: "https://www.tucan.tu-darmstadt.de/scripts".to_owned(),
        })
        .await?
        .value;

    let cnsc = web_extensions_sys::chrome()
        .cookies()
        .get(web_extensions_sys::CookieDetails {
            name: "cnsc".to_owned(),
            url: "https://www.tucan.tu-darmstadt.de/scripts".to_owned(),
            partition_key: None,
            store_id: None,
        })
        .await?
        .value;

    Some(tucant_types::LoginResponse {
        id: session_id.parse().unwrap(),
        cookie_cnsc: cnsc,
    })
}

#[cfg(feature = "api")]
pub async fn login_response() -> Option<tucant_types::LoginResponse> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

    Some(tucant_types::LoginResponse {
        id: cookie::Cookie::split_parse(&cookie)
            .find_map(|cookie| {
                let cookie = cookie.unwrap();
                if cookie.name() == "id" {
                    Some(cookie.value().to_string())
                } else {
                    None
                }
            })?
            .parse()
            .unwrap(),
        cookie_cnsc: cookie::Cookie::split_parse(&cookie).find_map(|cookie| {
            let cookie = cookie.unwrap();
            if cookie.name() == "cnsc" {
                Some(cookie.value().to_string())
            } else {
                None
            }
        })?,
    })
}
use crate::course_details::CourseDetails;
use crate::course_results::CourseResults;
use crate::exam_results::ExamResults;
use crate::gradeoverview::GradeOverview;
use crate::module_details::ModuleDetails;
use crate::my_courses::MyCourses;
use crate::my_documents::MyDocuments;
use crate::my_exams::MyExams;
use crate::my_modules::MyModules;
use crate::my_semester_modules::MySemesterModules;
use crate::registration::Registration;
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
    #[route("/gradeoverview/:gradeoverview")]
    GradeOverview { gradeoverview: GradeOverviewRequest },
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
            h1 { {"Willkommen bei TUCaN't!"} }
            p {
                {"Du kannst gerne die "}
                a {
                    href: "https://tucant.github.io/tucant/",
                    target: "_blank",
                    {"Browsererweiterung herunterladen"}
                }
                {", falls Du diese noch nicht verwendest."}
            }
            p {
                {"Der Quellcode dieses Projekts ist unter der AGPL-3.0 Lizenz auf "}
                a {
                    href: "https://github.com/tucant/tucant/",
                    target: "_blank",
                    {"GitHub"}
                }
                {" verfügbar."}
            }
            p {
                {"Du kannst Dir deine "}
                Link {
                    to: Route::Registration {
                        registration: AnmeldungRequest::default(),
                    },
                    {"anmeldbaren Module ansehen"}
                }
                {"."}
            }
        }
    }
}

pub struct RcTucanType(pub Arc<DynTucan<'static>>);

impl Clone for RcTucanType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for RcTucanType {
    fn eq(&self, other: &RcTucanType) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Deref for RcTucanType {
    type Target = Arc<DynTucan<'static>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
