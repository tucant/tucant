use serde_json::json;
use tucant_types::{
    LoginRequest, LoginResponse, RevalidationStrategy, SemesterId, Tucan, TucanError,
    coursedetails::{CourseDetailsRequest, CourseDetailsResponse},
    courseresults::ModuleResultsResponse,
    examresults::ExamResultsResponse,
    mlsstart::MlsStart,
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
    mycourses::MyCoursesResponse,
    mydocuments::MyDocumentsResponse,
    myexams::MyExamsResponse,
    registration::{AnmeldungRequest, AnmeldungResponse},
    vv::{ActionRequest, Vorlesungsverzeichnis},
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

    async fn anmeldung(&self, login_response: LoginResponse, _revalidation_strategy: RevalidationStrategy, request: AnmeldungRequest) -> Result<AnmeldungResponse, TucanError> {
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

    async fn module_details(&self, login_response: &LoginResponse, _revalidation_strategy: RevalidationStrategy, request: ModuleDetailsRequest) -> Result<ModuleDetailsResponse, TucanError> {
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

    async fn course_details(&self, login_response: &LoginResponse, _revalidation_strategy: RevalidationStrategy, request: CourseDetailsRequest) -> Result<CourseDetailsResponse, TucanError> {
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

    async fn logout(&self, _request: &LoginResponse) -> Result<(), TucanError> {
        todo!()
    }

    async fn after_login(&self, _request: &LoginResponse, _revalidation_strategy: RevalidationStrategy) -> Result<MlsStart, TucanError> {
        todo!()
    }

    async fn vv(&self, _login_response: Option<&LoginResponse>, _revalidation_strategy: RevalidationStrategy, _action: ActionRequest) -> Result<Vorlesungsverzeichnis, TucanError> {
        todo!()
    }

    async fn welcome(&self) -> Result<tucant_types::LoggedOutHead, TucanError> {
        todo!()
    }

    async fn my_modules(&self, _request: &LoginResponse, _revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<tucant_types::mymodules::MyModulesResponse, TucanError> {
        todo!()
    }

    async fn my_courses(&self, _request: &LoginResponse, _revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<MyCoursesResponse, TucanError> {
        todo!()
    }

    async fn my_exams(&self, _request: &LoginResponse, _revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<MyExamsResponse, TucanError> {
        todo!()
    }

    async fn exam_results(&self, _request: &LoginResponse, _revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<ExamResultsResponse, TucanError> {
        todo!()
    }

    async fn course_results(&self, _request: &LoginResponse, _revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<ModuleResultsResponse, TucanError> {
        todo!()
    }

    async fn my_documents(&self, _request: &LoginResponse, _revalidation_strategy: RevalidationStrategy) -> Result<MyDocumentsResponse, TucanError> {
        todo!()
    }
}
