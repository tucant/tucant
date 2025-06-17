pub mod navbar;
pub mod navbar_logged_out;

use dioxus::prelude::*;
use tucant_types::{coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest, registration::AnmeldungRequest, vv::ActionRequest, SemesterId};

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
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
    RootRegistration,
    #[route("/overview")]
    Overview,
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
    MyDocuments,
    #[route("/student-result/:course_of_study")]
    StudentResult { course_of_study: String },
}

#[component]
fn Root() -> Element {
    rsx! { }
}

#[component]
fn NotFound() -> Element {
    rsx! { }
}

#[component]
fn ModuleDetails() -> Element {
    rsx! { }
}

#[component]
fn CourseDetails() -> Element {
    rsx! { }
}

#[component]
fn Registration() -> Element {
    rsx! { }
}

#[component]
fn RootRegistration() -> Element {
    rsx! { }
}

#[component]
fn Overview() -> Element {
    rsx! { }
}

#[component]
fn Vorlesungsverzeichnis() -> Element {
    rsx! { }
}

#[component]
fn MyModules() -> Element {
    rsx! { }
}

#[component]
fn MySemesterModules() -> Element {
    rsx! { }
}

#[component]
fn MyCourses() -> Element {
    rsx! { }
}

#[component]
fn MyExams() -> Element {
    rsx! { }
}

#[component]
fn ExamResults() -> Element {
    rsx! { }
}
#[component]
fn CourseResults() -> Element {
    rsx! { }
}

#[component]
fn MyDocuments() -> Element {
    rsx! { }
}

#[component]
fn StudentResult() -> Element {
    rsx! { }
}
