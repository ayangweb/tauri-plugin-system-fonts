pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_system_fonts::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
