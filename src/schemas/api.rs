use crate::{
    api::MaplestoryApi,
    schemas::character::{achievement, character, character_list},
};

const API_KEY_HEADER_NAME: &str = "x-nxopen-api-key";

pub trait Kms {
    async fn get_character_list(&self) -> character_list::CharacterList;
    async fn get_user_achievement(&self) -> achievement::Achievement;
    async fn get_id(&self, character_name: String) -> character::Character;
}

impl Kms for MaplestoryApi {
    async fn get_character_list(&self) -> character_list::CharacterList {
        reqwest::Client::new()
            .get(&format!("{}/maplestory/v1/character/list", &self.origin))
            .header(API_KEY_HEADER_NAME, &self.api_key)
            .send()
            .await
            .unwrap()
            .json::<character_list::CharacterList>()
            .await
            .unwrap()
    }

    async fn get_user_achievement(&self) -> achievement::Achievement {
        reqwest::Client::new()
            .get(&format!("{}/maplestory/v1/id", &self.origin))
            .header(API_KEY_HEADER_NAME, &self.api_key)
            .send()
            .await
            .unwrap()
            .json::<achievement::Achievement>()
            .await
            .unwrap()
    }

    async fn get_id(&self, character_name: String) -> character::Character {
        reqwest::Client::new()
            .get(&format!("{}/maplestory/v1/id", &self.origin))
            .header(API_KEY_HEADER_NAME, &self.api_key)
            .query(&[("character_name", character_name)])
            .send()
            .await
            .unwrap()
            .json::<character::Character>()
            .await
            .unwrap()
    }
}
