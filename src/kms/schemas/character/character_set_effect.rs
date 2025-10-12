use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterSetEffect {
    pub date: Option<String>,
    pub set_effect: Vec<SetEffect>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SetEffect {
    pub set_name: String,
    pub total_set_count: i64,
    pub set_effect_info: Vec<Set>,
    pub set_option_full: Vec<Set>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Set {
    pub set_count: i64,
    pub set_option: String,
}
