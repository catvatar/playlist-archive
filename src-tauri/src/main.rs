// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::AppHandle;
use reqwest;
// use youtube_api::YoutubeApi;

fn main() {
  tauri::Builder::default()
    .setup(|app: &mut tauri::App| {
      let _ = std::fs::create_dir_all(app.path_resolver().app_data_dir().unwrap());
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_video_from_youtube_by_id,get_videos_from_youtube_by_playlist_id])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn youtube_api_key(app_data_dir : std::path::PathBuf) -> Result<String, std::io::Error> {
  let api_key_dir: std::path::PathBuf = app_data_dir.join("api_key.txt");
  std::fs::read_to_string(api_key_dir)
}

async fn list_video_by_id(api_key: String, id: String) -> Result<reqwest::Response,reqwest::Error> {
  let url: &String = &format!("https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}", id, api_key);
  reqwest::get(url).await
}

#[tauri::command]
async fn get_video_from_youtube_by_id(app: AppHandle,id: String) -> Result<String,String>{
  // authenticate the user
  let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();
  let api_key_result: Result<String, std::io::Error> = youtube_api_key(app_data_dir.clone());
  let api_key: String = match api_key_result {
    Ok(key) => key,
    Err(e) => {
      return Err(e.to_string());
    }
  };

  //fetch raw video data
  let raw_video_data_result:  Result<reqwest::Response,reqwest::Error> = list_video_by_id(api_key, id).await;
  match raw_video_data_result {
    Ok(response) => {
      let video: String = response.text().await.unwrap();
      Ok(video)
    },
    Err(e) => {
      Err(e.to_string())
    }
  }
}

async fn list_videos_by_playlist_id(api_key: String, id: String) -> Result<reqwest::Response,reqwest::Error> {
  let url: &String = &format!("https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&playlistId={}&key={}", id, api_key);
  reqwest::get(url).await
}

#[tauri::command]
async fn get_videos_from_youtube_by_playlist_id(app: AppHandle,id: String) -> Result<String, String> {
  let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();
  
  let api_key_result: Result<String, std::io::Error> = youtube_api_key(app_data_dir.clone());
  let api_key: String = match api_key_result {
    Ok(key) => key,
    Err(e) => {
      return Err(e.to_string());
    }
  };

  let list_videos_result:  Result<reqwest::Response,reqwest::Error> = list_videos_by_playlist_id(api_key, id).await;
  match list_videos_result {
    Ok(response) => {
      let videos: String = response.text().await.unwrap();
      Ok(videos)
    },
    Err(e) => {
      Err(e.to_string())
    }
  }
}