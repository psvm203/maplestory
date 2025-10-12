use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterAndroidEquipment {
    pub date: Option<String>,
    pub android_name: Option<String>,
    pub android_nickname: Option<String>,
    pub android_icon: Option<String>,
    pub android_description: Option<String>,
    pub android_hair: AndroidHair,
    pub android_face: AndroidFace,
    pub android_skin: Option<AndroidSkin>,
    pub android_cash_item_equipment: Vec<AndroidCashItemEquipment>,
    pub android_ear_sensor_clip_flag: String,
    pub android_gender: Option<String>,
    pub android_grade: Option<String>,
    pub android_non_humanoid_flag: Option<String>,
    pub android_shop_usable_flag: Option<String>,
    pub preset_no: i64,
    pub android_preset_1: Option<AndroidPreset>,
    pub android_preset_2: Option<AndroidPreset>,
    pub android_preset_3: Option<AndroidPreset>,
}

#[derive(Debug, Deserialize)]
pub struct AndroidHair {
    pub hair_name: Option<String>,
    pub base_color: Option<String>,
    pub mix_color: Option<String>,
    pub mix_rate: String,
    pub freestyle_flag: String,
}

#[derive(Debug, Deserialize)]
pub struct AndroidFace {
    pub face_name: Option<String>,
    pub base_color: Option<String>,
    pub mix_color: Option<String>,
    pub mix_rate: String,
    pub freestyle_flag: String,
}

#[derive(Debug, Deserialize)]
pub struct AndroidSkin {
    pub skin_name: String,
    pub color_style: Option<String>,
    pub hue: i64,
    pub saturation: i64,
    pub brightness: i64,
}

#[derive(Debug, Deserialize)]
pub struct AndroidCashItemEquipment {
    pub cash_item_equipment_part: String,
    pub cash_item_equipment_slot: String,
    pub cash_item_name: String,
    pub cash_item_icon: String,
    pub cash_item_description: Option<String>,
    pub cash_item_option: Vec<Option<CashItemOption>>,
    pub date_expire: Option<String>,
    pub date_option_expire: Option<String>,
    pub cash_item_label: Option<String>,
    pub cash_item_coloring_prism: Option<CashItemColoringPrism>,
    pub android_item_gender: Option<String>,
    pub freestyle_flag: String,
}

#[derive(Debug, Deserialize)]
pub struct AndroidPreset {
    pub android_name: String,
    pub android_nickname: String,
    pub android_icon: String,
    pub android_description: String,
    pub android_gender: String,
    pub android_grade: String,
    pub android_skin: Option<String>,
    pub android_hair: AndroidFace,
    pub android_face: AndroidFace,
    pub android_ear_sensor_clip_flag: String,
    pub android_non_humanoid_flag: String,
    pub android_shop_usable_flag: String,
}

#[derive(Debug, Deserialize)]
pub struct CashItemOption {
    pub option_type: String,
    pub option_value: String,
}

#[derive(Debug, Deserialize)]
pub struct CashItemColoringPrism {
    pub color_range: String,
    pub hue: i64,
    pub saturation: i64,
    pub value: i64,
}
