use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterList {
    pub account_list: Vec<AccountList>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AccountList {
    pub account_id: String,
    pub character_list: Vec<CharacterListElement>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CharacterListElement {
    pub ocid: String,
    pub character_name: String,
    pub world_name: String,
    pub character_class: String,
    pub character_level: u32,
}
