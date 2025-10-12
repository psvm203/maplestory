use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CubeHistory {
    pub count: i64,
    pub next_cursor: Option<String>,
    pub cube_history: Vec<CubeHistoryElement>,
}

#[derive(Debug, Deserialize)]
pub struct CubeHistoryElement {
    pub id: String,
    pub character_name: String,
    pub date_create: String,
    pub cube_type: String,
    pub item_upgrade_result: String,
    pub miracle_time_flag: String,
    pub item_equipment_part: String,
    pub item_level: i64,
    pub target_item: String,
    pub potential_option_grade: String,
    pub additional_potential_option_grade: String,
    pub upgrade_guarantee: bool,
    pub upgrade_guarantee_count: i64,
    pub before_potential_option: Vec<PotentialOption>,
    pub before_additional_potential_option: Vec<PotentialOption>,
    pub after_potential_option: Vec<PotentialOption>,
    pub after_additional_potential_option: Vec<PotentialOption>,
}

#[derive(Debug, Deserialize)]
pub struct PotentialOption {
    pub value: String,
    pub grade: String,
}
