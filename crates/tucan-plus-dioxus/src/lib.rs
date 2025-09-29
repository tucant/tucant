#[cfg(feature = "api")]
pub mod api_server;
pub mod common;
pub mod course_details;
pub mod course_results;
pub mod exam_results;
pub mod export_database;
pub mod export_semester;
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
pub mod planning;
pub mod registration;
pub mod student_result;
pub mod vv;

use crate::export_semester::FetchAnmeldung;
use crate::navbar::Navbar;
use crate::overview::Overview;
use crate::planning::Planning;
use dioxus::prelude::*;
use fragile::Fragile;
use log::info;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
#[cfg(target_arch = "wasm32")]
use web_sys::BroadcastChannel;
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use tucan_plus_worker::{RequestResponse, RequestResponseEnum};
use tucan_types::DynTucan;
use tucan_types::gradeoverview::GradeOverviewRequest;
use tucan_types::{
    SemesterId, coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest,
    registration::AnmeldungRequest, vv::ActionRequest,
};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::{AddEventListenerOptions, MessageEvent, WorkerOptions, WorkerType};

#[used]
pub static BOOTSTRAP_CSS: Asset = asset!(
    "/assets/bootstrap.css",
    AssetOptions::builder().with_hash_suffix(false)
);

#[used]
pub static APP_MANIFEST: Asset = asset!(
    "/assets/manifest.json",
    AssetOptions::builder().with_hash_suffix(false)
);

#[used]
pub static LOGO_SVG: Asset = asset!(
    "/assets/logo.svg",
    AssetOptions::builder().with_hash_suffix(false)
);

// unfortunately the asset! macro produces a different hash as the build output
#[cfg(target_arch = "wasm32")]
#[used]
pub static WORKER_JS: Asset = asset!(
    env!("WORKER_JS_PATH"),
    AssetOptions::builder().with_hash_suffix(false)
);

// unfortunately the asset! macro produces a different hash as the build output
#[cfg(target_arch = "wasm32")]
#[used]
pub static WORKER_WASM: Asset = asset!(
    env!("WORKER_WASM_PATH"),
    AssetOptions::builder().with_hash_suffix(false)
);

#[cfg(target_arch = "wasm32")]
#[used]
pub static SERVICE_WORKER_JS: Asset = asset!(
    env!("SERVICE_WORKER_JS_PATH"),
    AssetOptions::builder().with_hash_suffix(false)
);

pub static BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js",);

pub static BOOTSTRAP_PATCH_JS: Asset = asset!("/assets/bootstrap.patch.js",);

