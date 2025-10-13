use crate::{
    api::MaplestoryApi,
    error::ApiError,
    prelude::error_message::ErrorMessage,
    schemas::{achievement, character, character_list},
};

const API_KEY_HEADER_NAME: &str = "x-nxopen-api-key";

pub trait Kms {
    async fn get_character_list(&self) -> Result<character_list::CharacterList, ApiError>;
    async fn get_user_achievement(&self) -> Result<achievement::Achievement, ApiError>;
    async fn get_id(&self, character_name: String) -> Result<character::Character, ApiError>;
}

impl Kms for MaplestoryApi {
    async fn get_character_list(&self) -> Result<character_list::CharacterList, ApiError> {
        let response = reqwest::Client::new()
            .get(format!("{}/maplestory/v1/character/list", &self.origin))
            .header(API_KEY_HEADER_NAME, &self.api_key)
            .send()
            .await
            .or(Err(ApiError::SendRequestError))?;

        if response.status() != reqwest::StatusCode::OK {
            return Err(response
                .json::<ErrorMessage>()
                .await
                .or(Err(ApiError::ParseError))?
                .error
                .name);
        }

        response
            .json::<character_list::CharacterList>()
            .await
            .or(Err(ApiError::ParseError))
    }

    async fn get_user_achievement(&self) -> Result<achievement::Achievement, ApiError> {
        let response = reqwest::Client::new()
            .get(format!("{}/maplestory/v1/id", &self.origin))
            .header(API_KEY_HEADER_NAME, &self.api_key)
            .send()
            .await
            .or(Err(ApiError::SendRequestError))?;

        if response.status() != reqwest::StatusCode::OK {
            return Err(response
                .json::<ErrorMessage>()
                .await
                .or(Err(ApiError::ParseError))?
                .error
                .name);
        }

        response
            .json::<achievement::Achievement>()
            .await
            .or(Err(ApiError::ParseError))
    }

    async fn get_id(&self, character_name: String) -> Result<character::Character, ApiError> {
        let response = reqwest::Client::new()
            .get(&format!("{}/maplestory/v1/id", &self.origin))
            .header(API_KEY_HEADER_NAME, &self.api_key)
            .query(&[("character_name", character_name)])
            .send()
            .await
            .or(Err(ApiError::SendRequestError))?;

        if response.status() != reqwest::StatusCode::OK {
            return Err(response
                .json::<ErrorMessage>()
                .await
                .or(Err(ApiError::ParseError))?
                .error
                .name);
        }

        response
            .json::<character::Character>()
            .await
            .or(Err(ApiError::ParseError))
    }
}
