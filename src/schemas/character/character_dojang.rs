use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterDojang {
    pub date: Option<String>,
    pub character_class: String,
    pub world_name: String,
    pub dojang_best_floor: i64,
    pub date_dojang_record: Option<String>,
    pub dojang_best_time: i64,
}