#[derive(Copy, Clone)]
pub struct Anonymize(pub bool);

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct MyDatabase {
    broadcast_channel: BroadcastChannel,
    worker: Option<Fragile<web_sys::Worker>>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageWithId {
    id: String,
    message: RequestResponseEnum,
}

#[cfg(target_arch = "wasm32")]
impl MyDatabase {
    async fn send_message_internal<R: RequestResponse + Debug>(&self, message: R) -> R::Response {
        use rand::{Rng, distr::{Alphanumeric, SampleString as _}};

        let id = Alphanumeric.sample_string(&mut rand::rng(), 16);
       
        // worker in local 
        if let Some(worker) = self.worker {
            worker.get().post_message(&serde_wasm_bindgen::to_value(&MessageWithId {
                id,
                message: RequestResponseEnum::from(message)
            }).unwrap());
        }
        // try temporarily creating a broadcast channel?

        let temporary_broadcast_channel = BroadcastChannel::new(&id).unwrap();

        let temporary_message_closure: Closure<dyn Fn(_)> = {
            Closure::new(move |event: web_sys::MessageEvent| {
                
            })
        };
        temporary_broadcast_channel.add_event_listener_with_callback("message", temporary_message_closure.into_js_value())

    }

    pub async fn wait_for_worker() -> Self {
        use js_sys::Promise;

        let lock_manager = web_sys::window().unwrap().navigator().locks();
        let lock_closure: Closure<dyn Fn(_) -> Promise> = {
            Closure::new(move |event: web_sys::Lock| {
                // returning a promise
                let mut cb = |resolve: js_sys::Function, reject: js_sys::Function| {
                    let options = WorkerOptions::new();
                    options.set_type(WorkerType::Module);
                    let worker =
                        web_sys::Worker::new_with_options(&WORKER_JS.to_string(), &options).unwrap();
                    let message_closure: Rc<RefCell<Option<Closure<dyn Fn(MessageEvent)>>>> =
                        Rc::new(RefCell::new(None));
                    let error_closure: Closure<dyn Fn(_)> = {
                        let worker = worker.clone();
                        let message_closure = message_closure.clone();
                        Closure::new(move |event: web_sys::ErrorEvent| {
                            info!(
                                "error at client {event:?} {:?} {:?}",
                                event.message(),
                                event.error()
                            );
                            worker
                                .remove_event_listener_with_callback(
                                    "message",
                                    message_closure
                                        .borrow()
                                        .as_ref()
                                        .unwrap()
                                        .as_ref()
                                        .unchecked_ref(),
                                )
                                .unwrap();
                            reject.call0(&JsValue::undefined()).unwrap();
                        })
                    };
                    let error_closure_ref = error_closure.as_ref().clone();
                    *message_closure.borrow_mut() = {
                        let worker = worker.clone();
                        let error_closure_ref = error_closure_ref.clone();
                        Some(Closure::new(move |event: MessageEvent| {
                            //info!("received message at client {:?}", event.data());
                            worker
                                .remove_event_listener_with_callback(
                                    "error",
                                    error_closure_ref.unchecked_ref(),
                                )
                                .unwrap();
                            resolve.call1(&JsValue::undefined(), &worker).unwrap();
                        }))
                    };
                    let options = AddEventListenerOptions::new();
                    options.set_once(true);
                    worker
                        .add_event_listener_with_callback_and_add_event_listener_options(
                            "error",
                            error_closure_ref.unchecked_ref(),
                            &options,
                        )
                        .unwrap();
                    worker
                        .add_event_listener_with_callback_and_add_event_listener_options(
                            "message",
                            message_closure
                                .borrow()
                                .as_ref()
                                .unwrap()
                                .as_ref()
                                .unchecked_ref(),
                            &options,
                        )
                        .unwrap();
                    error_closure.forget();
                };

                return js_sys::Promise::new(&mut cb);
            })
        };
        let promise = lock_manager.request_with_callback("dedicated-worker-lock", lock_closure.as_ref().unchecked_ref());

        Self
    }

    pub async fn send_message<R: RequestResponse + Debug>(&self, value: R) -> R::Response
    {
        //info!("sending message from client {:?}", value);
        let mut cb = |resolve: js_sys::Function, reject: js_sys::Function| {
            let message_closure: Rc<RefCell<Option<Closure<dyn Fn(MessageEvent)>>>> =
                Rc::new(RefCell::new(None));
            let error_closure: Closure<dyn Fn(_)> = {
                let worker = self.worker.clone();
                let message_closure = message_closure.clone();
                Closure::new(move |event: web_sys::ErrorEvent| {
                    info!(
                        "error at client {event:?} {:?} {:?}",
                        event.message(),
                        event.error()
                    );
                    worker
                        .get()
                        .remove_event_listener_with_callback(
                            "message",
                            message_closure
                                .borrow()
                                .as_ref()
                                .unwrap()
                                .as_ref()
                                .unchecked_ref(),
                        )
                        .unwrap();
                    reject.call0(&JsValue::undefined()).unwrap();
                })
            };
            let error_closure_ref = error_closure.as_ref().clone();
            *message_closure.borrow_mut() = {
                let worker = self.worker.clone();
                let error_closure_ref = error_closure_ref.clone();
                Some(Closure::new(move |event: MessageEvent| {
                    //info!("received message at client {:?}", event.data());
                    worker
                        .get()
                        .remove_event_listener_with_callback(
                            "error",
                            error_closure_ref.unchecked_ref(),
                        )
                        .unwrap();
                    resolve.call1(&JsValue::undefined(), &event.data()).unwrap();
                }))
            };
            let options = AddEventListenerOptions::new();
            options.set_once(true);
            self.0
                .get()
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "error",
                    error_closure_ref.unchecked_ref(),
                    &options,
                )
                .unwrap();
            self.0
                .get()
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "message",
                    message_closure
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                    &options,
                )
                .unwrap();
            error_closure.forget();
        };

        let p = js_sys::Promise::new(&mut cb);

        self.0
            .get()
            .post_message(&serde_wasm_bindgen::to_value(&RequestResponseEnum::from(value)).unwrap())
            .unwrap();

        serde_wasm_bindgen::from_value(wasm_bindgen_futures::JsFuture::from(p).await.unwrap())
            .unwrap()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use tucan_plus_worker::MyDatabase;

#[cfg(not(any(
    feature = "desktop",
    feature = "mobile",
    feature = "direct",
    feature = "api"
)))]
pub async fn login_response() -> Option<tucan_types::LoginResponse> {
    None
}

