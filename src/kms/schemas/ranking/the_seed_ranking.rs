use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TheSeedRanking {
    pub ranking: Vec<Ranking>,
}

#[derive(Debug, Deserialize)]
pub struct Ranking {
    pub date: String,
    pub ranking: i64,
    pub theseed_floor: i64,
    pub theseed_time_record: i64,
    pub character_name: String,
    pub world_name: String,
    pub class_name: String,
    pub sub_class_name: String,
    pub character_level: i64,
}
