use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ApiError {
    #[serde(rename = "OPENAPI00001")]
    InternalServerError,
    #[serde(rename = "OPENAPI00002")]
    UnauthorizedAccess,
    #[serde(rename = "OPENAPI00003")]
    InvalidIdentifier,
    #[serde(rename = "OPENAPI00004")]
    InvalidParameter,
    #[serde(rename = "OPENAPI00005")]
    InvalidApiKey,
    #[serde(rename = "OPENAPI00006")]
    InvalidPath,
    #[serde(rename = "OPENAPI00007")]
    LimitExceeded,
    #[serde(rename = "OPENAPI00009")]
    DataBeingPrepared,
    #[serde(rename = "OPENAPI00010")]
    ServiceUnderMaintenance,
    #[serde(rename = "OPENAPI00011")]
    ApiUnderMaintenance,
    SendRequestError,
    ParseError,
}
