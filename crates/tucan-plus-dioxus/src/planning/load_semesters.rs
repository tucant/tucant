use dioxus::signals::Signal;
use futures::StreamExt as _;
use js_sys::Uint8Array;
use tucan_plus_planning::decompress;
use tucan_plus_worker::{
    models::{NewAnmeldung, NewAnmeldungEntry, Semester, State},
    schema::{anmeldungen_entries, anmeldungen_plan},
};
use tucan_types::{
    CONCURRENCY, LoginResponse, RevalidationStrategy, Tucan as _, registration::AnmeldungResponse,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{FileList, HtmlInputElement};

use crate::RcTucanType;

async fn handle_semester(
    course_of_study: &str,
    tucan: RcTucanType,
    login_response: &LoginResponse,
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
                course_of_study,
                url: e.path.last().unwrap().1.inner(),
                name: &e.path.last().unwrap().0,
                parent: e.path.len().checked_sub(2).map(|v| e.path[v].1.inner()),
                min_cp: 0,
                max_cp: None,
                min_modules: 0,
                max_modules: None,
            })
            .collect();
        // TODO
        let inserts: Vec<NewAnmeldungEntry> = futures::stream::iter(result.iter())
            .flat_map(|anmeldung| {
                futures::stream::iter(anmeldung.entries.iter()).map(async |entry| {
                    NewAnmeldungEntry {
                        course_of_study: course_of_study,
                        available_semester: semester,
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
                        year: None,
                        semester: None,
                    }
                })
            })
            .buffer_unordered(CONCURRENCY)
            .collect()
            .await;
        // prevent too many variable error, TODO maybe batching
        for insert in inserts {
            diesel::insert_into(anmeldungen_entries::table)
                .values(&insert)
                .on_conflict((
                    anmeldungen_entries::course_of_study,
                    anmeldungen_entries::anmeldung,
                    anmeldungen_entries::available_semester,
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
}
