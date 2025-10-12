use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterAbility {
    pub date: Option<String>,
    pub ability_grade: String,
    pub ability_info: Vec<AbilityInfo>,
    pub remain_fame: i64,
    pub preset_no: i64,
    pub ability_preset_1: AbilityPreset,
    pub ability_preset_2: AbilityPreset,
    pub ability_preset_3: AbilityPreset,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AbilityInfo {
    pub ability_no: String,
    pub ability_grade: String,
    pub ability_value: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AbilityPreset {
    pub ability_preset_grade: String,
    pub ability_info: Vec<AbilityInfo>,
}
