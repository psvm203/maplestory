use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UnionRanking {
    pub ranking: Vec<Ranking>,
}

#[derive(Debug, Deserialize)]
pub struct Ranking {
    pub date: String,
    pub ranking: i64,
    pub character_name: String,
    pub world_name: String,
    pub class_name: String,
    pub sub_class_name: String,
    pub union_level: i64,
    pub union_power: i64,
}
