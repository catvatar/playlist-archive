use serde::{Deserialize, Serialize, Serializer, Deserializer, de};

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
    pub thumbnail: Option<String>,
    pub comment: String,
}
impl PartialEq for Metadata{
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
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
    pub genres: Option<Vec<String>>,
    pub sources: Option<Vec<Source>>,
}
impl PartialEq for Song{
    fn eq(&self, other: &Self) -> bool {
        (self.metadata == other.metadata) && (self.artist == other.artist)
    }
}

#[derive(Debug)]
pub enum Visibility{
    Visible,
    Hidden,
}

impl Serialize for Visibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            Visibility::Visible => "Visible",
            Visibility::Hidden => "Hidden",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for Visibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VisibilityVisitor;

        impl<'de> de::Visitor<'de> for VisibilityVisitor {
            type Value = Visibility;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("either \"Visible\" or \"Hidden\"")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "Visible" => Ok(Visibility::Visible),
                    "Hidden" => Ok(Visibility::Hidden),
                    _ => Err(de::Error::unknown_variant(value, &["Visible", "Hidden"])),
                }
            }
        }

        deserializer.deserialize_str(VisibilityVisitor)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist{
    pub metadata: Metadata,
    pub visibility: Visibility,
    pub songs: Option<Vec<Song>>,
}