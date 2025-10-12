use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterCashItemEquipment {
    pub date: Option<String>,
    pub character_gender: String,
    pub character_class: String,
    pub character_look_mode: String,
    pub preset_no: i64,
    pub cash_item_equipment_base: Vec<CashItemEquipment>,
    pub cash_item_equipment_preset_1: Vec<CashItemEquipment>,
    pub cash_item_equipment_preset_2: Vec<CashItemEquipment>,
    pub cash_item_equipment_preset_3: Vec<CashItemEquipment>,
    pub additional_cash_item_equipment_base: Vec<CashItemEquipment>,
    pub additional_cash_item_equipment_preset_1: Vec<CashItemEquipment>,
    pub additional_cash_item_equipment_preset_2: Vec<CashItemEquipment>,
    pub additional_cash_item_equipment_preset_3: Vec<CashItemEquipment>,
}

#[derive(Debug, Deserialize)]
pub struct CashItemEquipment {
    pub cash_item_equipment_part: String,
    pub cash_item_equipment_slot: String,
    pub cash_item_name: String,
    pub cash_item_icon: String,
    pub cash_item_description: Option<String>,
    pub cash_item_option: Vec<CashItemOption>,
    pub date_expire: Option<String>,
    pub date_option_expire: Option<String>,
    pub cash_item_label: Option<String>,
    pub cash_item_coloring_prism: Option<CashItemColoringPrism>,
    pub item_gender: Option<String>,
    pub skills: Vec<String>,
    pub freestyle_flag: String,
}

#[derive(Debug, Deserialize)]
pub struct CashItemColoringPrism {
    pub color_range: String,
    pub hue: i64,
    pub saturation: i64,
    pub value: i64,
}

#[derive(Debug, Deserialize)]
pub struct CashItemOption {
    pub option_type: String,
    pub option_value: String,
}
