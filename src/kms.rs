use crate::{
    api::MaplestoryApi,
    error::ApiError,
    macros::Param,
    params,
    schemas::{achievement::Achievement, character::Character, character_list::CharacterList},
};

pub async fn get_character_list(api: &MaplestoryApi) -> Result<CharacterList, ApiError> {
    api.make_request("v1/character/list", params!()).await
}

pub async fn get_user_achievement(api: &MaplestoryApi) -> Result<Achievement, ApiError> {
    api.make_request("v1/user/achievement", params!()).await
}

pub async fn get_id(api: &MaplestoryApi, character_name: &str) -> Result<Character, ApiError> {
    api.make_request("v1/id", params!(character_name)).await
}
