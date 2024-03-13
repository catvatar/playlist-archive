use serde::{Deserialize, Serialize};
use crate::entry;
use chrono;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct VideoListResponse {
    kind: String,
    etag: String,
    pub items: Vec<Video>,
    page_info: Option<PageInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    kind: String,
    etag: String,
    id: String,
    snippet: Snippet,
    content_details: Option<ContentDetails>,
    statistics: Option<Statistics>,
}

impl Into<entry::Song> for &Video {
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
            artist: self.snippet.channel_title.clone().into(),
            genres: self.snippet.tags.clone(),
            sources: vec![entry::Source{
                id: self.id.clone(),
                source: entry::SourceType::Youtube,
            },].into(),
        };
        song
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Snippet {
    published_at: String,
    channel_id: String,
    title: String,
    description: String,
    thumbnails: Thumbnails,
    channel_title: String,
    tags: Option<Vec<String>>,
    category_id: String,
    live_broadcast_content: String,
    localized: Localized,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thumbnails {
    default: Thumbnail,
    medium: Thumbnail,
    high: Thumbnail,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thumbnail {
    url: String,
    width: i32,
    height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Localized {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ContentDetails {
    duration: String,
    dimension: String,
    definition: String,
    caption: String,
    licensed_content: bool,
    content_rating: ContentRating,
    projection: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentRating {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Statistics {
    view_count: String,
    like_count: String,
    favorite_count: String,
    comment_count: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PageInfo {
    total_results: i32,
    results_per_page: i32,
}