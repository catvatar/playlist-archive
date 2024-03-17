use serde::{Deserialize, Serialize};
use url::Url;
use crate::youtube;
use crate::youtube::error_type::YouTubeError;
use tauri::AppHandle;
use crate::entry::{ImportType, SourceType, Source};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadMediaError {
    pub error: String,
    pub message: String
}

impl Into<String> for LoadMediaError {
    fn into(self) -> String {
        format!("Error: {}, Message: {}", self.error, self.message)
    }
}

impl From<url::ParseError> for LoadMediaError {
    fn from(error: url::ParseError) -> Self {
        LoadMediaError {
            error: "Url Parse".to_string(),
            message: error.to_string()
        }
    }
}

impl From<YouTubeError> for LoadMediaError {
    fn from(error: YouTubeError) -> Self {
        LoadMediaError {
            error: "YouTube".to_string(),
            message: error.error_message
        }
    }
}


fn match_source(url: &Url) -> SourceType {
    match url.host_str() {
        Some("www.youtube.com") => SourceType::YouTube,
        Some("youtube.com") => SourceType::YouTube,
        Some("youtu.be") => SourceType::YouTube,
        Some("music.youtube.com") => SourceType::YouTube,
        Some("www.spotify.com") => SourceType::Spotify,
        Some("spotify.com") => SourceType::Spotify,
        Some("open.spotify.com") => SourceType::Spotify,
        _ => SourceType::Unknown
    }
}

fn match_type(url: &Url, source: &SourceType) -> ImportType {
    match source {
        SourceType::YouTube => {
            if url.path().starts_with("/playlist") {
                ImportType::Playlist
            } else {
                ImportType::Song
            }
        },
        SourceType::Spotify => {
            if url.path().starts_with("/playlist") {
                ImportType::Playlist
            } else if url.path().starts_with("/album") {
                ImportType::Album
            } else {
                ImportType::Song
            }
        }
        SourceType::Unknown => ImportType::Song
    }
}

fn youtube_extract_id(url: &Url) -> String {
    if url.host_str().unwrap() == "youtu.be" {
        url.path().to_string().replace("/", "")
    } else {
        url.query_pairs().find(|(key, _)| key == "v").unwrap().1.to_string()
    }
}

async fn handle_youtube_video(app:AppHandle,source: &Source) -> Result<String, LoadMediaError> {
    if source.source_type != SourceType::YouTube {
        return Err(LoadMediaError {
            error: "Source".to_string(),
            message: "Invalid source type".to_string()
        })
    }
    match source.import_type {
        ImportType::Song => {
            let response = "Success, YouTube, video".to_string();
            Ok(response)
        },
        ImportType::Playlist => {
            let response = (youtube::get_videos_from_youtube_by_playlist_id(app,&source.id).await)?;
            Ok(response)
        },
        ImportType::Album => {
            let response = LoadMediaError {
                error: "Import Type".to_string(),
                message: "Album import type not implemented".to_string()
            };
            Err(response)
        }
    }

}

fn construct_source(source_media_url: String, forced_source_type: Option<String>, force_import_type: Option<String>) -> Result<Source, LoadMediaError> {
    let url: Url = Url::parse(&source_media_url)?; 

    let source_type: SourceType = match forced_source_type {
        Some(source) => {
            match source.as_str() {
                "YouTube" => SourceType::YouTube,
                "Spotify" => SourceType::Spotify,
                _ => SourceType::Unknown
            }
        },
        None => {
            match_source(&url)
        }
    };
    let import_type : ImportType = match force_import_type {
        Some(import_type) => {
            match import_type.as_str() {
                "Song" => ImportType::Song,
                "Playlist" => ImportType::Playlist,
                "Album" => ImportType::Album,
                _ => return Err(LoadMediaError {
                    error: "Import Type".to_string(),
                    message: "Invalid import type".to_string()
                })
            }
        },
        None => {
            match_type(&url, &source_type)
        }
    };

    let id: String = match source_type {
        SourceType::YouTube => {
            youtube_extract_id(&url)
        },
        SourceType::Spotify => {
            url.path().to_string().replace("/", "")
        },
        SourceType::Unknown => {
            return Err(LoadMediaError {
                error: "Source".to_string(),
                message: "Unknown source".to_string()
            })
        }
    };

    let source = Source {
        id,
        import_type,
        source_type
    };
    Ok(source)
}

pub async fn load_media(app: AppHandle,source_media_url: String, forced_source_type: Option<String>, force_import_type: Option<String>) -> Result<String, LoadMediaError> {
    let source = construct_source(source_media_url, forced_source_type, force_import_type)?;

    let result: Result<String, LoadMediaError> = match source.source_type {
        SourceType::YouTube => handle_youtube_video(app.clone(),&source).await,
        SourceType::Spotify => Ok("Success, Spotify, video".to_string()),
        SourceType::Unknown => Err(LoadMediaError {
            error: "Source".to_string(),
            message: "Unknown source".to_string()
        })
    };
    result
//   // matching hostname to source
//   const source = urlObject.hostname;
//   if (forcedSource === "YouTube" || hostIsYouTube(source)) {
//     return handleYoutubeVideo(urlObject);
//   }

//   if (forcedSource === "Spotify" || hostIsSpotify(source)) {
//     return "Error, Spotify, not implemented";
//   }

//   return "Error, ,could not find source";
// }

// async function handleYoutubeVideo(url: URL) {
//   const params = url.searchParams;
//   const source = url.hostname;

//   if (source === "youtu.be") {
//     const id = url.pathname.substring(1);
//     const response = await invoke("get_video_from_youtube_by_id", { id });
//     console.log(response);
//     return "Success, Youtube, video";
//   }
//   if (params.has("v")) {
//     const id = params.get("v");
//     const response = await invoke("get_video_from_youtube_by_id", { id });
//     console.log(response);
//     return "Success, YouTube, video";
//   }
//   if (params.has("list")) {
//     const id = params.get("list");
//     const response = await invoke("get_videos_from_youtube_by_playlist_id", {
//       id,
//     });
//     console.log(response);
//     return "Success, YouTube, playlist";
//   }
//   return "Error, YouTube, could not find video or playlist id";
// }

// function hostIsSpotify(host: string) {
//   return (
//     host === "www.spotify.com" ||
//     host === "spotify.com" ||
//     host === "open.spotify.com"
//   );
// }

// function hostIsYouTube(host: string) {
//   return (
//     host === "www.youtube.com" ||
//     host === "youtube.com" ||
//     host === "youtu.be" ||
//     host === "music.youtube.com"
//   );
// }
}