use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct StarforceHistory {
    pub count: i64,
    pub next_cursor: Option<String>,
    pub starforce_history: Vec<StarforceHistoryElement>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct StarforceHistoryElement {
    pub id: String,
    pub item_upgrade_result: String,
    pub before_starforce_count: i64,
    pub after_starforce_count: i64,
    pub starforce_event_list: Option<Vec<StarforceEventList>>,
    pub starcatch_result: String,
    pub superior_item_flag: String,
    pub destroy_defence: String,
    pub chance_time: String,
    pub event_field_flag: String,
    pub upgrade_item: String,
    pub protect_shield: String,
    pub bonus_stat_upgrade: String,
    pub character_name: String,
    pub world_name: String,
    pub target_item: String,
    pub date_create: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct StarforceEventList {
    pub success_rate: Option<String>,
    pub destroy_decrease_rate: Option<String>,
    pub cost_discount_rate: Option<String>,
    pub plus_value: Option<String>,
    pub starforce_event_range: String,
}
