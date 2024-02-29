use tauri::AppHandle;
use reqwest;

pub mod response;
use response::VideoListResponse;

fn youtube_api_key(app_data_dir : std::path::PathBuf) -> Result<String, std::io::Error> {
    let api_key_dir: std::path::PathBuf = app_data_dir.join("api_key.txt");
    std::fs::read_to_string(api_key_dir)
}

async fn list_video_by_id(api_key: String, id: String) -> Result<VideoListResponse,reqwest::Error> {
    let url: &String = &format!("https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}", id, api_key);
    Ok(reqwest::get(url).await?.json::<VideoListResponse>().await?)
}

#[tauri::command]
pub async fn get_video_from_youtube_by_id(app: AppHandle,id: String) -> Result<VideoListResponse,String>{
    // authenticate the user
    let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();
    let api_key_result: Result<String, std::io::Error> = self::youtube_api_key(app_data_dir.clone());
    let api_key: String = match api_key_result {
        Ok(key) => key,
        Err(e) => {
            return Err("Rust: ".to_owned() + &e.to_string());
        }
    };

    //fetch raw video data
    let video_data_result:  Result<VideoListResponse,reqwest::Error> = list_video_by_id(api_key, id).await;
    let video_data: VideoListResponse = match video_data_result{
        Ok(response) => response,
        Err(e) => {
            return Err("Rust: ".to_owned() + &e.to_string());
        }
    };
    println!("{:?}", video_data);
    
    Ok(video_data)
}

async fn list_videos_by_playlist_id(api_key: String, id: String) -> Result<reqwest::Response,reqwest::Error> {
    let url: &String = &format!("https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&playlistId={}&key={}", id, api_key);
    reqwest::get(url).await
}

#[tauri::command]
pub async fn get_videos_from_youtube_by_playlist_id(app: AppHandle,id: String) -> Result<String, String> {
    let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();
    
    let api_key_result: Result<String, std::io::Error> = youtube_api_key(app_data_dir.clone());
    let api_key: String = match api_key_result {
        Ok(key) => key,
        Err(e) => {
            return Err("Rust: ".to_owned() + &e.to_string());
        }
    };

    let list_videos_result:  Result<reqwest::Response,reqwest::Error> = list_videos_by_playlist_id(api_key, id).await;
    match list_videos_result {
        Ok(response) => {
            let videos: String = response.text().await.unwrap();
            Ok(videos)
        },
        Err(e) => {
            Err("Rust: ".to_owned() + &e.to_string())
        }
    }
}