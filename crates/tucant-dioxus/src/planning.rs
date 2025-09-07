use dioxus::prelude::*;
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

// TODO at some point put opfs into a dedicated worker as that is the most
// correct approach TODO put this into a shared worker so there are no race
// conditions

async fn open_db() {
    // install relaxed-idb persistent vfs and set as default vfs
    install_idb_vfs(&RelaxedIdbCfg::default(), true)
        .await
        .unwrap();

    // open with relaxed-idb vfs
    let mut db = std::ptr::null_mut();
    let ret = unsafe {
        ffi::sqlite3_open_v2(
            c"relaxed-idb.db".as_ptr().cast(),
            &mut db as *mut _,
            ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE,
            std::ptr::null(),
        )
    };
    assert_eq!(ffi::SQLITE_OK, ret);
}

#[component]
pub fn Planning() -> Element {
    let mut sommersemester: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let mut wintersemester: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let test = use_resource(move || async move {
        let test = open_db().await;
        "Semesterplanung"
    });
    let onsubmit = move |evt: Event<FormData>| {
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
                let result: Vec<AnmeldungResponse> =
                    serde_json::from_reader(decompressed.as_slice()).unwrap();
                info!("{:?}", result);
            }
        }
    };
    rsx! {
        div { class: "container",
            h2 {
                class: "text-center",
                { *test.read() }
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
                        },
                        onchange: move |evt| {
                            async move {
                                if let Some(file_engine) = evt.files() {
                                    let files = file_engine.files();
                                    for file_name in &files {
                                        if let Some(file) = file_engine.read_file_to_string(file_name).await
                                        {
                                            //files_uploaded.write().push(file);
                                        }
                                    }
                                }
                            }
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
                        },
                        onchange: move |evt| {
                            async move {
                                if let Some(file_engine) = evt.files() {
                                    let files = file_engine.files();
                                    for file_name in &files {
                                        if let Some(file) = file_engine.read_file_to_string(file_name).await
                                        {
                                            //files_uploaded.write().push(file);
                                        }
                                    }
                                }
                            }
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
