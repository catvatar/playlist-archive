use tauri::AppHandle;
use reqwest;

pub mod video_list_response;

pub mod auth;

pub mod error_type;
use error_type::YouTubeError;

use crate::entry::{self, Song, Playlist, Metadata};
use video_list_response::VideoListResponse;
use serde_json;
use std::fs;


async fn list_video_by_id(authenticator: String, id: String) -> Result<VideoListResponse,reqwest::Error> {
    let url: &String = &format!("https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}", id, authenticator);
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

fn create_saved_songs_playlist() -> Playlist {
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
}
 
fn save_song_on_disk(app: AppHandle, song: Song) -> Result<(), std::io::Error> {
    let mut saved_songs_playlist: Playlist = match get_playlist_from_disk(app.clone(), "saved_songs".to_string()) {
        Ok(playlist) => playlist,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    create_saved_songs_playlist()
                },
                _ => {return Err(e);}
            }
        }
    };
    saved_songs_playlist.songs = match saved_songs_playlist.songs {
        Some(mut songs) => {
            if songs.contains(&song) {
                // TODO: update different fields of the song
                // for example, if song if from a different source, update the sources
                return Ok(());
            }
            songs.push(song);
            Some(songs)
        },
        None => {
            Some(vec![song])
        }
    };
    // TODO: update to serialize to toml instead of json
    let song_json = match serde_json::to_string(&saved_songs_playlist) {
        Ok(json) => json,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
    };
    let _ = fs::write(playlist_save_directory(app).join("saved_songs.json"),song_json);
    Ok(())
}

#[tauri::command]
pub async fn get_video_from_youtube_by_id(app: AppHandle,id: String) -> Result<String,YouTubeError>{  
    let api_key: String = auth::authenticate(app.clone())?;

    let video_data: VideoListResponse = list_video_by_id(id,api_key).await?;

    let song_entry : Song = (&video_data.items[0]).into();

    // TODO: some basic vanity checks on the video data
    // intent: to reliably detect duplicates from different sources
    // for example, change "Skinshape - I Didn't Know (Official Video)" to "I Didn't Know" and check if author is "Skinshape"
    save_song_on_disk(app.clone(), song_entry)?;

    Ok("Rust: Video fetched successfully.".to_string())
}

async fn list_videos_by_playlist_id(api_key: String, id: String) -> Result<reqwest::Response,reqwest::Error> {
    let url: &String = &format!("https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&playlistId={}&key={}", id, api_key);
    reqwest::get(url).await
}

#[tauri::command]
pub async fn get_videos_from_youtube_by_playlist_id(app: AppHandle,id: String) -> Result<String, YouTubeError> {  
    let api_key: String = auth::authenticate(app.clone())?;

    // TODO: fetch and parse songs on the playlist
    // I will need to requests to the youtube api
    // 1. to get the playlist data
    // 2. to get the list of videos on the playlist
    let list_videos_result:  Result<reqwest::Response,reqwest::Error> = list_videos_by_playlist_id(api_key, id).await;
    match list_videos_result {
        Ok(response) => {
            let videos: String = response.text().await.unwrap();
            Ok(videos)
        },
        Err(e) => {
            Err(e.into())
        }
    }
}