use crate::{
    error::ApiError,
    kms,
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
        dojang_ranking::DojangRanking, error_message::ErrorMessage,
        event_notice_detail::EventNoticeDetail, event_notice_list::EventNoticeList, guild::Guild,
        guild_basic::GuildBasic, guild_ranking::GuildRanking, notice_detail::NoticeDetail,
        notice_list::NoticeList, overall_ranking::OverallRanking,
        potential_history::PotentialHistory,
        ring_exchange_skill_equipment::RingExchangeSkillEquipment,
        starforce_history::StarforceHistory, the_seed_ranking::TheSeedRanking, union::Union,
        union_artifact::UnionArtifact, union_champion::UnionChampion, union_raider::UnionRaider,
        union_ranking::UnionRanking, update_notice_detail::UpdateNoticeDetail,
        update_notice_list::UpdateNoticeList, user::User,
    },
};
use serde::de::Deserialize;

const API_KEY_HEADER_NAME: &str = "x-nxopen-api-key";

pub enum Region {
    KMS,
    MSEA,
}

pub struct MaplestoryApi {
    pub(crate) region: Region,
    pub(crate) api_key: String,
    pub(crate) origin: String,
}

impl MaplestoryApi {
    pub fn builder() -> MaplestoryApiBuilder {
        MaplestoryApiBuilder::default()
    }

