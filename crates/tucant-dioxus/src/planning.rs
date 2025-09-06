use dioxus::prelude::*;
use tucant_planning::abc;

use sqlite_wasm_rs::{
    self as ffi,
    sahpool_vfs::{install as install_opfs_sahpool, OpfsSAHPoolCfg},
};

async fn open_db() {
    // install opfs-sahpool persistent vfs and set as default vfs
    install_opfs_sahpool(&OpfsSAHPoolCfg::default(), true)
        .await
        .unwrap();

    // open with opfs-sahpool vfs
    let mut db = std::ptr::null_mut();
    let ret = unsafe {
        ffi::sqlite3_open_v2(
            c"opfs-sahpool.db".as_ptr().cast(),
            &mut db as *mut _,
            ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE,
            std::ptr::null()
        )
    };
    assert_eq!(ffi::SQLITE_OK, ret);
}

#[component]
pub fn Planning() -> Element {
    let a = abc();
    rsx! {
        "Test"
    }
}