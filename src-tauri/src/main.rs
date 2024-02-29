// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use youtube_api::YoutubeApi;

pub mod youtube;

fn main() {
  tauri::Builder::default()
    .setup(|app: &mut tauri::App| {
      let _ = std::fs::create_dir_all(app.path_resolver().app_data_dir().unwrap());
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![youtube::get_video_from_youtube_by_id,youtube::get_videos_from_youtube_by_playlist_id])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
