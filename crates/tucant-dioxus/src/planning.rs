use std::cell::RefCell;
use std::sync::Arc;

use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness as _, embed_migrations};
use dioxus::prelude::*;
use futures::StreamExt;
use js_sys::Uint8Array;
use log::info;
use tucant_planning::decompress;
use tucant_types::registration::AnmeldungResponse;
use tucant_types::student_result::StudentResultLevel;
use tucant_types::{
    CONCURRENCY, LeistungsspiegelGrade, LoginResponse, RevalidationStrategy, Tucan as _,
};
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
    #[cfg(target_arch = "wasm32")]
    sqlite_wasm_rs::relaxed_idb_vfs::install(
        &sqlite_wasm_rs::relaxed_idb_vfs::RelaxedIdbCfg::default(),
        true,
    )
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
    tucan: RcTucanType,
    login_response: &LoginResponse,
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
        diesel::insert_into(anmeldungen_plan::table)
            .values(&inserts)
            .on_conflict(anmeldungen_plan::url)
            .do_update()
            .set(anmeldungen_plan::parent.eq(excluded(anmeldungen_plan::parent)))
            .execute(&mut *connection_clone.borrow_mut())
            .expect("Error saving anmeldungen");
        let inserts: Vec<NewAnmeldungEntry> = futures::stream::iter(result.iter())
            .flat_map(|anmeldung| {
                futures::stream::iter(anmeldung.entries.iter()).map(async |entry| {
                    NewAnmeldungEntry {
                        semester,
                        anmeldung: anmeldung.path.last().unwrap().1.inner(),
                        module_url: entry.module.as_ref().unwrap().url.inner(),
                        id: &entry.module.as_ref().unwrap().id,
                        name: &entry.module.as_ref().unwrap().name,
                        credits: tucan
                            .module_details(
                                login_response,
                                RevalidationStrategy::cache(),
                                entry.module.as_ref().unwrap().url.clone(),
                            )
                            .await
                            .unwrap()
                            .credits
                            .unwrap_or_default()
                            .try_into()
                            .unwrap(),
                        state: State::NotPlanned,
                    }
                })
            })
            .buffer_unordered(CONCURRENCY)
            .collect()
            .await;
        diesel::insert_into(anmeldungen_entries::table)
            .values(&inserts)
            .on_conflict((
                anmeldungen_entries::anmeldung,
                anmeldungen_entries::semester,
                anmeldungen_entries::id,
            ))
            .do_update()
            .set((
                // TODO FIXME I think updating does not work
                anmeldungen_entries::state.eq(excluded(anmeldungen_entries::state)),
                (anmeldungen_entries::credits.eq(excluded(anmeldungen_entries::credits))),
            ))
            .execute(&mut *connection_clone.borrow_mut())
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
            id: entry.id.as_ref().unwrap_or(&entry.name), /* TODO FIXME, use two columns
                                 * and both as primary key */
            credits: i32::try_from(entry.used_cp.unwrap_or_else(|| {
                if level.name.as_deref() == Some("Masterarbeit") {
                    30
                } else {
                    0
                }
            }))
            .unwrap(),
            name: &entry.name,
            state: if matches!(
                entry.grade,
                LeistungsspiegelGrade::Grade(_) | LeistungsspiegelGrade::BestandenOhneNote
            ) {
                State::Done
            } else {
                State::Planned
            },
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
    let current_session_handle = use_context::<Signal<Option<LoginResponse>>>();
    let mut loading = use_signal(|| false);
    let mut future = {
        let connection_clone = connection_clone.clone();
        use_resource(move || {
            let connection_clone = connection_clone.clone();
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
    let load_leistungsspiegel = {
        let connection_clone = connection_clone.clone();
        let tucan = tucan.clone();
        move |_event: Event<MouseData>| {
            let connection_clone = connection_clone.clone();
            let current_session_handle = current_session_handle;
            let tucan = tucan.clone();
            async move {
                loading.set(true);
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
                loading.set(false);
                future.restart();
            }
        }
    };
    let connection_clone = connection.clone();
    let tucan = tucan.clone();
    let onsubmit = move |evt: Event<FormData>| {
        let connection_clone = connection_clone.clone();
        let tucan = tucan.clone();
        evt.prevent_default();
        async move {
            loading.set(true);
            handle_semester(
                tucan.clone(),
                &current_session_handle().unwrap(),
                connection_clone.clone(),
                Semester::Sommersemester,
                sommersemester,
            )
            .await;
            handle_semester(
                tucan.clone(),
                &current_session_handle().unwrap(),
                connection_clone,
                Semester::Wintersemester,
                wintersemester,
            )
            .await;
            info!("done");
            loading.set(false);
            future.restart();
        }
    };
    rsx! {
        div {
            class: "container",
            if loading() {
                div {
                    style: "z-index: 10000",
                    class: "position-fixed top-50 start-50 translate-middle",
                    div {
                        class: "spinner-grow",
                        role: "status",
                        span {
                            class: "visually-hidden",
                            "Loading..."
                        }
                    }
                }
            }
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
                    disabled: loading(),
                    type: "submit",
                    class: "btn btn-primary",
                    "Planung starten"
                }
            }
            button {
                disabled: loading(),
                type: "button",
                class: "btn btn-primary mb-3",
                onclick: load_leistungsspiegel,
                "Leistungsspiegel laden (nach Laden der Semester)"
            }
            if let Some(value) = future() {
                for entry in value {
                    PlanningAnmeldung {
                        future,
                        connection: connection.clone(),
                        anmeldung: entry.clone(),
                    }
                }
            }
        }
    }
}

