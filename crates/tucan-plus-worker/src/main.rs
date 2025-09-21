use std::time::Duration;

use diesel::{Connection as _, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness as _, embed_migrations};
use log::info;
use tucan_plus_worker::{Request, RequestResponse, RequestResponseEnum, RequestResponseObjectSafe};
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

pub async fn sleep(duration: Duration) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
        global
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve,
                duration.as_millis().try_into().unwrap(),
            )
            .unwrap();
    };

    let p = js_sys::Promise::new(&mut cb);

    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[wasm_bindgen(main)]
async fn main() {
    console_log::init().unwrap();

    let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
    let closure: Closure<dyn Fn(MessageEvent)> = Closure::new(move |event: MessageEvent| {
        let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
        global.post_message(&JsValue::from_str("Response")).unwrap();
        info!("Got message");

        let afewe: RequestResponseEnum = serde_wasm_bindgen::from_value(event.data()).unwrap();
        let result = afewe.execute();
    });
    global
        .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
        .unwrap();

    let util = sqlite_wasm_rs::sahpool_vfs::install(
        &sqlite_wasm_rs::sahpool_vfs::OpfsSAHPoolCfg::default(),
        true,
    )
    .await
    .unwrap();

    let mut connection = SqliteConnection::establish("tucan-plus.db").unwrap();

    connection.run_pending_migrations(MIGRATIONS).unwrap();

    //util.export_db("tucan-plus.db").unwrap();

    global.post_message(&JsValue::from_str("ready")).unwrap();
    sleep(Duration::from_secs(100000)).await;
}
