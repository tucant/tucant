use std::cell::RefCell;
use std::sync::Arc;

use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use diesel::upsert::excluded;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness as _, embed_migrations};
use dioxus::prelude::*;
use js_sys::Uint8Array;
use log::info;
use sqlite_wasm_rs::relaxed_idb_vfs::{RelaxedIdbCfg, install as install_idb_vfs};
use tucant_planning::decompress;
use tucant_types::registration::AnmeldungResponse;
use tucant_types::student_result::StudentResultLevel;
use tucant_types::{LoginResponse, RevalidationStrategy, Tucan as _};
use wasm_bindgen_futures::JsFuture;
use web_sys::{FileList, HtmlInputElement};

use crate::models::{Anmeldung, AnmeldungEntry, NewAnmeldung, NewAnmeldungEntry, Semester, State};
use crate::schema::{anmeldungen_entries, anmeldungen_plan};
use crate::{MyRc, RcTucanType};

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
                connection,
            }
        }
    }
}

async fn handle_semester(
    connection_clone: MyRc<RefCell<SqliteConnection>>,
    semester: Semester,
    element: Signal<Option<web_sys::Element>>,
) {
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
        diesel::insert_into(anmeldungen_plan::table)
            .values(&inserts)
            .on_conflict((anmeldungen_plan::url))
            .do_update()
            .set(anmeldungen_plan::parent.eq(excluded(anmeldungen_plan::parent)))
            .execute(connection)
            .expect("Error saving anmeldungen");
        let inserts: Vec<NewAnmeldungEntry> = result
            .iter()
            .flat_map(|anmeldung| {
                anmeldung.entries.iter().map(|entry| NewAnmeldungEntry {
                    semester,
                    anmeldung: anmeldung.path.last().unwrap().1.inner(),
                    module_url: entry.module.as_ref().unwrap().url.inner(),
                    id: &entry.module.as_ref().unwrap().id,
                    name: &entry.module.as_ref().unwrap().name,
                    credits: 42, // TODO fetch
                    state: State::NotPlanned,
                })
            })
            .collect();
        diesel::insert_into(anmeldungen_entries::table)
            .values(&inserts)
            .on_conflict((
                anmeldungen_entries::anmeldung,
                anmeldungen_entries::semester,
                anmeldungen_entries::id,
            ))
            .do_update()
            .set(anmeldungen_entries::state.eq(excluded(anmeldungen_entries::state)))
            .execute(connection)
            .expect("Error saving anmeldungen");
    }
}

pub async fn recursive_update(
    connection_clone: MyRc<RefCell<SqliteConnection>>,
    url: String,
    level: StudentResultLevel,
) {
    for child in level.children {
        let name = child.name.as_ref().unwrap();
        let child_url = diesel::update(QueryDsl::filter(
            anmeldungen_plan::table,
            anmeldungen_plan::parent
                .eq(&url)
                .and(anmeldungen_plan::name.eq(name)),
        ))
        .set((
            anmeldungen_plan::min_cp.eq(child.rules.min_cp as i32),
            anmeldungen_plan::max_cp.eq(child.rules.max_cp.map(|v| v as i32)),
            anmeldungen_plan::min_modules.eq(child.rules.min_modules as i32),
            anmeldungen_plan::max_modules.eq(child.rules.max_modules.map(|v| v as i32)),
        ))
        .returning(anmeldungen_plan::url)
        .get_result(&mut *connection_clone.borrow_mut())
        .expect("Error updating anmeldungen");
        info!("updated");
        Box::pin(recursive_update(connection_clone.clone(), child_url, child)).await;
    }
    let inserts: Vec<_> = level
        .entries
        .iter()
        .map(|entry| NewAnmeldungEntry {
            semester: Semester::Sommersemester, // TODO FIXME
            anmeldung: &url,
            module_url: "TODO", // TODO FIXME
            id: entry.id.as_ref().unwrap_or_else(|| &entry.name), /* TODO FIXME, use two columns
                                 * and both as primary key */
            credits: i32::try_from(entry.used_cp.unwrap_or_default()).unwrap(),
            name: &entry.name,
            state: State::Done,
        })
        .collect();
    diesel::insert_into(anmeldungen_entries::table)
        .values(&inserts)
        .on_conflict((
            anmeldungen_entries::anmeldung,
            anmeldungen_entries::semester,
            anmeldungen_entries::id,
        ))
        .do_update()
        .set(anmeldungen_entries::state.eq(excluded(anmeldungen_entries::state)))
        .execute(&mut *connection_clone.borrow_mut())
        .expect("Error saving anmeldungen");
}

