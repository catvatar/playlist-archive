use tauri::AppHandle;
use reqwest;

pub mod response;
use crate::entry::{self, Song, Playlist, Metadata};
use response::VideoListResponse;
use serde_json;
use std::fs;

fn youtube_api_key(app : AppHandle) -> Result<String, std::io::Error> {
    let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();

    let api_key_dir: std::path::PathBuf = app_data_dir.join("api_key.txt");
    std::fs::read_to_string(api_key_dir)
}

async fn list_video_by_id(api_key: String, id: String) -> Result<VideoListResponse,reqwest::Error> {
    let url: &String = &format!("https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}", id, api_key);
    Ok(reqwest::get(url).await?.json::<VideoListResponse>().await?)
}

fn playlist_save_directory(app: AppHandle) -> std::path::PathBuf {
    let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();
    let playlist_dir: std::path::PathBuf = app_data_dir.join("playlists");
    let _ = fs::create_dir_all(&playlist_dir);
    playlist_dir
}

fn get_playlist_from_disk(app: AppHandle, playlist_name: String) -> Result<Playlist, std::io::Error> {
    let playlist_directory: std::path::PathBuf = playlist_save_directory(app);
    let playlist: Playlist = match fs::read_to_string(playlist_directory.join(playlist_name.to_owned() + ".json")) {
        Ok(playlist) => serde_json::from_str(&playlist).expect("Rust: Failed to parse playlist."),
        Err(e) => {
            return Err(e);
        }
    };
    Ok(playlist)
}
 
fn save_song_on_disk(app: AppHandle, song: Song) -> Result<(), std::io::Error> {
    let mut saved_songs_playlist: Playlist = match get_playlist_from_disk(app.clone(), "saved_songs".to_string()) {
        Ok(playlist) => playlist,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let metadata : Metadata = Metadata{
                        title: "saved_songs".to_string(),
                        author: "catvatar".to_string(),
                        tags: None,
                        date: chrono::offset::Local::now().date_naive().to_string().into(),
                        thumbnail: None,
                        comment: "Write your comment here.".to_string(),
                    };
                    Playlist{
                        metadata,
                        visibility: entry::Visibility::Visible,
                        songs: None,
                    }
                },
                _ => {return Err(e);}
            }
        }
    };
    // append the video to the playlist
    saved_songs_playlist.songs = match saved_songs_playlist.songs {
        Some(mut songs) => {
            if songs.contains(&song) {
                return Ok(());
            }
            songs.push(song);
            Some(songs)
        },
        None => {
            Some(vec![song])
        }
    };
    // update to serialize to toml instead of json
    let song_json_result:Result<String, serde_json::Error> = serde_json::to_string(&saved_songs_playlist);
    let song_json = match song_json_result {
        Ok(json) => json,
        Err(e) => {
            println!("Rust: Failed to convert song to json. {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
    };
    let _ = fs::write(playlist_save_directory(app).join("saved_songs.json"),song_json);
    Ok(())
}

#[tauri::command]
pub async fn get_video_from_youtube_by_id(app: AppHandle,id: String) -> Result<String,String>{
    // authenticate the user
    let api_key_result: Result<String, std::io::Error> = self::youtube_api_key(app.clone());
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
    let song_entry : Song = video_data.into();
    save_song_on_disk(app.clone(), song_entry).expect("Rust: Failed to save video on disk.");
    Ok("Rust: Video fetched successfully.".to_string())
}

async fn list_videos_by_playlist_id(api_key: String, id: String) -> Result<reqwest::Response,reqwest::Error> {
    let url: &String = &format!("https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&playlistId={}&key={}", id, api_key);
    reqwest::get(url).await
}

#[tauri::command]
pub async fn get_videos_from_youtube_by_playlist_id(app: AppHandle,id: String) -> Result<String, String> {  
    let api_key_result: Result<String, std::io::Error> = youtube_api_key(app.clone());
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