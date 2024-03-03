use serde::{Deserialize, Serialize};
use crate::entry;
use chrono;
// use crate::entry::Metadata as EntryMetadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoListResponse {
    kind: String,
    etag: String,
    items: Vec<Video>,
    page_info: Option<PageInfo>,
}

impl Into<entry::Song> for VideoListResponse {
    fn into(self) -> entry::Song {
        let metadata : entry::Metadata = entry::Metadata{
            title: self.items[0].snippet.title.clone(),
            author: "catvatar".to_string(),
            tags: None,
            date: chrono::offset::Local::now().date_naive().to_string().into(),
            thumbnail: None,
            comment: "Write your comment here.".to_string(),
        };
        let song : entry::Song = entry::Song{
            metadata,
            artist: self.items[0].snippet.channel_title.clone().into(),
            genres: self.items[0].snippet.tags.clone(),
            sources: vec![entry::Source{
                url: "https://www.youtube.com/watch?v=".to_string() + &self.items[0].id,
                source: "youtube".to_string(),
            },].into(),
        };
        song
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Video {
    kind: String,
    etag: String,
    id: String,
    snippet: Snippet,
    content_details: Option<ContentDetails>,
    statistics: Option<Statistics>,
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


#[derive(Serialize, Deserialize, Debug)]
struct Error {
    code: i32,
    message: String,
    errors: Vec<ErrorDetail>,
    status: String,
    details: Vec<Detail>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorDetail {
    message: String,
    domain: String,
    reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Detail {
    #[serde(rename = "@type")]
    type_: String,
    reason: String,
    domain: String,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    service: String,
}
