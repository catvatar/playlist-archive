use serde::{Deserialize, Serialize};
use crate::entry;
use chrono;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemsResponse {
    kind: String,
    etag: String,
    pub next_page_token: Option<String>,
    pub items: Vec<PlaylistItem>,
    page_info: PageInfo,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    kind: String,
    etag: String,
    id: String,
    snippet: Snippet,
}

impl Into<entry::Song> for &PlaylistItem {
    fn into(self) -> entry::Song {
        let metadata : entry::Metadata = entry::Metadata{
            title: self.snippet.title.clone(),
            author: "catvatar".to_string(),
            tags: None,
            date: chrono::offset::Local::now().date_naive().to_string().into(),
            thumbnail: None,
            comment: "Write your comment here.".to_string(),
        };
        let song : entry::Song = entry::Song{
            metadata,
            artist: self.snippet.video_owner_channel_title.clone().into(),
            genres: None,
            sources: vec![entry::Source{
                id: self.snippet.resource_id.video_id.clone(),
                source: entry::SourceType::Youtube,
            },].into(),
        };
        song
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Snippet {
    published_at: String,
    channel_id: String,
    title: String,
    description: String,
    thumbnails: Thumbnails,
    channel_title: String,
    playlist_id: String,
    position: u32,
    resource_id: ResourceId,
    video_owner_channel_title: Option<String>,
    video_owner_channel_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Thumbnails {
    default: Option<Thumbnail>,
    medium: Option<Thumbnail>,
    high: Option<Thumbnail>,
    standard: Option<Thumbnail>,
    maxres: Option<Thumbnail>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Thumbnail {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResourceId {
    kind: String,
    video_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    total_results: u32,
    results_per_page: u32,
}