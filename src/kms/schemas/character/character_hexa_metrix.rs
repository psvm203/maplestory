use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterHexaMetrix {
    pub date: Option<String>,
    pub character_hexa_core_equipment: Option<Vec<CharacterHexaCoreEquipment>>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterHexaCoreEquipment {
    pub hexa_core_name: String,
    pub hexa_core_level: i64,
    pub hexa_core_type: String,
    pub linked_skill: Vec<LinkedSkill>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct LinkedSkill {
    pub hexa_skill_id: String,
}
