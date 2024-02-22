// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::AppHandle;
use reqwest;
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

fn youtube_api_key(app_data_dir : std::path::PathBuf) -> String {
  let api_key_dir: std::path::PathBuf = app_data_dir.join("api_key.txt");
  std::fs::read_to_string(api_key_dir).unwrap()
}

// fn youtube_client_id(app_data_dir : std::path::PathBuf) -> String {
//   let client_id_dir: std::path::PathBuf = app_data_dir.join("client_id.txt");
//   std::fs::read_to_string(client_id_dir).unwrap()
// }
// fn youtube_client_secret(app_data_dir : std::path::PathBuf) -> String {
//   let client_secret_dir: std::path::PathBuf = app_data_dir.join("client_secret.txt");
//   std::fs::read_to_string(client_secret_dir).unwrap()
// }

async fn list_video_by_id(api_key: String, id: String) -> Result<reqwest::Response,reqwest::Error> {
  let url: &String = &format!("https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}", id, api_key);
  // let video = reqwest::get(url).await?.text().await?;
  // println!("{}", video);
  match reqwest::get(url).await {
    Ok(response) => Ok(response),
    Err(e) => Err(e)
  }
}

#[tauri::command]
async fn get_video_from_youtube_by_id(app: AppHandle,id: String) -> Result<String,String>{
  let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();
  let api_key = youtube_api_key(app_data_dir.clone());
  let result = list_video_by_id(api_key, id).await;
  println!("{:?}", result);
  match result {
    Ok(response) => {
      let video = response.text().await.unwrap();
      Ok(video)
    },
    Err(e) => {
      println!("{:?}", e);
      Err("This failed".into())
    }
  }
}

