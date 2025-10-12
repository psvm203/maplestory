use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Character {
    pub ocid: String,
}
