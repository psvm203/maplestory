use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OverallRanking {
    pub ranking: Vec<Ranking>,
}

#[derive(Debug, Deserialize)]
pub struct Ranking {
    pub date: String,
    pub world_name: String,
    pub ranking: i64,
    pub character_name: String,
    pub character_level: i64,
    pub character_exp: i64,
    pub class_name: String,
    pub sub_class_name: String,
    pub character_popularity: i64,
    pub character_guildname: Option<String>,
}
