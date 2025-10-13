use crate::{
    api::MaplestoryApi,
    error::ApiError,
    schemas::{achievement, character, character_list},
};

pub async fn get_character_list(
    api: &MaplestoryApi,
) -> Result<character_list::CharacterList, ApiError> {
    api.make_request("v1/character/list", None).await
}

pub async fn get_user_achievement(
    api: &MaplestoryApi,
) -> Result<achievement::Achievement, ApiError> {
    api.make_request("v1/user/achievement", None).await
}

pub async fn get_id(
    api: &MaplestoryApi,
    character_name: &str,
) -> Result<character::Character, ApiError> {
    api.make_request("v1/id", Some(&[("character_name", character_name)]))
        .await
}
