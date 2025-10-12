use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterVMatrix {
    pub date: Option<String>,
    pub character_class: Option<String>,
    pub character_v_core_equipment: Vec<CharacterVCoreEquipment>,
    pub character_v_matrix_remain_slot_upgrade_point: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterVCoreEquipment {
    pub slot_id: String,
    pub slot_level: i64,
    pub v_core_name: Option<String>,
    pub v_core_level: i64,
    pub v_core_skill_1: Option<String>,
    pub v_core_skill_2: Option<String>,
    pub v_core_skill_3: Option<String>,
    pub v_core_type: Option<String>,
}
