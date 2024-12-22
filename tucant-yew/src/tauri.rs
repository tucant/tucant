use serde_json::json;
use tucant_types::{
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
    registration::{AnmeldungRequest, AnmeldungResponse},
    LoginRequest, LoginResponse, Tucan, TucanError,
};
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
    async fn login(request: LoginRequest) -> Result<LoginResponse, TucanError> {
        Ok(serde_wasm_bindgen::from_value(
            invoke(
                "tucant_login",
                serde_wasm_bindgen::to_value(&json!({
                    "request": request
                }))
                .unwrap(),
            )
            .await,
        )
        .unwrap())
    }

    async fn anmeldung(
        login_response: LoginResponse,
        request: AnmeldungRequest,
    ) -> Result<AnmeldungResponse, TucanError> {
        Ok(serde_wasm_bindgen::from_value(
            invoke(
                "tucant_registration",
                serde_wasm_bindgen::to_value(&json!({
                    "request": request,
                    "loginResponse": login_response,
                }))
                .unwrap(),
            )
            .await,
        )
        .unwrap())
    }

    async fn module_details(
        login_response: &LoginResponse,
        request: ModuleDetailsRequest,
    ) -> Result<ModuleDetailsResponse, TucanError> {
        Ok(serde_wasm_bindgen::from_value(
            invoke(
                "tucant_module_details",
                serde_wasm_bindgen::to_value(&json!({
                    "request": request,
                    "loginResponse": login_response,
                }))
                .unwrap(),
            )
            .await,
        )
        .unwrap())
    }
}
