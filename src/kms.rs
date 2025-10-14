use crate::{
    api::MaplestoryApi,
    error::ApiError,
    macros::Param,
    params,
    schemas::{
        achievement::Achievement, achievement_ranking::AchievementRanking,
        cashshop_notice_detail::CashshopNoticeDetail, cashshop_notice_list::CashshopNoticeList,
        character::Character, character_ability::CharacterAbility,
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
        character_v_matrix::CharacterVMatrix, cube_history::CubeHistory,
        dojang_ranking::DojangRanking, event_notice_detail::EventNoticeDetail,
        event_notice_list::EventNoticeList, guild::Guild, guild_basic::GuildBasic,
        guild_ranking::GuildRanking, notice_detail::NoticeDetail, notice_list::NoticeList,
        overall_ranking::OverallRanking, potential_history::PotentialHistory,
        ring_exchange_skill_equipment::RingExchangeSkillEquipment,
        starforce_history::StarforceHistory, the_seed_ranking::TheSeedRanking, union::Union,
        union_artifact::UnionArtifact, union_champion::UnionChampion, union_raider::UnionRaider,
        union_ranking::UnionRanking, update_notice_detail::UpdateNoticeDetail,
        update_notice_list::UpdateNoticeList, user::User,
    },
};

pub(crate) async fn get_character_list(api: &MaplestoryApi) -> Result<CharacterList, ApiError> {
    api.make_request("v1/character/list", params!()).await
}

pub(crate) async fn get_user_achievement(api: &MaplestoryApi) -> Result<Achievement, ApiError> {
    api.make_request("v1/user/achievement", params!()).await
}

pub(crate) async fn get_id(
    api: &MaplestoryApi,
    character_name: &str,
) -> Result<Character, ApiError> {
    api.make_request("v1/id", params!(character_name)).await
}

