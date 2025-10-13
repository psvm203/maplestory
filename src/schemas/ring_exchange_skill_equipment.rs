use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct LinkExchangeSkillEquipment {
    pub date: Option<String>,
    pub character_class: String,
    pub special_ring_exchange_name: Option<String>,
    pub special_ring_exchange_level: Option<i64>,
}
