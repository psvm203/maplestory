use crate::{
    api::MaplestoryApi,
    error::ApiError,
    schemas::{
        achievement::Achievement, character::Character, character_basic::CharacterBasic,
        character_list::CharacterList,
    },
};

pub async fn get_character_list(api: &MaplestoryApi) -> Result<CharacterList, ApiError> {
    api.make_request("v1/character/list", None).await
}

pub async fn get_user_achievement(api: &MaplestoryApi) -> Result<Achievement, ApiError> {
    api.make_request("v1/user/achievement", None).await
}

pub async fn get_id(api: &MaplestoryApi, character_name: &str) -> Result<Character, ApiError> {
    api.make_request("v1/id", Some(&[("character_name", character_name)]))
        .await
}