pub(crate) async fn get_character_basic(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterBasic, ApiError> {
    api.make_request("v1/character/basic", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_popularity(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterPopularity, ApiError> {
    api.make_request("v1/character/popularity", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_stat(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterStat, ApiError> {
    api.make_request("v1/character/stat", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_hyperstat(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterHyperStat, ApiError> {
    api.make_request("v1/character/hyper-stat", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_propensity(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterPropensity, ApiError> {
    api.make_request("v1/character/propensity", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_ability(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterAbility, ApiError> {
    api.make_request("v1/character/ability", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_item_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterItemEquipment, ApiError> {
    api.make_request("v1/character/item-equipment", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_cashitem_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterCashItemEquipment, ApiError> {
    api.make_request("v1/character/cashitem-equipment", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_symbol_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterSymbolEquipment, ApiError> {
    api.make_request("v1/character/symbol-equipment", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_set_effect(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterSetEffect, ApiError> {
    api.make_request("v1/character/set-effect", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_beauty_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterBeautyEquipment, ApiError> {
    api.make_request("v1/character/beauty-equipment", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_android_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterAndroidEquipment, ApiError> {
    api.make_request("v1/character/android-equipment", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_pet_equipment(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterPetEquipment, ApiError> {
    api.make_request("v1/character/pet-equipment", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_skill(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterSkill, ApiError> {
    api.make_request("v1/character/skill", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_link_skill(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterLinkSkill, ApiError> {
    api.make_request("v1/character/link-skill", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_vmatrix(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterVMatrix, ApiError> {
    api.make_request("v1/character/vmatrix", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_hexamatrix(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterHexaMatrix, ApiError> {
    api.make_request("v1/character/hexamatrix", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_hexamatrix_stat(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterHexaMatrixStat, ApiError> {
    api.make_request("v1/character/hexamatrix-stat", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_dojang(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterDojang, ApiError> {
    api.make_request("v1/character/dojang", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_other_stat(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<CharacterOtherStat, ApiError> {
    api.make_request("v1/character/other-stat", params!(ocid, date))
        .await
}

pub(crate) async fn get_character_ring_exchange_skill_equipment(
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

pub(crate) async fn get_user_union(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<Union, ApiError> {
    api.make_request("v1/user/union", params!(ocid, date)).await
}

pub(crate) async fn get_user_union_raider(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<UnionRaider, ApiError> {
    api.make_request("v1/user/union-raider", params!(ocid, date))
        .await
}

pub(crate) async fn get_user_union_artifact(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<UnionArtifact, ApiError> {
    api.make_request("v1/user/union-artifact", params!(ocid, date))
        .await
}

pub(crate) async fn get_user_union_champion(
    api: &MaplestoryApi,
    ocid: &str,
    date: Option<&str>,
) -> Result<UnionChampion, ApiError> {
    api.make_request("v1/user/union-champion", params!(ocid, date))
        .await
}

pub(crate) async fn get_guild_id(
    api: &MaplestoryApi,
    guild_name: &str,
    world_name: &str,
) -> Result<Guild, ApiError> {
    api.make_request("v1/guild/id", params!(guild_name, world_name))
        .await
}

pub(crate) async fn get_guild_basic(
    api: &MaplestoryApi,
    oguild_id: &str,
    date: Option<&str>,
) -> Result<GuildBasic, ApiError> {
    api.make_request("v1/guild/basic", params!(oguild_id, date))
        .await
}

pub(crate) async fn get_ouid(api: &MaplestoryApi) -> Result<User, ApiError> {
    api.make_request("v1/ouid", params!()).await
}

pub(crate) async fn get_history_starforce(
    api: &MaplestoryApi,
    count: &str,
    date: Option<&str>,
    cursor: Option<&str>,
) -> Result<StarforceHistory, ApiError> {
    api.make_request("v1/history/starforce", params!(count, date, cursor))
        .await
}

pub(crate) async fn get_history_potential(
    api: &MaplestoryApi,
    count: &str,
    date: Option<&str>,
    cursor: Option<&str>,
) -> Result<PotentialHistory, ApiError> {
    api.make_request("v1/history/potential", params!(count, date, cursor))
        .await
}

pub(crate) async fn get_history_cube(
    api: &MaplestoryApi,
    count: &str,
    date: Option<&str>,
    cursor: Option<&str>,
) -> Result<CubeHistory, ApiError> {
    api.make_request("v1/history/cube", params!(count, date, cursor))
        .await
}

pub(crate) async fn get_ranking_overall(
    api: &MaplestoryApi,
    date: &str,
    world_name: Option<&str>,
    world_type: Option<&str>,
    class: Option<&str>,
    ocid: Option<&str>,
    page: Option<&str>,
) -> Result<OverallRanking, ApiError> {
    api.make_request(
        "v1/ranking/overall",
        params!(date, world_name, world_type, class, ocid, page),
    )
    .await
}

pub(crate) async fn get_ranking_union(
    api: &MaplestoryApi,
    date: &str,
    world_name: Option<&str>,
    ocid: Option<&str>,
    page: Option<&str>,
) -> Result<UnionRanking, ApiError> {
    api.make_request("v1/ranking/union", params!(date, world_name, ocid, page))
        .await
}

pub(crate) async fn get_ranking_guild(
    api: &MaplestoryApi,
    date: &str,
    world_name: Option<&str>,
    ranking_type: &str,
    guild_name: Option<&str>,
    page: Option<&str>,
) -> Result<GuildRanking, ApiError> {
    api.make_request(
        "v1/ranking/guild",
        params!(date, world_name, ranking_type, guild_name, page),
    )
    .await
}

pub(crate) async fn get_ranking_dojang(
    api: &MaplestoryApi,
    date: &str,
    world_name: Option<&str>,
    difficulty: &str,
    class: Option<&str>,
    ocid: Option<&str>,
    page: Option<&str>,
) -> Result<DojangRanking, ApiError> {
    api.make_request(
        "v1/ranking/dojang",
        params!(date, world_name, difficulty, class, ocid, page),
    )
    .await
}

pub(crate) async fn get_ranking_theseed(
    api: &MaplestoryApi,
    date: &str,
    world_name: Option<&str>,
    ocid: Option<&str>,
    page: Option<&str>,
) -> Result<TheSeedRanking, ApiError> {
    api.make_request("v1/ranking/theseed", params!(date, world_name, ocid, page))
        .await
}

pub(crate) async fn get_ranking_achievement(
    api: &MaplestoryApi,
    date: &str,
    ocid: Option<&str>,
    page: Option<&str>,
) -> Result<AchievementRanking, ApiError> {
    api.make_request("v1/ranking/achievement", params!(date, ocid, page))
        .await
}

pub(crate) async fn get_notice(api: &MaplestoryApi) -> Result<NoticeList, ApiError> {
    api.make_request("v1/notice", params!()).await
}

pub(crate) async fn get_notice_detail(
    api: &MaplestoryApi,
    notice_id: &str,
) -> Result<NoticeDetail, ApiError> {
    api.make_request("v1/notice/detail", params!(notice_id))
        .await
}

pub(crate) async fn get_notice_update(api: &MaplestoryApi) -> Result<UpdateNoticeList, ApiError> {
    api.make_request("v1/notice-update", params!()).await
}

pub(crate) async fn get_notice_update_detail(
    api: &MaplestoryApi,
    notice_id: &str,
) -> Result<UpdateNoticeDetail, ApiError> {
    api.make_request("v1/notice-update/detail", params!(notice_id))
        .await
}

pub(crate) async fn get_notice_event(api: &MaplestoryApi) -> Result<EventNoticeList, ApiError> {
    api.make_request("v1/notice-event", params!()).await
}

pub(crate) async fn get_notice_event_detail(
    api: &MaplestoryApi,
    notice_id: &str,
) -> Result<EventNoticeDetail, ApiError> {
    api.make_request("v1/notice-event/detail", params!(notice_id))
        .await
}

pub(crate) async fn get_notice_cashshop(
    api: &MaplestoryApi,
) -> Result<CashshopNoticeList, ApiError> {
    api.make_request("v1/notice-cashshop", params!()).await
}

pub(crate) async fn get_notice_cashshop_detail(
    api: &MaplestoryApi,
    notice_id: &str,
) -> Result<CashshopNoticeDetail, ApiError> {
    api.make_request("v1/notice-cashshop/detail", params!(notice_id))
        .await
}
