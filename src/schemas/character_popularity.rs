use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterPopularity {
    pub date: Option<String>,
    pub popularity: i64,
}
