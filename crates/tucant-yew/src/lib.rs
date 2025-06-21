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
