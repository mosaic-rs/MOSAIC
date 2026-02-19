use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                apply_vibrancy(
                    &window,
                    NSVisualEffectMaterial::HudWindow,
                    Some(NSVisualEffectState::Active),
                    None,
                ).expect("Unsupported platform!");
                
                let _ = window.set_shadow(true);
            }

            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init()) 
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}