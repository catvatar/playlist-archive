use tauri::AppHandle;
use reqwest;

mod video_list_response;
use video_list_response::VideoListResponse;

mod playlist_responses;
use playlist_responses::PlaylistListResponse;

mod playlist_items_response;
use playlist_items_response::PlaylistItemsResponse;

pub mod error_type;
use error_type::YouTubeError;

use crate::entry::{Song, Playlist};

use crate::io_interface::{save_song_on_disk, save_playlist_on_disk_by_name, save_songs_from_playlist};

pub mod auth;

async fn get_video_by_id(id: &String, authenticator: &String) -> Result<VideoListResponse,reqwest::Error> {
    let url: &String = &format!("https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}", id, authenticator);
    // TODO: handle errors
    // intent: api can return 400 and response should be parsed to ErrorResponse
    Ok(reqwest::get(url).await?.json::<VideoListResponse>().await?)
}

#[tauri::command]
pub async fn get_video_from_youtube_by_id(app: AppHandle,id: String) -> Result<String,YouTubeError>{  
    let api_key: String = auth::authenticate(app.clone())?;

    let video_data: VideoListResponse = get_video_by_id(&id, &api_key).await?;
    
    let song_entry : Song = (&video_data.items[0]).into();
    
    // TODO: some basic vanity checks on the video data
    // intent: to reliably detect duplicates from different sources
    // for example, change "Skinshape - I Didn't Know (Official Video)" to "I Didn't Know" and check if author is "Skinshape"
    save_song_on_disk(app.clone(), song_entry)?;

    Ok("Rust: Video fetched successfully.".to_string())
}

async fn get_playlist_by_id(id: &String, authenticator: &String) -> Result<PlaylistListResponse,reqwest::Error> {
    let url: &String = &format!("https://www.googleapis.com/youtube/v3/playlists?part=snippet&id={}&key={}", id, authenticator);
    Ok(reqwest::get(url).await?.json::<PlaylistListResponse>().await?)
}

async fn get_videos_by_playlist_id(id: &String, authenticator: &String, next_page_token: Option<&String>) -> Result<PlaylistItemsResponse,reqwest::Error> {
    let url: String = match next_page_token {
        Some(token) => {
            format!("https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&maxResults=50&playlistId={}&key={}&pageToken={}", id, authenticator, token)
        },        
        None => 
            format!("https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&maxResults=50&playlistId={}&key={}", id, authenticator)
    };
    Ok(reqwest::get(url).await?.json::<PlaylistItemsResponse>().await?)
}

async fn get_all_videos_by_playlist_id(id: &String, authenticator: &String) -> Result<Vec<Song>,reqwest::Error> {
    let mut songs: Vec<Song> = Vec::new();
    let mut next_page_token: Option<String> = None;
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        }
        let response: PlaylistItemsResponse = get_videos_by_playlist_id(id, authenticator, next_page_token.as_ref()).await?;
        for item in response.items {
            let song : Song = (&item).into();
            match song.artist {
                Some(_) => {
                    songs.push((&item).into());
                },
                None => (),
            }
        }
        next_page_token = match response.next_page_token {
            Some(token) => Some(token),
            None => break,
        };
    }
    Ok(songs)
}

#[tauri::command]
pub async fn get_videos_from_youtube_by_playlist_id(app: AppHandle,id: String) -> Result<String, YouTubeError> {
    let api_key: String = auth::authenticate(app.clone())?;
    let playlist_data : PlaylistListResponse = get_playlist_by_id(&id, &api_key).await?;
    let playlist : Playlist = (&playlist_data.items[0]).into();

    let songs: Vec<Song> = get_all_videos_by_playlist_id(&id, &api_key).await?;
    let playlist_with_songs : Playlist = Playlist{
        songs: Some(songs),
        ..playlist
    };

    save_playlist_on_disk_by_name(app.clone(), playlist_with_songs.metadata.title.clone(), &playlist_with_songs)?;
    save_songs_from_playlist(app, playlist_with_songs)?;
    
    Ok("Rust: Playlist fetched successfully.".to_string())
}