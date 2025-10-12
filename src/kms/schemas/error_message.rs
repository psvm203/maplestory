use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorMessage {
    pub error: Error,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub name: String,
    pub message: String,
}
