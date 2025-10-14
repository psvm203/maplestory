use crate::{
    api::MaplestoryApi,
    error::ApiError,
    macros::Param,
    params,
    schemas::{
        achievement::Achievement, character::Character, character_ability::CharacterAbility,
        character_android_equipment::CharacterAndroidEquipment, character_basic::CharacterBasic,
        character_beauty_equipment::CharacterBeautyEquipment,
        character_cash_item_equipment::CharacterCashItemEquipment,
        character_dojang::CharacterDojang, character_hexa_matrix::CharacterHexaMatrix,
        character_hexa_matrix_stat::CharacterHexaMatrixStat,
        character_hyper_stat::CharacterHyperStat, character_item_equipment::CharacterItemEquipment,
        character_link_skill::CharacterLinkSkill, character_list::CharacterList,
        character_other_stat::CharacterOtherStat, character_pet_equipment::CharacterPetEquipment,
        character_popularity::CharacterPopularity, character_propensity::CharacterPropensity,
        character_set_effect::CharacterSetEffect, character_skill::CharacterSkill,
        character_stat::CharacterStat, character_symbol_equipment::CharacterSymbolEquipment,
        character_v_matrix::CharacterVMatrix,
        ring_exchange_skill_equipment::RingExchangeSkillEquipment,
    },
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

pub async fn get_character_basic(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterBasic, ApiError> {
    api.make_request("v1/character/basic", params!(ocid, date))
        .await
}

pub async fn get_character_popularity(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterPopularity, ApiError> {
    api.make_request("v1/character/popularity", params!(ocid, date))
        .await
}

pub async fn get_character_stat(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterStat, ApiError> {
    api.make_request("v1/character/stat", params!(ocid, date))
        .await
}

pub async fn get_character_hyperstat(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterHyperStat, ApiError> {
    api.make_request("v1/character/hyper-stat", params!(ocid, date))
        .await
}

pub async fn get_character_propensity(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterPropensity, ApiError> {
    api.make_request("v1/character/propensity", params!(ocid, date))
        .await
}

pub async fn get_character_ability(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterAbility, ApiError> {
    api.make_request("v1/character/ability", params!(ocid, date))
        .await
}

pub async fn get_character_item_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterItemEquipment, ApiError> {
    api.make_request("v1/character/item-equipment", params!(ocid, date))
        .await
}

pub async fn get_character_cashitem_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterCashItemEquipment, ApiError> {
    api.make_request("v1/character/cashitem-equipment", params!(ocid, date))
        .await
}

pub async fn get_character_symbol_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterSymbolEquipment, ApiError> {
    api.make_request("v1/character/symbol-equipment", params!(ocid, date))
        .await
}

pub async fn get_character_set_effect(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterSetEffect, ApiError> {
    api.make_request("v1/character/set-effect", params!(ocid, date))
        .await
}

pub async fn get_character_beauty_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterBeautyEquipment, ApiError> {
    api.make_request("v1/character/beauty-equipment", params!(ocid, date))
        .await
}

pub async fn get_character_android_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterAndroidEquipment, ApiError> {
    api.make_request("v1/character/android-equipment", params!(ocid, date))
        .await
}

pub async fn get_character_pet_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterPetEquipment, ApiError> {
    api.make_request("v1/character/pet-equipment", params!(ocid, date))
        .await
}

pub async fn get_character_skill(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterSkill, ApiError> {
    api.make_request("v1/character/skill", params!(ocid, date))
        .await
}

pub async fn get_character_link_skill(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterLinkSkill, ApiError> {
    api.make_request("v1/character/link-skill", params!(ocid, date))
        .await
}

pub async fn get_character_vmatrix(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterVMatrix, ApiError> {
    api.make_request("v1/character/vmatrix", params!(ocid, date))
        .await
}

pub async fn get_character_hexamatrix(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterHexaMatrix, ApiError> {
    api.make_request("v1/character/hexamatrix", params!(ocid, date))
        .await
}

pub async fn get_character_hexamatrix_stat(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterHexaMatrixStat, ApiError> {
    api.make_request("v1/character/hexamatrix-stat", params!(ocid, date))
        .await
}

pub async fn get_character_dojang(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterDojang, ApiError> {
    api.make_request("v1/character/dojang", params!(ocid, date))
        .await
}

pub async fn get_character_other_stat(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterOtherStat, ApiError> {
    api.make_request("v1/character/other-stat", params!(ocid, date))
        .await
}

pub async fn get_character_ring_exchange_skill_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<RingExchangeSkillEquipment, ApiError> {
    api.make_request(
        "v1/character/ring-exchange-skill-equipment",
        params!(ocid, date),
    )
    .await
}
