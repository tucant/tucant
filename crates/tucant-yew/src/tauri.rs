use serde_json::json;
use tucant_types::{
    LoginRequest, LoginResponse, Tucan, TucanError, Vorlesungsverzeichnis,
    coursedetails::{CourseDetailsRequest, CourseDetailsResponse},
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
    registration::{AnmeldungRequest, AnmeldungResponse},
};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

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
    async fn login(&self, request: LoginRequest) -> Result<LoginResponse, TucanError> {
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

    async fn anmeldung(&self, login_response: LoginResponse, request: AnmeldungRequest) -> Result<AnmeldungResponse, TucanError> {
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

    async fn module_details(&self, login_response: &LoginResponse, request: ModuleDetailsRequest) -> Result<ModuleDetailsResponse, TucanError> {
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

    async fn course_details(&self, login_response: &LoginResponse, request: CourseDetailsRequest) -> Result<CourseDetailsResponse, TucanError> {
        Ok(serde_wasm_bindgen::from_value(
            invoke(
                "tucant_course_details",
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

    async fn logout(&self, request: &LoginResponse) -> Result<(), TucanError> {
        todo!()
    }

    async fn after_login(&self, request: &LoginResponse) -> Result<tucant_types::LoggedInHead, TucanError> {
        todo!()
    }

    async fn vv(&self, login_response: &LoginResponse, action: String) -> Result<Vorlesungsverzeichnis, TucanError> {
        todo!()
    }
}
