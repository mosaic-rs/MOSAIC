#[tauri::command]
pub async fn open_settings_window(app: tauri::AppHandle) {
    let _ = tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("pages/settings/index.html".into()),
    )
    .title("MOSAIC Settings")
    .build()
    .unwrap();
}