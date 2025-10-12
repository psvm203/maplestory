use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterSkill {
    pub date: Option<String>,
    pub character_class: String,
    pub character_skill_grade: Option<String>,
    pub character_skill: Vec<CharacterSkillElement>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterSkillElement {
    pub skill_name: String,
    pub skill_description: String,
    pub skill_level: i64,
    pub skill_effect: Option<String>,
    pub skill_icon: String,
    pub skill_effect_next: Option<String>,
}
