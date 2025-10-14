use crate::{
    error::ApiError,
    kms,
    schemas::{
        achievement::Achievement, character::Character, character_list::CharacterList,
        error_message::ErrorMessage,
    },
};
use serde::de::Deserialize;

const API_KEY_HEADER_NAME: &str = "x-nxopen-api-key";

pub enum Region {
    KMS,
    MSEA,
}

pub struct MaplestoryApi {
    pub region: Region,
    pub api_key: String,
    pub origin: String,
}

impl MaplestoryApi {
    pub fn builder() -> MaplestoryApiBuilder {
        MaplestoryApiBuilder::default()
    }

    pub(crate) async fn make_request<T>(
        &self,
        endpoint: &str,
        query_params: Option<&[(&'static str, &str)]>,
    ) -> Result<T, ApiError>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut request = reqwest::Client::new()
            .get(format!("{}/maplestory/{endpoint}", &self.origin))
            .header(API_KEY_HEADER_NAME, &self.api_key);

        if let Some(params) = query_params {
            request = request.query(params);
        }

        let response = request.send().await.or(Err(ApiError::SendRequestError))?;

        if response.status() != reqwest::StatusCode::OK {
            return Err(response
                .json::<ErrorMessage>()
                .await
                .or(Err(ApiError::ParseError))?
                .error
                .name);
        }

        response.json::<T>().await.or(Err(ApiError::ParseError))
    }

    pub async fn get_character_list(&self) -> Result<CharacterList, ApiError> {
        kms::get_character_list(self).await
    }

    pub async fn get_user_achievement(&self) -> Result<Achievement, ApiError> {
        kms::get_user_achievement(self).await
    }

    pub async fn get_id(&self, character_name: &str) -> Result<Character, ApiError> {
        kms::get_id(self, character_name).await
    }
}

#[derive(Default)]
pub struct MaplestoryApiBuilder {
    pub region: Option<Region>,
    pub api_key: Option<String>,
    pub origin: Option<String>,
}

impl MaplestoryApiBuilder {
    pub const fn new() -> Self {
        Self {
            region: None,
            api_key: None,
            origin: None,
        }
    }

    pub const fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn origin<S: Into<String>>(mut self, origin: S) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn build(self) -> Result<MaplestoryApi, &'static str> {
        let region = self.region.unwrap_or(Region::KMS);

        let api_key = self.api_key.ok_or("api_key is required")?;

        let origin = self
            .origin
            .unwrap_or_else(|| "https://open.api.nexon.com".to_owned());

        Ok(MaplestoryApi {
            region,
            api_key,
            origin,
        })
    }
}
