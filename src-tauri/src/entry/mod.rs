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
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata{
    pub title: String,
    pub author: String,
    pub tags: Option<Vec<Tag>>,
    pub date: Date,
    pub thumbnail: Option<String>,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Genre{
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source{
    pub url: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Song{
    pub metadata: Metadata,
    pub artist: Option<String>,
    pub genres: Option<Vec<Genre>>,
    pub sources: Option<Vec<Source>>,
}

// pub fn new_song(title: String, author: String, tags: Option<Vec<Tag>>, date: Date, thumbnail: Option<String>, comment: String, artist: Option<String>, genres: Option<Vec<Genre>>, sources: Option<Vec<Sources>>) -> Song{
//     Song{
//         title,
//         author,
//         tags,
//         date,
//         thumbnail,
//         comment,    
//         genres,
//         sources,
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist{
    pub metadata: Metadata,
    pub visibility: Option<bool>,
    pub songs: Option<Vec<Song>>,
}