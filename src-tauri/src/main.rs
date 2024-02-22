// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::AppHandle;
use youtube_api::YoutubeApi;

fn main() {
  tauri::Builder::default()
    .setup(|app: &mut tauri::App| {
      let _ = std::fs::create_dir_all(app.path_resolver().app_data_dir().unwrap());
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_video_from_youtube_by_id])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn get_video_from_youtube_by_id(app: AppHandle,id: String) -> Result<String,String>{
  let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();
  let api_key_dir: std::path::PathBuf = app_data_dir.join("api_key.txt");
  let _api_key: String = std::fs::read_to_string(api_key_dir).unwrap();

  let _api: YoutubeApi = YoutubeApi::new_with_oauth("", String::new(), String::new(), None).unwrap();
  // api.login(stdio_login).await.unwrap();
  _ = id;
  Err("This failed".into())
}
