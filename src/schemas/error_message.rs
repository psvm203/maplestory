use crate::error::ApiError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ErrorMessage {
    pub error: Error,
}

#[derive(Deserialize)]
pub struct Error {
    pub name: ApiError,
    pub message: String,
}
