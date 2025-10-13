use crate::schemas::{self, api::Kms};

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
        MaplestoryApiBuilder::new()
    }

    pub async fn get_character_list(&self) -> schemas::character::character_list::CharacterList {
        Kms::get_character_list(self).await
    }

    pub async fn get_user_achievement(&self) -> schemas::character::achievement::Achievement {
        Kms::get_user_achievement(self).await
    }

    pub async fn get_id<S: Into<String>>(
        &self,
        character_name: S,
    ) -> schemas::character::character::Character {
        Kms::get_id(self, character_name.into()).await
    }
}

pub struct MaplestoryApiBuilder {
    pub region: Option<Region>,
    pub api_key: Option<String>,
    pub origin: Option<String>,
}

impl MaplestoryApiBuilder {
    pub fn new() -> Self {
        Self {
            region: None,
            api_key: None,
            origin: None,
        }
    }

    pub fn region(mut self, region: Region) -> Self {
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
