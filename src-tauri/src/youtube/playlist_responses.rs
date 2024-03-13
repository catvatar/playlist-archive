use serde::{Serialize, Deserialize};
use crate::entry;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistListResponse {
    kind: String,
    etag: String,
    page_info: PageInfo,
    pub items: Vec<PlaylistItem>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    total_results: u32,
    results_per_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistItem {
    kind: String,
    etag: String,
    id: String,
    snippet: Snippet,
}

impl Into<entry::Playlist> for &PlaylistItem {
    fn into(self) -> entry::Playlist {
        let metadata : entry::Metadata = entry::Metadata{
            title: self.snippet.title.clone(),
            author: "catvatar".to_string(),
            tags: None,
            date: chrono::offset::Local::now().date_naive().to_string().into(),
            thumbnail: None,
            comment: "Write your comment here.".to_string(),
        };
        let playlist : entry::Playlist = entry::Playlist{
            metadata,
            visibility: entry::Visibility::Visible,
            songs: None,
        };
        playlist
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Snippet {
    published_at: String,
    channel_id: String,
    title: String,
    description: String,
    thumbnails: Thumbnails,
    channel_title: String,
    localized: Localized,
}

#[derive(Debug, Serialize, Deserialize)]
struct Thumbnails {
    default: Thumbnail,
    medium: Thumbnail,
    high: Thumbnail,
    standard: Thumbnail,
    maxres: Thumbnail,
}

#[derive(Debug, Serialize, Deserialize)]
struct Thumbnail {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Localized {
    title: String,
    description: String,
}