use crate::{
    api::MaplestoryApi,
    error::ApiError,
    prelude::error_message::ErrorMessage,
    schemas::{achievement, character, character_list},
};
use std::future::Future;

const API_KEY_HEADER_NAME: &str = "x-nxopen-api-key";

pub trait Kms {
    fn get_character_list(
        &self,
    ) -> impl Future<Output = Result<character_list::CharacterList, ApiError>> + Send;
    fn get_user_achievement(
        &self,
    ) -> impl Future<Output = Result<achievement::Achievement, ApiError>> + Send;
    fn get_id(
        &self,
        character_name: String,
    ) -> impl Future<Output = Result<character::Character, ApiError>> + Send;
}

impl Kms for MaplestoryApi {
    fn get_character_list(
        &self,
    ) -> impl Future<Output = Result<character_list::CharacterList, ApiError>> + Send {
        async move {
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
    }

    fn get_user_achievement(
        &self,
    ) -> impl Future<Output = Result<achievement::Achievement, ApiError>> + Send {
        async move {
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
    }

    fn get_id(
        &self,
        character_name: String,
    ) -> impl Future<Output = Result<character::Character, ApiError>> + Send {
        async move {
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
}
