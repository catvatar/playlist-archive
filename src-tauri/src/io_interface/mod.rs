use tauri::AppHandle;
use std::fs;

use crate::entry::{Song, Playlist, Metadata, Visibility};


pub fn playlist_save_directory(app: AppHandle) -> std::path::PathBuf {
    let app_data_dir: std::path::PathBuf = app.path_resolver().app_data_dir().unwrap();
    let playlist_dir: std::path::PathBuf = app_data_dir.join("playlists");
    let _ = fs::create_dir_all(&playlist_dir);
    playlist_dir
}

pub fn get_playlist_from_disk(app: AppHandle, playlist_name: String) -> Result<Playlist, std::io::Error> {
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
        visibility: Visibility::Visible,
        songs: None,
    }
}

pub fn save_playlist_on_disk(save_path: std::path::PathBuf, playlist: &Playlist) -> Result<(), std::io::Error> {
    // TODO: update to serialize to toml instead of json
    let song_json = match serde_json::to_string(&playlist) {
        Ok(json) => json,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
    };
    let _ = fs::write(save_path,song_json)?;
    Ok(())
}


pub fn save_playlist_on_disk_by_name(app: AppHandle, playlist_name: String, playlist: &Playlist) -> Result<(), std::io::Error> {
    let playlist_directory: std::path::PathBuf = playlist_save_directory(app).join(playlist_name.to_owned() + ".json");
    save_playlist_on_disk(playlist_directory, playlist)?;
    Ok(())
}

pub fn save_song_on_disk(app: AppHandle, song: Song) -> Result<(), std::io::Error> {
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
    save_playlist_on_disk_by_name(app, "saved_songs".to_owned(), &saved_songs_playlist)?;
    Ok(())
}

pub fn save_songs_from_playlist(app: AppHandle, playlist: Playlist) -> Result<(), std::io::Error> {
    match playlist.songs {
        Some(songs) => {
            for song in songs {
                save_song_on_disk(app.clone(), song)?;
            }
        },
        None => (),
    }
    Ok(())
}