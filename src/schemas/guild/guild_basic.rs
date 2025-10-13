use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GuildBasic {
    pub date: Option<String>,
    pub world_name: String,
    pub guild_name: String,
    pub guild_level: i64,
    pub guild_fame: i64,
    pub guild_point: i64,
    pub guild_master_name: String,
    pub guild_member_count: i64,
    pub guild_member: Vec<String>,
    pub guild_skill: Vec<GuildSkill>,
    pub guild_noblesse_skill: Vec<GuildSkill>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GuildSkill {
    pub skill_name: String,
    pub skill_description: String,
    pub skill_level: i64,
    pub skill_effect: String,
    pub skill_icon: String,
}
