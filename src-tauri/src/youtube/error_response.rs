use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    error: Error,
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
