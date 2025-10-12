use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DojangRanking {
    pub ranking: Vec<Ranking>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Ranking {
    pub date: String,
    pub ranking: i64,
    pub dojang_floor: i64,
    pub dojang_time_record: i64,
    pub character_name: String,
    pub world_name: String,
    pub class_name: String,
    pub sub_class_name: String,
    pub character_level: i64,
}
