use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterPopularity {
    pub date: Option<String>,
    pub popularity: i64,
}
