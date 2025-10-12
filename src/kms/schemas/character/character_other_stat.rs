use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterOtherStat {
    pub date: Option<String>,
    pub other_stat: Vec<OtherStat>,
}

#[derive(Debug, Deserialize)]
pub struct OtherStat {
    pub other_stat_type: String,
    pub stat_info: Vec<StatInfo>,
}

#[derive(Debug, Deserialize)]
pub struct StatInfo {
    pub stat_name: String,
    pub stat_value: String,
}
