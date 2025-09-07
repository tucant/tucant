use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness as _, embed_migrations};
use dioxus::prelude::{*, Element as DioxusElement};
use futures::FutureExt;
use js_sys::{ArrayBuffer, Uint8Array};
use log::info;
use sqlite_wasm_rs::{
    self as ffi,
    relaxed_idb_vfs::{RelaxedIdbCfg, install as install_idb_vfs},
};
use tucant_planning::{abc, decompress};
use tucant_types::{LoginResponse, RevalidationStrategy, Tucan as _};
use tucant_types::registration::AnmeldungResponse;
use wasm_bindgen_futures::JsFuture;
use web_sys::{FileList, FileReader, HtmlInputElement, console};

use crate::{MyRc, RcTucanType};
use crate::models::{Anmeldung, NewAnmeldung, Semester};
use crate::schema::anmeldungen_plan::{self, url};

// TODO at some point put opfs into a dedicated worker as that is the most
// correct approach TODO put this into a shared worker so there are no race
// conditions

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

async fn open_db() -> MyRc<RefCell<SqliteConnection>> {
    // install relaxed-idb persistent vfs and set as default vfs
    install_idb_vfs(&RelaxedIdbCfg::default(), true)
        .await
        .unwrap();

    let mut connection = SqliteConnection::establish("tucant.db").unwrap();
    connection.run_pending_migrations(MIGRATIONS).unwrap();
    MyRc(Arc::new(RefCell::new(connection)))
}

#[component]
pub fn Planning() -> Element {
    let connection = use_resource(move || async move { open_db().await });
    rsx! {
        if let Some(connection) = connection() {
            PlanningInner {
                connection
            }
        }
    }
}

async fn handle_semester(connection_clone: MyRc<RefCell<SqliteConnection>>, semester: Semester, element: Signal<Option<web_sys::Element>>) {
    use wasm_bindgen::JsCast;
    let element = element().unwrap();
    let b: HtmlInputElement = element.dyn_into::<HtmlInputElement>().unwrap();
    let files: FileList = b.files().unwrap();
    for i in 0..files.length() {
        let file = files.get(i).unwrap();
        let array_buffer = JsFuture::from(file.array_buffer()).await.unwrap();
        let array = Uint8Array::new(&array_buffer);
        let decompressed = decompress(&array.to_vec()).await.unwrap();
        let mut result: Vec<AnmeldungResponse> =
            serde_json::from_reader(decompressed.as_slice()).unwrap();
        result.sort_by_key(|e| e.path.len());
        let inserts: Vec<_> = result
            .iter()
            .map(|e| NewAnmeldung {
                url: e.path.last().unwrap().1.inner(),
                name: &e.path.last().unwrap().0,
                parent: e.path.len().checked_sub(2).map(|v| e.path[v].1.inner()),
                min_cp: 0,
                max_cp: None,
                min_modules: 0,
                max_modules: None,
            })
            .collect();
        let mut connection = connection_clone.borrow_mut();
        let connection = &mut *connection;
        let result = diesel::insert_into(anmeldungen_plan::table)
            .values(&inserts)
            .on_conflict((anmeldungen_plan::url))
            .do_update()
            .set(anmeldungen_plan::parent.eq(excluded(anmeldungen_plan::parent)))
            .execute(connection)
            .expect("Error saving anmeldungen");
    }
}

#[component]
pub fn PlanningInner(connection: MyRc<RefCell<SqliteConnection>>) -> Element {
    let mut sommersemester: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let mut wintersemester: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let connection_clone = connection.clone();
    let tucan: RcTucanType = use_context();
    let mut current_session_handle = use_context::<Signal<Option<LoginResponse>>>();
    let mut future = use_resource(move || {
        let connection_clone = connection_clone.clone();
        let current_session_handle = current_session_handle.clone();
        let tucan = tucan.clone();
        async move {
            let current_session = current_session_handle().unwrap();
            let student_result = tucan
                .student_result(&current_session, RevalidationStrategy::cache(), 0)
                .await.unwrap();

            // top level anmeldung has name "M.Sc. Informatik (2023)"
            // top level leistunggspiegel has "Informatik"

            let name = &student_result.course_of_study.iter().find(|e| e.selected).unwrap().name;
            let the_url: String = diesel::update(anmeldungen_plan::table.filter(anmeldungen_plan::name.eq(name)))
                .set((anmeldungen_plan::min_cp.eq(student_result.level0.rules.min_cp as i32),
                             anmeldungen_plan::max_cp.eq(student_result.level0.rules.max_cp.map(|v| v as i32)),
                             anmeldungen_plan::min_modules.eq(student_result.level0.rules.min_modules as i32),
                             anmeldungen_plan::max_modules.eq(student_result.level0.rules.max_modules.map(|v| v as i32))))
                .returning(anmeldungen_plan::url)
                .get_result(&mut *connection_clone.borrow_mut())
                .expect("Error updating anmeldungen");
            info!("updated");

            for child in student_result.level0.children {
                let name = &child.name.unwrap();
                diesel::update(anmeldungen_plan::table.filter(anmeldungen_plan::parent.eq(&the_url).and(anmeldungen_plan::name.eq(name))))
                    .set((anmeldungen_plan::min_cp.eq(child.rules.min_cp as i32),
                                anmeldungen_plan::max_cp.eq(child.rules.max_cp.map(|v| v as i32)),
                                anmeldungen_plan::min_modules.eq(child.rules.min_modules as i32),
                                anmeldungen_plan::max_modules.eq(child.rules.max_modules.map(|v| v as i32))))
                    .execute(&mut *connection_clone.borrow_mut())
                    .expect("Error updating anmeldungen");
                info!("updated");
            }


            let results: Vec<Anmeldung> = anmeldungen_plan::table
                .select(Anmeldung::as_select())
                .load(&mut *connection_clone.borrow_mut())
                .expect("Error loading anmeldungen");
            results
        }
    });
    let connection_clone = connection.clone();
    let onsubmit = move |evt: Event<FormData>| {
        let connection_clone = connection_clone.clone();
        evt.prevent_default();
        async move {
            handle_semester(connection_clone.clone(), Semester::Sommersemester, sommersemester).await;
            handle_semester(connection_clone, Semester::Wintersemester, wintersemester).await;
            info!("done");
            future.restart();
        }
    };
    rsx! {
        div { class: "container",
            h2 {
                class: "text-center",
                "Semesterplanung"
            }
            form {
                onsubmit: onsubmit,
                div {
                    class: "mb-3",
                    label {
                        for: "sommersemester-file",
                        class: "form-label",
                        "Sommersemester"
                    }
                    input {
                        type: "file",
                        class: "form-control",
                        id: "sommersemester-file",
                        onmounted: move |element| {
                            use dioxus::web::WebEventExt;
                            sommersemester.set(Some(element.as_web_event()))
                        }
                    }
                }
                div {
                    class: "mb-3",
                    label {
                        for: "wintersemester-file",
                        class: "form-label",
                        "Wintersemester"
                    }
                    input {
                        type: "file",
                        class: "form-control",
                        id: "wintersemester-file",
                        onmounted: move |element| {
                            use dioxus::web::WebEventExt;
                            wintersemester.set(Some(element.as_web_event()))
                        }
                    }
                }
                button {
                    type: "submit",
                    class: "btn btn-primary",
                    "Planung starten"
                }
            }
            ul {
                if let Some(value) = &*future.read() {
                    for entry in value {
                        li {
                            "{entry:?}"
                        }
                    }
                }
            }
        }
    }
}