#[component]
pub fn PlanningInner(connection: MyRc<RefCell<SqliteConnection>>) -> Element {
    let mut sommersemester: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let mut wintersemester: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let connection_clone = connection.clone();
    let tucan: RcTucanType = use_context();
    let mut current_session_handle = use_context::<Signal<Option<LoginResponse>>>();
    let mut future = {
        let connection_clone = connection_clone.clone();
        let current_session_handle = current_session_handle.clone();
        let tucan = tucan.clone();
        use_resource(move || {
            let connection_clone = connection_clone.clone();
            let current_session_handle = current_session_handle.clone();
            let tucan = tucan.clone();
            async move {
                let results: Vec<Anmeldung> =
                    QueryDsl::filter(anmeldungen_plan::table, anmeldungen_plan::parent.is_null())
                        .select(Anmeldung::as_select())
                        .load(&mut *connection_clone.borrow_mut())
                        .expect("Error loading anmeldungen");
                results
            }
        })
    };
    let mut load_leistungsspiegel = {
        let connection_clone = connection_clone.clone();
        let current_session_handle = current_session_handle.clone();
        let tucan = tucan.clone();
        move |evt: Event<MouseData>| {
            let connection_clone = connection_clone.clone();
            let current_session_handle = current_session_handle.clone();
            let tucan = tucan.clone();
            async move {
                let current_session = current_session_handle().unwrap();
                let student_result = tucan
                    .student_result(&current_session, RevalidationStrategy::cache(), 0)
                    .await
                    .unwrap();

                // top level anmeldung has name "M.Sc. Informatik (2023)"
                // top level leistunggspiegel has "Informatik"

                let name = &student_result
                    .course_of_study
                    .iter()
                    .find(|e| e.selected)
                    .unwrap()
                    .name;
                let the_url: String = diesel::update(QueryDsl::filter(
                    anmeldungen_plan::table,
                    anmeldungen_plan::name.eq(name),
                ))
                .set((
                    anmeldungen_plan::min_cp.eq(student_result.level0.rules.min_cp as i32),
                    anmeldungen_plan::max_cp.eq(student_result
                        .level0
                        .rules
                        .max_cp
                        .map(|v| v as i32)),
                    anmeldungen_plan::min_modules
                        .eq(student_result.level0.rules.min_modules as i32),
                    anmeldungen_plan::max_modules.eq(student_result
                        .level0
                        .rules
                        .max_modules
                        .map(|v| v as i32)),
                ))
                .returning(anmeldungen_plan::url)
                .get_result(&mut *connection_clone.borrow_mut())
                .expect("Error updating anmeldungen");

                recursive_update(connection_clone.clone(), the_url, student_result.level0).await;

                info!("updated");
                future.restart();
            }
        }
    };
    let connection_clone = connection.clone();
    let onsubmit = move |evt: Event<FormData>| {
        let connection_clone = connection_clone.clone();
        evt.prevent_default();
        async move {
            handle_semester(
                connection_clone.clone(),
                Semester::Sommersemester,
                sommersemester,
            )
            .await;
            handle_semester(connection_clone, Semester::Wintersemester, wintersemester).await;
            info!("done");
            future.restart();
        }
    };
    rsx! {
        div {
            class: "container",
            h2 {
                class: "text-center",
                "Semesterplanung"
            }
            form {
                onsubmit: onsubmit,
                class: "mb-3",
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
                        },
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
                        },
                    }
                }
                button {
                    type: "submit",
                    class: "btn btn-primary",
                    "Planung starten"
                }
            }
            button {
                type: "button",
                class: "btn btn-primary",
                onclick: load_leistungsspiegel,
                "Leistungsspiegel laden (nach Laden der Semester)"
            }
            if let Some(value) = &*future.read() {
                for entry in value {
                    PlanningAnmeldung {
                        connection: connection.clone(),
                        anmeldung: entry.clone(),
                        depth: 1,
                    }
                }
            }
        }
    }
}

