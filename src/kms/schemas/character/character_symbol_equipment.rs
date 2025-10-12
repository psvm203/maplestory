use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterSymbolEquipment {
    pub date: Option<String>,
    pub character_class: String,
    pub symbol: Vec<Symbol>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Symbol {
    pub symbol_name: String,
    pub symbol_icon: String,
    pub symbol_description: String,
    pub symbol_force: String,
    pub symbol_level: i64,
    pub symbol_str: String,
    pub symbol_dex: String,
    pub symbol_int: String,
    pub symbol_luk: String,
    pub symbol_hp: String,
    pub symbol_drop_rate: String,
    pub symbol_meso_rate: String,
    pub symbol_exp_rate: String,
    pub symbol_growth_count: i64,
    pub symbol_require_growth_count: i64,
}
