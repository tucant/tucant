pub mod navbar;
pub mod navbar_logged_out;
pub mod login_component;
pub mod rc_tucan_type;
pub mod api_server;
pub mod navbar_logged_in;
pub mod overview;
pub mod common;

use dioxus::prelude::*;
use tucant_types::{coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest, registration::AnmeldungRequest, vv::ActionRequest, SemesterId, Tucan};

use crate::navbar::Navbar;
use crate::overview::Overview;

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
