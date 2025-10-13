use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterStat {
    pub date: Option<String>,
    pub character_class: String,
    pub final_stat: Vec<FinalStat>,
    pub remain_ap: i64,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FinalStat {
    pub stat_name: String,
    pub stat_value: String,
}
