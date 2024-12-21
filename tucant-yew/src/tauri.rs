use tucant_types::Tucan;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    // invoke with arguments (default)
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub struct TauriTucan;

impl Tucan for TauriTucan {
    async fn login(
        request: tucant_types::LoginRequest,
    ) -> Result<tucant_types::LoginResponse, tucant_types::TucanError> {
        Ok(serde_wasm_bindgen::from_value(
            invoke(
                "tucant_login",
                serde_wasm_bindgen::to_value(&request).unwrap(),
            )
            .await,
        )
        .unwrap())
    }

    async fn anmeldung(
        request: tucant_types::registration::AnmeldungRequest,
    ) -> Result<tucant_types::registration::AnmeldungResponse, tucant_types::TucanError> {
        Ok(serde_wasm_bindgen::from_value(
            invoke(
                "tucant_anmeldung",
                serde_wasm_bindgen::to_value(&request).unwrap(),
            )
            .await,
        )
        .unwrap())
    }
}