    pub(crate) async fn make_request<T>(
        &self,
        endpoint: &str,
        query_params: Vec<(&str, &str)>,
    ) -> Result<T, ApiError>
    where
        for<'de> T: Deserialize<'de>,
    {
        let response = reqwest::Client::new()
            .get(format!("{}/maplestory/{endpoint}", &self.origin))
            .header(API_KEY_HEADER_NAME, &self.api_key)
            .query(&query_params)
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

    pub async fn get_character_basic(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterBasic, ApiError> {
        kms::get_character_basic(self, ocid, date).await
    }

    pub async fn get_character_popularity(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterPopularity, ApiError> {
        kms::get_character_popularity(self, ocid, date).await
    }

    pub async fn get_character_stat(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterStat, ApiError> {
        kms::get_character_stat(self, ocid, date).await
    }

    pub async fn get_character_hyperstat(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterHyperStat, ApiError> {
        kms::get_character_hyperstat(self, ocid, date).await
    }

    pub async fn get_character_propensity(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterPropensity, ApiError> {
        kms::get_character_propensity(self, ocid, date).await
    }

    pub async fn get_character_ability(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterAbility, ApiError> {
        kms::get_character_ability(self, ocid, date).await
    }

    pub async fn get_character_item_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterItemEquipment, ApiError> {
        kms::get_character_item_equipment(self, ocid, date).await
    }

    pub async fn get_character_cashitem_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterCashItemEquipment, ApiError> {
        kms::get_character_cashitem_equipment(self, ocid, date).await
    }

    pub async fn get_character_symbol_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterSymbolEquipment, ApiError> {
        kms::get_character_symbol_equipment(self, ocid, date).await
    }

    pub async fn get_character_set_effect(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterSetEffect, ApiError> {
        kms::get_character_set_effect(self, ocid, date).await
    }

    pub async fn get_character_beauty_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterBeautyEquipment, ApiError> {
        kms::get_character_beauty_equipment(self, ocid, date).await
    }

    pub async fn get_character_android_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterAndroidEquipment, ApiError> {
        kms::get_character_android_equipment(self, ocid, date).await
    }

    pub async fn get_character_pet_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterPetEquipment, ApiError> {
        kms::get_character_pet_equipment(self, ocid, date).await
    }

    pub async fn get_character_skill(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterSkill, ApiError> {
        kms::get_character_skill(self, ocid, date).await
    }

    pub async fn get_character_link_skill(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterLinkSkill, ApiError> {
        kms::get_character_link_skill(self, ocid, date).await
    }

    pub async fn get_character_vmatrix(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterVMatrix, ApiError> {
        kms::get_character_vmatrix(self, ocid, date).await
    }

    pub async fn get_character_hexamatrix(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterHexaMatrix, ApiError> {
        kms::get_character_hexamatrix(self, ocid, date).await
    }

    pub async fn get_character_hexamatrix_stat(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterHexaMatrixStat, ApiError> {
        kms::get_character_hexamatrix_stat(self, ocid, date).await
    }

    pub async fn get_character_dojang(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterDojang, ApiError> {
        kms::get_character_dojang(self, ocid, date).await
    }

    pub async fn get_character_other_stat(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<CharacterOtherStat, ApiError> {
        kms::get_character_other_stat(self, ocid, date).await
    }

    pub async fn get_character_ring_exchange_skill_equipment(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<RingExchangeSkillEquipment, ApiError> {
        kms::get_character_ring_exchange_skill_equipment(self, ocid, date).await
    }

    pub async fn get_user_union(&self, ocid: &str, date: Option<&str>) -> Result<Union, ApiError> {
        kms::get_user_union(self, ocid, date).await
    }

    pub async fn get_user_union_raider(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<UnionRaider, ApiError> {
        kms::get_user_union_raider(self, ocid, date).await
    }

    pub async fn get_user_union_artifact(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<UnionArtifact, ApiError> {
        kms::get_user_union_artifact(self, ocid, date).await
    }

    pub async fn get_user_union_champion(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<UnionChampion, ApiError> {
        kms::get_user_union_champion(self, ocid, date).await
    }

    pub async fn get_guild_id(
        &self,
        guild_name: &str,
        world_name: &str,
    ) -> Result<Guild, ApiError> {
        kms::get_guild_id(self, guild_name, world_name).await
    }

    pub async fn get_guild_basic(
        &self,
        oguild_id: &str,
        date: Option<&str>,
    ) -> Result<GuildBasic, ApiError> {
        kms::get_guild_basic(self, oguild_id, date).await
    }

    pub async fn get_ouid(&self) -> Result<User, ApiError> {
        kms::get_ouid(self).await
    }

    pub async fn get_history_starforce(
        &self,
        count: &str,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<StarforceHistory, ApiError> {
        kms::get_history_starforce(self, count, date, cursor).await
    }

    pub async fn get_history_potential(
        &self,
        count: &str,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<PotentialHistory, ApiError> {
        kms::get_history_potential(self, count, date, cursor).await
    }

    pub async fn get_history_cube(
        &self,
        count: &str,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<CubeHistory, ApiError> {
        kms::get_history_cube(self, count, date, cursor).await
    }

    pub async fn get_ranking_overall(
        &self,
        date: &str,
        world_name: Option<&str>,
        world_type: Option<&str>,
        class: Option<&str>,
        ocid: Option<&str>,
        page: Option<&str>,
    ) -> Result<OverallRanking, ApiError> {
        kms::get_ranking_overall(self, date, world_name, world_type, class, ocid, page).await
    }

    pub async fn get_ranking_union(
        &self,
        date: &str,
        world_name: Option<&str>,
        ocid: Option<&str>,
        page: Option<&str>,
    ) -> Result<UnionRanking, ApiError> {
        kms::get_ranking_union(self, date, world_name, ocid, page).await
    }

    pub async fn get_ranking_guild(
        &self,
        date: &str,
        world_name: Option<&str>,
        ranking_type: &str,
        guild_name: Option<&str>,
        page: Option<&str>,
    ) -> Result<GuildRanking, ApiError> {
        kms::get_ranking_guild(self, date, world_name, ranking_type, guild_name, page).await
    }

    pub async fn get_ranking_dojang(
        &self,
        date: &str,
        world_name: Option<&str>,
        difficulty: &str,
        class: Option<&str>,
        ocid: Option<&str>,
        page: Option<&str>,
    ) -> Result<DojangRanking, ApiError> {
        kms::get_ranking_dojang(self, date, world_name, difficulty, class, ocid, page).await
    }

    pub async fn get_ranking_theseed(
        &self,
        date: &str,
        world_name: Option<&str>,
        ocid: Option<&str>,
        page: Option<&str>,
    ) -> Result<TheSeedRanking, ApiError> {
        kms::get_ranking_theseed(self, date, world_name, ocid, page).await
    }

    pub async fn get_ranking_achievement(
        &self,
        date: &str,
        ocid: Option<&str>,
        page: Option<&str>,
    ) -> Result<AchievementRanking, ApiError> {
        kms::get_ranking_achievement(self, date, ocid, page).await
    }

    pub async fn get_notice(&self) -> Result<NoticeList, ApiError> {
        kms::get_notice(self).await
    }

    pub async fn get_notice_detail(&self, notice_id: &str) -> Result<NoticeDetail, ApiError> {
        kms::get_notice_detail(self, notice_id).await
    }

    pub async fn get_notice_update(&self) -> Result<UpdateNoticeList, ApiError> {
        kms::get_notice_update(self).await
    }

    pub async fn get_notice_update_detail(
        &self,
        notice_id: &str,
    ) -> Result<UpdateNoticeDetail, ApiError> {
        kms::get_notice_update_detail(self, notice_id).await
    }

    pub async fn get_notice_event(&self) -> Result<EventNoticeList, ApiError> {
        kms::get_notice_event(self).await
    }

    pub async fn get_notice_event_detail(
        &self,
        notice_id: &str,
    ) -> Result<EventNoticeDetail, ApiError> {
        kms::get_notice_event_detail(self, notice_id).await
    }

    pub async fn get_notice_cashshop(&self) -> Result<CashshopNoticeList, ApiError> {
        kms::get_notice_cashshop(self).await
    }

    pub async fn get_notice_cashshop_detail(
        &self,
        notice_id: &str,
    ) -> Result<CashshopNoticeDetail, ApiError> {
        kms::get_notice_cashshop_detail(self, notice_id).await
    }
}

#[derive(Default)]
pub struct MaplestoryApiBuilder {
    pub region: Option<Region>,
    pub api_key: Option<String>,
    pub origin: Option<String>,
}

impl MaplestoryApiBuilder {
    pub fn new() -> Self {
        Self::default()
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
