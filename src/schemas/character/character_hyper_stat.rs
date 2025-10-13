use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterHyperStat {
    pub date: Option<String>,
    pub character_class: String,
    pub use_preset_no: String,
    pub use_available_hyper_stat: i64,
    pub hyper_stat_preset_1: Vec<HyperStatPreset>,
    pub hyper_stat_preset_1_remain_point: i64,
    pub hyper_stat_preset_2: Vec<HyperStatPreset>,
    pub hyper_stat_preset_2_remain_point: i64,
    pub hyper_stat_preset_3: Vec<HyperStatPreset>,
    pub hyper_stat_preset_3_remain_point: i64,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct HyperStatPreset {
    pub stat_type: String,
    pub stat_point: Option<i64>,
    pub stat_level: i64,
    pub stat_increase: Option<String>,
}
