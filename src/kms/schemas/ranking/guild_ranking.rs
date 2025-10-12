use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GuildRanking {
    pub ranking: Vec<Ranking>,
}

#[derive(Debug, Deserialize)]
pub struct Ranking {
    pub date: String,
    pub world_name: String,
    pub guild_name: String,
    pub guild_level: i64,
    pub guild_mark: String,
    pub guild_point: i64,
    pub ranking: i64,
    pub guild_master_name: String,
}
