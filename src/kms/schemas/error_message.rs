use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ErrorMessage {
    pub error: Error,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Error {
    pub name: String,
    pub message: String,
}