fn prep_planning(
    connection: MyRc<RefCell<SqliteConnection>>,
    anmeldung: Anmeldung,
    depth: i32,
) -> (bool, Element) {
    let results: Vec<Anmeldung> = QueryDsl::filter(
        anmeldungen_plan::table,
        anmeldungen_plan::parent.eq(&anmeldung.url),
    )
    .select(Anmeldung::as_select())
    .load(&mut *connection.borrow_mut())
    .expect("Error loading anmeldungen");
    let entries: Vec<AnmeldungEntry> = QueryDsl::filter(
        anmeldungen_entries::table,
        anmeldungen_entries::anmeldung.eq(&anmeldung.url),
    )
    .select(AnmeldungEntry::as_select())
    .load(&mut *connection.borrow_mut())
    .expect("Error loading anmeldungen");
    let inner: Vec<(bool, Element)> = results
        .into_iter()
        .map(|result| prep_planning(connection.clone(), result, depth + 1))
        .collect();
    let has_rules = anmeldung.min_cp != 0
        || anmeldung.max_cp.is_some()
        || anmeldung.min_modules != 0
        || anmeldung.max_modules.is_some();
    let interesting = has_rules || !entries.is_empty() || inner.iter().any(|v| v.0);
    (
        interesting,
        rsx! {
            p {
                class: "h3",
                { anmeldung.name.clone() }
            }
            div {
                class: "ms-2 ps-2",
                style: "border-left: 1px solid #ccc;",
                if !entries.is_empty() {
                    table {
                        class: "table",
                        tbody {
                            for entry in entries {
                                tr {
                                    td {
                                        { entry.id }
                                    }
                                    td {
                                        { entry.name }
                                    }
                                    td {
                                        { entry.credits.to_string() }
                                    }
                                    td {
                                        { format!("{:?}", entry.state) }
                                    }
                                }
                            }
                        }
                    }
                }
                if inner.iter().any(|v| v.0) {
                    for inner in inner {
                        { inner.1 }
                    }
                }
                if has_rules {
                    p {
                        { "Summe ".to_owned() + &anmeldung.name + ":" }
                        br {
                        }
                        if anmeldung.min_cp != 0 || anmeldung.max_cp.is_some() {
                            "CP: "
                            { anmeldung.min_cp.to_string() }
                            {
                                anmeldung
                                    .max_cp
                                    .map(|max_cp| " - ".to_string() + &max_cp.to_string())
                            }
                        }
                        if (anmeldung.min_cp != 0 || anmeldung.max_cp.is_some())
                            && (anmeldung.min_modules != 0 || anmeldung.max_modules.is_some()) {
                            br {
                            }
                        }
                        if anmeldung.min_modules != 0 || anmeldung.max_modules.is_some() {
                            "Module: "
                            { anmeldung.min_modules.to_string() }
                            {
                                anmeldung
                                    .max_modules
                                    .map(|max_modules| " - ".to_string() + &max_modules.to_string())
                            }
                        }
                    }
                }
            }
        },
    )
}

#[component]
pub fn PlanningAnmeldung(
    connection: MyRc<RefCell<SqliteConnection>>,
    anmeldung: Anmeldung,
    depth: i32,
) -> Element {
    prep_planning(connection, anmeldung, depth).1
}
