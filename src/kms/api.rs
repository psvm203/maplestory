use crate::{
    api::MaplestoryApi,
    kms::character::{achievement, character, character_list},
};

const API_KEY_HEADER_NAME: &str = "x-nxopen-api-key";

pub async fn get_character_list(api: &MaplestoryApi) -> character_list::CharacterList {
    reqwest::Client::new()
        .get(&format!("{}/maplestory/v1/character/list", &api.origin))
        .header(API_KEY_HEADER_NAME, &api.api_key)
        .send()
        .await
        .unwrap()
        .json::<character_list::CharacterList>()
        .await
        .unwrap()
}

pub async fn get_user_achievement(api: &MaplestoryApi) -> achievement::Achievement {
    reqwest::Client::new()
        .get(&format!("{}/maplestory/v1/id", &api.origin))
        .header(API_KEY_HEADER_NAME, &api.api_key)
        .send()
        .await
        .unwrap()
        .json::<achievement::Achievement>()
        .await
        .unwrap()
}

pub async fn get_id(api: &MaplestoryApi, character_name: String) -> character::Character {
    reqwest::Client::new()
        .get(&format!("{}/maplestory/v1/id", &api.origin))
        .header(API_KEY_HEADER_NAME, &api.api_key)
        .query(&[("character_name", character_name)])
        .send()
        .await
        .unwrap()
        .json::<character::Character>()
        .await
        .unwrap()
}
