use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharacterLinkSkill {
    pub date: Option<String>,
    pub character_class: String,
    pub character_link_skill: Vec<CharacterLinkSkillElement>,
    pub character_link_skill_preset_1: Vec<Character>,
    pub character_link_skill_preset_2: Vec<Character>,
    pub character_link_skill_preset_3: Vec<Character>,
    pub character_owned_link_skill: Character,
    pub character_owned_link_skill_preset_1: Character,
    pub character_owned_link_skill_preset_2: Character,
    pub character_owned_link_skill_preset_3: Character,
}

#[derive(Debug, Deserialize)]
pub struct CharacterLinkSkillElement {
    pub skill_name: String,
    pub skill_description: String,
    pub skill_level: i64,
    pub skill_effect: String,
    pub skill_icon: String,
    pub skill_effect_next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub skill_name: String,
    pub skill_description: String,
    pub skill_level: i64,
    pub skill_effect: String,
    pub skill_icon: String,
}
