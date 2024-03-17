// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod youtube;
pub mod entry;
pub mod io_interface;
pub mod load_media_interface;

use tauri::AppHandle;

fn main() {
  tauri::Builder::default()
    .setup(|app: &mut tauri::App| {
      let _ = std::fs::create_dir_all(app.path_resolver().app_data_dir().unwrap());
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![load_media_handle])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn load_media_handle(app: AppHandle,source_media_url: String, forced_source_type: Option<String>, force_import_type: Option<String>) -> Result<String, load_media_interface::LoadMediaError> {
    load_media_interface::load_media(app,source_media_url, forced_source_type, force_import_type).await
}