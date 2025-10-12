use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterPetEquipment {
    pub date: Option<String>,
    pub pet_1_name: Option<String>,
    pub pet_1_nickname: Option<String>,
    pub pet_1_icon: Option<String>,
    pub pet_1_description: Option<String>,
    pub pet_1_equipment: Option<PetEquipment>,
    pub pet_1_auto_skill: Option<PetAutoSkill>,
    pub pet_1_pet_type: Option<String>,
    pub pet_1_skill: Vec<String>,
    pub pet_1_date_expire: Option<String>,
    pub pet_1_appearance: Option<String>,
    pub pet_1_appearance_icon: Option<String>,
    pub pet_2_name: Option<String>,
    pub pet_2_nickname: Option<String>,
    pub pet_2_icon: Option<String>,
    pub pet_2_description: Option<String>,
    pub pet_2_equipment: PetEquipment,
    pub pet_2_auto_skill: Option<PetAutoSkill>,
    pub pet_2_pet_type: Option<String>,
    pub pet_2_skill: Vec<String>,
    pub pet_2_date_expire: Option<String>,
    pub pet_2_appearance: Option<String>,
    pub pet_2_appearance_icon: Option<String>,
    pub pet_3_name: Option<String>,
    pub pet_3_nickname: Option<String>,
    pub pet_3_icon: Option<String>,
    pub pet_3_description: Option<String>,
    pub pet_3_equipment: PetEquipment,
    pub pet_3_auto_skill: Option<PetAutoSkill>,
    pub pet_3_pet_type: Option<String>,
    pub pet_3_skill: Vec<String>,
    pub pet_3_date_expire: Option<String>,
    pub pet_3_appearance: Option<String>,
    pub pet_3_appearance_icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PetEquipment {
    pub item_name: Option<String>,
    pub item_icon: Option<String>,
    pub item_description: Option<String>,
    pub item_option: Vec<ItemOption>,
    pub scroll_upgrade: i64,
    pub scroll_upgradable: i64,
    pub item_shape: Option<String>,
    pub item_shape_icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ItemOption {
    pub option_type: String,
    pub option_value: String,
}

#[derive(Debug, Deserialize)]
pub struct PetAutoSkill {
    pub skill_1: Option<String>,
    pub skill_1_icon: Option<String>,
    pub skill_2: Option<String>,
    pub skill_2_icon: Option<String>,
}