pub struct PrepPlanningReturn {
    has_contents: bool,
    credits: i32,
    modules: usize,
    element: Element,
}

pub struct YearAndSemester(pub u32, pub Semester);

pub enum PlanningState {
    NotPlanned,
    MaybePlanned(Option<YearAndSemester>),
    Planned(Option<YearAndSemester>),
    Done(Option<YearAndSemester>),
}

fn prep_planning(
    mut future: Resource<Vec<Anmeldung>>,
    connection: MyRc<RefCell<SqliteConnection>>,
    anmeldung: Anmeldung, // ahh this needs to be a signal?
) -> PrepPlanningReturn {
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
    let inner: Vec<PrepPlanningReturn> = results
        .iter()
        .map(|result| prep_planning(future, connection.clone(), result.clone()))
        .collect();
    let has_rules = anmeldung.min_cp != 0
        || anmeldung.max_cp.is_some()
        || anmeldung.min_modules != 0
        || anmeldung.max_modules.is_some();
    let mut expanded = use_signal(|| false);
    let interesting = expanded()
        || has_rules
        || entries.iter().any(|entry| entry.state != State::NotPlanned)
        || inner.iter().any(|v| v.has_contents);
    let cp: i32 = entries
        .iter()
        .filter(|entry| entry.state == State::Done || entry.state == State::Planned)
        .map(|entry| entry.credits)
        .sum::<i32>()
        + inner.iter().map(|inner| inner.credits).sum::<i32>();
    let used_cp = std::cmp::min(cp, anmeldung.max_cp.unwrap_or(cp));
    let modules: usize = entries
        .iter()
        .filter(|entry| entry.state == State::Done || entry.state == State::Planned)
        .count()
        + inner.iter().map(|inner| inner.modules).sum::<usize>();
    PrepPlanningReturn {
        has_contents: interesting,
        credits: used_cp,
        modules,
        element: rsx! {
            div {
                class: "h3",
                { anmeldung.name.clone() }
                " "
                button {
                    type: "button",
                    class: "btn btn-secondary",
                    onclick: move |_| {
                        expanded.toggle();
                    },
                    { if expanded() { "-" } else { "+" } }
                }
            }
            div {
                class: "ms-2 ps-2",
                style: "border-left: 1px solid #ccc;",
                if (!entries.is_empty() && expanded())
                    || entries.iter().any(|entry| entry.state != State::NotPlanned) {
                    table {
                        class: "table",
                        tbody {
                            for (key, entry) in entries
                                .iter()
                                .filter(|entry| expanded() || entry.state != State::NotPlanned)
                                .map(|entry| (
                                    format!("{}{:?}", entry.id, entry.available_semester),
                                    entry
                                )) {
                                tr {
                                    key: "{key}",
                                    td {
                                        { entry.id.clone() }
                                    }
                                    td {
                                        { entry.name.clone() }
                                    }
                                    td {
                                        { format!("{:?}", entry.available_semester) }
                                    }
                                    td {
                                        { entry.credits.to_string() }
                                    }
                                    td {
                                        select {
                                            class: match entry.state {
                                                State::NotPlanned => "form-select bg-secondary",
                                                State::Planned => "form-select bg-primary",
                                                State::Done => "form-select bg-success",
                                            },
                                            option {
                                                onclick: {
                                                    let connection = connection.clone();
                                                    let mut entry = entry.clone();
                                                    move |event| {
                                                        event.prevent_default();
                                                        let connection = connection.clone();
                                                        entry.state = State::NotPlanned;
                                                        diesel::update(&entry)
                                                            .set(&entry)
                                                            .execute(&mut *connection.borrow_mut())
                                                            .unwrap();
                                                        future.restart();
                                                    }
                                                },
                                                selected: entry.state == State::NotPlanned,
                                                { format!("{:?}", State::NotPlanned) }
                                            }
                                            option {
                                                onclick: {
                                                    let connection = connection.clone();
                                                    let mut entry = entry.clone();
                                                    move |event| {
                                                        event.prevent_default();
                                                        let connection = connection.clone();
                                                        entry.state = State::Planned;
                                                        diesel::update(&entry)
                                                            .set(&entry)
                                                            .execute(&mut *connection.borrow_mut())
                                                            .unwrap();
                                                        future.restart();
                                                    }
                                                },
                                                selected: entry.state == State::Planned,
                                                { format!("{:?}", State::Planned) }
                                            }
                                            option {
                                                onclick: {
                                                    let connection = connection.clone();
                                                    let mut entry = entry.clone();
                                                    move |event| {
                                                        event.prevent_default();
                                                        let connection = connection.clone();
                                                        entry.state = State::Done;
                                                        diesel::update(&entry)
                                                            .set(&entry)
                                                            .execute(&mut *connection.borrow_mut())
                                                            .unwrap();
                                                        future.restart();
                                                    }
                                                },
                                                selected: entry.state == State::Done,
                                                { format!("{:?}", State::Done) }
                                            }
                                        }
                                        select {
                                            class: "form-select",
                                            style: "min-width: 15em",
                                            option {
                                                value: "",
                                                "Choose semester"
                                            }
                                            for i in 2025..2030 {
                                                option {
                                                    onclick: {
                                                        let connection = connection.clone();
                                                        let mut entry = entry.clone();
                                                        move |event| {
                                                            event.prevent_default();
                                                            let connection = connection.clone();
                                                            entry.semester =
                                                                Some(Semester::Sommersemester);
                                                            entry.year = Some(i);
                                                            diesel::update(&entry)
                                                                .set(&entry)
                                                                .execute(
                                                                    &mut *connection.borrow_mut(),
                                                                )
                                                                .unwrap();
                                                            future.restart();
                                                        }
                                                    },
                                                    selected: false,
                                                    "Sommersemester {i}"
                                                }
                                                option {
                                                    onclick: {
                                                        let connection = connection.clone();
                                                        let mut entry = entry.clone();
                                                        move |event| {
                                                            event.prevent_default();
                                                            let connection = connection.clone();
                                                            entry.semester =
                                                                Some(Semester::Wintersemester);
                                                            entry.year = Some(i);
                                                            diesel::update(&entry)
                                                                .set(&entry)
                                                                .execute(
                                                                    &mut *connection.borrow_mut(),
                                                                )
                                                                .unwrap();
                                                            future.restart();
                                                        }
                                                    },
                                                    selected: false,
                                                    "Wintersemester {i}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if expanded() || inner.iter().any(|v| v.has_contents) {
                    for (key, value) in results
                        .iter()
                        .zip(inner.into_iter())
                        .filter(|(_, value)| expanded() || value.has_contents)
                        .map(|(key, value)| (&key.url, value)) {
                        div {
                            key: "{key}",
                            { value.element }
                        }
                    }
                }
                if has_rules {
                    p {
                        { "Summe ".to_owned() + &anmeldung.name + ":" }
                        br {
                        }
                        if anmeldung.min_cp != 0 || anmeldung.max_cp.is_some() {
                            span {
                                class: if anmeldung.min_cp <= cp
                                    && anmeldung.max_cp.map(|max| cp <= max).unwrap_or(true)
                                {
                                    "bg-success"
                                } else {
                                    if anmeldung.max_cp.map(|max| cp > max).unwrap_or(false) {
                                        "bg-warning"
                                    } else {
                                        "bg-danger"
                                    }
                                },
                                "CP: "
                                { cp.to_string() }
                                " / "
                                { anmeldung.min_cp.to_string() }
                                {
                                    anmeldung
                                        .max_cp
                                        .map(|max_cp| " - ".to_string() + &max_cp.to_string())
                                }
                            }
                        }
                        if (anmeldung.min_cp != 0 || anmeldung.max_cp.is_some())
                            && (anmeldung.min_modules != 0 || anmeldung.max_modules.is_some()) {
                            br {
                            }
                        }
                        if anmeldung.min_modules != 0 || anmeldung.max_modules.is_some() {
                            span {
                                class: if anmeldung.min_modules <= modules.try_into().unwrap()
                                    && anmeldung
                                        .max_modules
                                        .map(|max| modules <= max.try_into().unwrap())
                                        .unwrap_or(true)
                                {
                                    "bg-success"
                                } else {
                                    "bg-danger"
                                },
                                "Module: "
                                { modules.to_string() }
                                " / "
                                { anmeldung.min_modules.to_string() }
                                {
                                    anmeldung.max_modules.map(|max_modules| {
                                        " - ".to_string() + &max_modules.to_string()
                                    })
                                }
                            }
                        }
                    }
                }
            }
        },
    }
}

#[component]
pub fn PlanningAnmeldung(
    future: Resource<Vec<Anmeldung>>,
    connection: MyRc<RefCell<SqliteConnection>>,
    anmeldung: Anmeldung,
) -> Element {
    let _ = future();
    prep_planning(future, connection, anmeldung).element
}
