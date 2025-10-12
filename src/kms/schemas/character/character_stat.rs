use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterStat {
    pub date: Option<String>,
    pub character_class: String,
    pub final_stat: Vec<FinalStat>,
    pub remain_ap: i64,
}

#[derive(Debug, Deserialize)]
pub struct FinalStat {
    pub stat_name: String,
    pub stat_value: String,
}
