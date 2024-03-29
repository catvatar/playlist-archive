use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag{
    pub name: String,
    pub color: String,
}

impl From<String> for Tag{
    fn from(name: String) -> Self{
        Tag{
            name,
            color: String::from("blank"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Date{
    date: String,
}

impl From<String> for Date{
    fn from(date: String) -> Self{
        Date{
            date,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata{
    pub title: String,
    pub author: String,
    pub tags: Option<Vec<Tag>>,
    pub date: Date,
    pub thumbnail: Option<Thumbnails>,
    pub comment: String,
}
impl PartialEq for Metadata{
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnails {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SourceType{
    YouTube,
    Spotify,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ImportType{
    Song,
    Playlist,
    Album,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Source{
    pub id: String,
    pub import_type: ImportType,
    pub source_type: SourceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Song{
    pub metadata: Metadata,
    pub artist: Option<String>,
    pub genres: Option<Vec<String>>,
    pub sources: Option<Vec<Source>>,
}
impl PartialEq for Song{
    fn eq(&self, other: &Self) -> bool {
        (self.metadata == other.metadata) && (self.artist == other.artist)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Visibility{
    Visible,
    Hidden,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist{
    pub metadata: Metadata,
    pub visibility: Visibility,
    pub songs: Option<Vec<Song>>,
}