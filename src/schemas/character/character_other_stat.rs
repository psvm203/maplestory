use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterOtherStat {
    pub date: Option<String>,
    pub other_stat: Vec<OtherStat>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct OtherStat {
    pub other_stat_type: String,
    pub stat_info: Vec<StatInfo>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct StatInfo {
    pub stat_name: String,
    pub stat_value: String,
}
