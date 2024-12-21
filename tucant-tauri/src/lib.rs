use tucan_connector::{login::login, registration::index::anmeldung_cached, Tucan};
use tucant_types::{
    registration::{AnmeldungRequest, AnmeldungResponse},
    LoginRequest, LoginResponse,
};

#[tauri::command]
async fn tucant_login(request: LoginRequest) -> LoginResponse {
    let tucan = tucan_connector::Tucan::new().await.unwrap();
    login(&tucan.client, &request).await.unwrap()
}

#[tauri::command]
async fn tucant_registration(
    login_response: LoginResponse,
    request: AnmeldungRequest,
) -> AnmeldungResponse {
    let tucan = tucan_connector::Tucan::new().await.unwrap();
    anmeldung_cached(&tucan, &login_response, request)
        .await
        .unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![tucant_login, tucant_registration])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
