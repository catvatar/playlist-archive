use serde::{Deserialize, Serialize};
use crate::entry;
use chrono;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PlaylistResponse {
    kind: String,
    etag: String,
    next_page_token: String,
    items: Vec<PlaylistItem>,
    page_info: PageInfo,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PlaylistItem {
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
    video_owner_channel_title: String,
    video_owner_channel_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Thumbnails {
    default: Thumbnail,
    medium: Thumbnail,
    high: Thumbnail,
    standard: Thumbnail,
    maxres: Thumbnail,
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