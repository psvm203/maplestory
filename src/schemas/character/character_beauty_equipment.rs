use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterBeautyEquipment {
    pub date: Option<String>,
    pub character_gender: String,
    pub character_class: String,
    pub character_hair: CharacterHair,
    pub character_face: CharacterFace,
    pub character_skin: CharacterSkin,
    pub additional_character_hair: Option<CharacterHair>,
    pub additional_character_face: Option<CharacterFace>,
    pub additional_character_skin: Option<CharacterSkin>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterHair {
    pub hair_name: String,
    pub base_color: String,
    pub mix_color: Option<String>,
    pub mix_rate: String,
    pub freestyle_flag: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterFace {
    pub face_name: String,
    pub base_color: String,
    pub mix_color: Option<String>,
    pub mix_rate: String,
    pub freestyle_flag: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterSkin {
    pub skin_name: String,
    pub color_style: Option<String>,
    pub hue: Option<i64>,
    pub saturation: Option<i64>,
    pub brightness: Option<i64>,
}
