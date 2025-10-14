use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ApiError {
    #[serde(rename = "OPENAPI00001")]
    InternalServerError,
    #[serde(rename = "OPENAPI00002")]
    Forbidden,
    #[serde(rename = "OPENAPI00003")]
    InvalidIdentifier,
    #[serde(rename = "OPENAPI00004")]
    InvalidParameter,
    #[serde(rename = "OPENAPI00005")]
    InvalidApiKey,
    #[serde(rename = "OPENAPI00006")]
    InvalidPath,
    #[serde(rename = "OPENAPI00007")]
    TooManyRequests,
    #[serde(rename = "OPENAPI00009")]
    DataNotReady,
    #[serde(rename = "OPENAPI00010")]
    GameUnderMaintenance,
    #[serde(rename = "OPENAPI00011")]
    ApiUnderMaintenance,
    SendRequestError,
    ParseError,
}
