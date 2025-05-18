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

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/module-details/:module")]
    ModuleDetails { module: ModuleDetailsRequest },
    #[at("/course-details/:course")]
    CourseDetails { course: CourseDetailsRequest },
    #[at("/registration/:registration")]
    Registration { registration: AnmeldungRequest },
    #[at("/registration/")]
    RootRegistration,
    #[at("/overview")]
    Overview,
    #[at("/vv/:vv")]
    Vorlesungsverzeichnis { vv: ActionRequest },
    #[at("/my-modules/:semester")]
    MyModules { semester: SemesterId },
    #[at("/my-semester-modules/:semester")]
    MySemesterModules { semester: SemesterId },
    #[at("/my-courses/:semester")]
    MyCourses { semester: SemesterId },
    #[at("/my-exams/:semester")]
    MyExams { semester: SemesterId },
    #[at("/exam-results/:semester")]
    ExamResults { semester: SemesterId },
    #[at("/course-results/:semester")]
    CourseResults { semester: SemesterId },
    #[at("/my-documents")]
    MyDocuments,
    #[at("/student-result/:course_of_study")]
    StudentResult { course_of_study: String },
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
