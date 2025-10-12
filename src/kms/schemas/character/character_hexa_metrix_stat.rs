use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterHexaMetrixStat {
    pub date: Option<String>,
    pub character_class: Option<String>,
    pub character_hexa_stat_core: Vec<HexaStatCore>,
    pub character_hexa_stat_core_2: Option<Vec<HexaStatCore>>,
    pub character_hexa_stat_core_3: Option<Vec<HexaStatCore>>,
    pub preset_hexa_stat_core: Vec<HexaStatCore>,
    pub preset_hexa_stat_core_2: Option<Vec<HexaStatCore>>,
    pub preset_hexa_stat_core_3: Option<Vec<HexaStatCore>>,
}

#[derive(Debug, Deserialize)]
pub struct HexaStatCore {
    pub slot_id: String,
    pub main_stat_name: Option<String>,
    pub sub_stat_name_1: Option<String>,
    pub sub_stat_name_2: Option<String>,
    pub main_stat_level: i64,
    pub sub_stat_level_1: i64,
    pub sub_stat_level_2: i64,
    pub stat_grade: i64,
}
