use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub ouid: String,
}
