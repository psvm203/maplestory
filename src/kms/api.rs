use crate::{api::MaplestoryApi, kms::character::character_list::CharacterList};

const X_NXOPEN_API_KEY: &str = "x-nxopen-api-key";

pub async fn get_character_list(api: &MaplestoryApi) -> CharacterList {
    let url =
        reqwest::Url::parse_with_params(&api.origin, [(X_NXOPEN_API_KEY, &api.api_key)]).unwrap();

    reqwest::get(url)
        .await
        .unwrap()
        .json::<CharacterList>()
        .await
        .unwrap()
}
