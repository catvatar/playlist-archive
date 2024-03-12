use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct YouTubeError {
    pub error: String,
    pub error_message: String,
}

impl Into<String> for YouTubeError {
    fn into(self) -> String {
        format!("Rs: Error: {}, Message: {}", self.error, self.error_message)
    }
} 

impl From<reqwest::Error> for YouTubeError {
    fn from(error: reqwest::Error) -> Self {
        YouTubeError{
            error: "reqwest".to_string(),
            error_message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for YouTubeError {
    fn from(error: std::io::Error) -> Self {
        YouTubeError{
            error: "std::io".to_string(),
            error_message: error.to_string(),
        }
    }
}

impl From<String> for YouTubeError {
    fn from(error: String) -> Self {
        YouTubeError{
            error: "std::string".to_string(),
            error_message: error,
        }
    }
}