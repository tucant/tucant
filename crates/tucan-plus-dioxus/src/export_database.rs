use std::cell::RefCell;

use crate::MyRc;
use diesel::{Connection as _, SqliteConnection};
use dioxus::prelude::*;
use js_sys::{Array, Uint8Array};
use web_sys::{Blob, Url};

async fn export_db() -> Vec<u8> {
    #[cfg(target_arch = "wasm32")]
    {
        let util = sqlite_wasm_rs::relaxed_idb_vfs::install(
            &sqlite_wasm_rs::relaxed_idb_vfs::RelaxedIdbCfg::default(),
            true,
        )
        .await
        .unwrap();

        return util.export_db("tucan-plus.db").unwrap();
    }
    panic!();
}

#[component]
pub fn ExportDatabase() -> Element {
    let connection = use_resource(move || async move { export_db().await });
    rsx! {
        if let Some(database) = connection() {
            a {
                href: {
                    let blob_properties = web_sys::BlobPropertyBag::new();
                    blob_properties.set_type("octet/stream");
                    let bytes = Array::new();
                    bytes.push(&Uint8Array::from(&database[..]));
                    let blob =
                        Blob::new_with_blob_sequence_and_options(&bytes, &blob_properties).unwrap();
                    Url::create_object_url_with_blob(&blob).unwrap()
                },
                download: "tucan-plus.db",
                "Download database"
            }
        }
    }
}
