use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterHexaMetrix {
    pub date: Option<String>,
    pub character_hexa_core_equipment: Option<Vec<CharacterHexaCoreEquipment>>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterHexaCoreEquipment {
    pub hexa_core_name: String,
    pub hexa_core_level: i64,
    pub hexa_core_type: String,
    pub linked_skill: Vec<LinkedSkill>,
}

#[derive(Debug, Deserialize)]
pub struct LinkedSkill {
    pub hexa_skill_id: String,
}
