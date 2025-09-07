use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness as _, embed_migrations};
use dioxus::prelude::*;
use futures::FutureExt;
use js_sys::{ArrayBuffer, Uint8Array};
use log::info;
use sqlite_wasm_rs::{
    self as ffi,
    relaxed_idb_vfs::{RelaxedIdbCfg, install as install_idb_vfs},
};
use tucant_planning::{abc, decompress};
use tucant_types::registration::AnmeldungResponse;
use wasm_bindgen_futures::JsFuture;
use web_sys::{FileList, FileReader, HtmlInputElement, console};

use crate::MyRc;
use crate::models::{Anmeldung, NewAnmeldung};
use crate::schema::anmeldungen;

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

#[component]
pub fn PlanningInner(connection: MyRc<RefCell<SqliteConnection>>) -> Element {
    let mut sommersemester: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let mut wintersemester: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let connection_clone = connection.clone();
    let mut future = use_resource(move || {
        let connection_clone = connection_clone.clone();
        async move {
            use crate::schema::anmeldungen::dsl::anmeldungen;
            let results: Vec<Anmeldung> = anmeldungen
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
            use wasm_bindgen::JsCast;
            let a: web_sys::Element = sommersemester().unwrap();
            let b: HtmlInputElement = a.dyn_into::<HtmlInputElement>().unwrap();
            let files: FileList = b.files().unwrap();
            for i in 0..files.length() {
                let file = files.get(i).unwrap();
                info!("{}", file.name());
                let array_buffer = JsFuture::from(file.array_buffer()).await.unwrap();
                let array = Uint8Array::new(&array_buffer);
                let decompressed = decompress(&array.to_vec()).await.unwrap();
                let mut result: Vec<AnmeldungResponse> =
                    serde_json::from_reader(decompressed.as_slice()).unwrap();
                info!("{:?}", result);
                result.sort_by_key(|e| e.path.len());
                let inserts: Vec<_> = result
                    .iter()
                    .map(|e| NewAnmeldung {
                        url: e.path.last().unwrap().1.inner(),
                        name: &e.path.last().unwrap().0,
                        parent: None,
                    })
                    .collect();
                let mut connection = connection_clone.borrow_mut();
                let connection = &mut *connection;
                let result = diesel::insert_into(anmeldungen::table)
                    .values(&inserts)
                    .execute(connection)
                    .expect("Error saving anmeldungen");
            }
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
        }
    }
}