#[cfg(any(feature = "desktop", feature = "mobile"))]
pub async fn login_response() -> Option<tucan_types::LoginResponse> {
    #[cfg(feature = "mobile")]
    keyring_core::set_default_store(
        android_native_keyring_store::AndroidStore::from_ndk_context().unwrap(),
    );

    #[cfg(feature = "desktop")]
    keyring_core::set_default_store(dbus_secret_service_keyring_store::Store::new().unwrap());

    let entry = keyring_core::Entry::new("tucan-plus", "session").ok()?;
    Some(serde_json::from_str(&entry.get_password().ok()?).unwrap())
    //println!("My password is '{}'", password);
    //entry.set_password("topS3cr3tP4$$w0rd").ok()?;
    //println!("could set password");
    //None
}

#[cfg(feature = "direct")]
pub async fn login_response() -> Option<tucan_types::LoginResponse> {
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

    Some(tucan_types::LoginResponse {
        id: session_id.parse().unwrap(),
        cookie_cnsc: cnsc,
    })
}

#[cfg(feature = "api")]
pub async fn login_response() -> Option<tucan_types::LoginResponse> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

    Some(tucan_types::LoginResponse {
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
use crate::export_database::ExportDatabase;
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
    #[route("/fetch-anmeldung")]
    FetchAnmeldung {},
    #[route("/planning/:course_of_study")]
    Planning { course_of_study: String },
    #[route("/export-database")]
    ExportDatabase {},
}

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 {
            "Page not found"
        }
    }
}

#[component]
pub fn Root() -> Element {
    rsx! {
        div {
            class: "container",
            h1 {
                { "Willkommen bei TUCaN Plus!" }
            }
            p {
                { "Du kannst gerne die " }
                a {
                    href: "https://tucan-plus.github.io/tucan-plus/",
                    target: "_blank",
                    { "Browsererweiterung herunterladen" }
                }
                { ", falls Du diese noch nicht verwendest." }
            }
            p {
                { "Der Quellcode dieses Projekts ist unter der AGPL-3.0 Lizenz auf " }
                a {
                    href: "https://github.com/tucan-plus/tucan-plus/",
                    target: "_blank",
                    { "GitHub" }
                }
                { " verfügbar." }
            }
            p {
                { "Du kannst Dir deine " }
                Link {
                    to: Route::Registration {
                        registration: AnmeldungRequest::default(),
                    },
                    { "anmeldbaren Module ansehen" }
                }
                { "." }
            }
            p {
                "Version "
                { git_version::git_version!() }
            }
        }
    }
}

pub struct MyRc<T: ?Sized>(pub Arc<T>);

impl<T: ?Sized> MyRc<T> {
    pub fn new(value: Arc<T>) -> Self {
        Self(value)
    }
}

impl<T: ?Sized> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> PartialEq for MyRc<T> {
    fn eq(&self, other: &MyRc<T>) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: ?Sized> Deref for MyRc<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type RcTucanType = MyRc<DynTucan<'static>>;
