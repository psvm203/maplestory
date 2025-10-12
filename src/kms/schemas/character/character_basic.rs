use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterBasic {
    pub date: Option<String>,
    pub character_name: String,
    pub world_name: String,
    pub character_gender: String,
    pub character_class: String,
    pub character_class_level: String,
    pub character_level: u32,
    pub character_exp: u64,
    pub character_exp_rate: String,
    pub character_guild_name: Option<String>,
    pub character_image: String,
    pub character_date_create: String,
    pub access_flag: String,
    pub liberation_quest_clear: String,
}
