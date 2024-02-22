// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::AppHandle;
use youtube_api::{YoutubeApi, auth::stdio_login};

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
  let _add_data_dir = app.path_resolver().app_data_dir().unwrap();

  let api = YoutubeApi::new_with_oauth("", String::new(), String::new(), None).unwrap();
  api.login(stdio_login).await.unwrap();
  _ = id;
  Err("This failed".into())
}
