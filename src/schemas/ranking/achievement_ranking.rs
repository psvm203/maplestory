use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AchievementRanking {
    pub ranking: Vec<Ranking>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Ranking {
    pub date: String,
    pub ranking: i64,
    pub trophy_score: i64,
    pub trophy_grade: String,
    pub character_name: String,
    pub world_name: String,
    pub class_name: String,
    pub sub_class_name: String,
}
