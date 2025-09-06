use dioxus::prelude::*;
use sqlite_wasm_rs::{
    self as ffi,
    relaxed_idb_vfs::{RelaxedIdbCfg, install as install_idb_vfs},
};
use tucant_planning::abc;

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
    let test = use_resource(move || async move {
        let test = open_db().await;
        "Semesterplanung"
    });
    rsx! {
        div { class: "container",
            h2 {
                class: "text-center",
                { *test.read() }
            }
            form {
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
            }
        }
    }
}
