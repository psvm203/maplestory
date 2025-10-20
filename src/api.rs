use crate::{
    error::ApiError,
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
use std::fmt::Display;

macro_rules! params {
    ($($key: ident), *) => {
        {
            [$(
                $key.to_param().map(|value| (stringify!($key), value))
            ), *]
            .into_iter()
            .filter_map(|opt| opt)
            .collect()
        }
    };
}

trait Param {
    fn to_param(&self) -> Option<String>;
}

impl<T: Display> Param for Option<T> {
    fn to_param(&self) -> Option<String> {
        self.as_ref().map(|x| x.to_string())
    }
}

const API_KEY_HEADER_NAME: &str = "x-nxopen-api-key";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Region {
    KMS,
    MSEA,
    TMS,
}

/// Examples
///
/// ```rust
/// use maplestory::prelude::*;
///
/// async fn get_character_level() -> Result<i64, ApiError> {
///     let api = MaplestoryApi::builder()
///         .region(Region::KMS)
///         .api_key("YOUR_API_KEY")
///         .build();
///
///     let ocid = api.get_id("00월").await?.ocid;
///     let character_level = api.get_character_basic(ocid, Some("2025-10-20")).await?.character_level;
///
///     Ok(character_level)
/// }
///
/// # tokio_test::block_on(async {
/// // Error occurs due to an invalid API key.
/// // With a valid API key, you'll get `Some(281)`.
/// assert_eq!(get_character_level().await, Err(ApiError::InvalidApiKey))
/// # })
/// ```

#[derive(Clone)]
pub struct MaplestoryApi {
    pub(crate) region: Region,
    pub(crate) api_key: Option<String>,
    pub(crate) origin: String,
}

impl MaplestoryApi {
    pub fn builder() -> MaplestoryApiBuilder {
        MaplestoryApiBuilder::default()
    }

    pub(crate) async fn make_request<T>(
        &self,
        endpoint: &str,
        query_params: Vec<(&str, String)>,
    ) -> Result<T, ApiError>
    where
        for<'de> T: Deserialize<'de>,
    {
        let origin = &self.origin;
        let service = match self.region {
            Region::KMS => "maplestory",
            Region::MSEA => "maplestorysea",
            Region::TMS => "maplestorytw",
        };

        let mut request = reqwest::Client::new()
            .get(format!("{origin}/{service}/{endpoint}"))
            .query(&query_params);

        if let Some(api_key) = &self.api_key {
            request = request.header(API_KEY_HEADER_NAME, api_key);
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
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        self.make_request("v1/character/list", params!()).await
    }

    pub async fn get_user_achievement(&self) -> Result<Achievement, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        self.make_request("v1/user/achievement", params!()).await
    }

    pub async fn get_id(&self, character_name: impl Display) -> Result<Character, ApiError> {
        let character_name = Some(character_name);
        self.make_request("v1/id", params!(character_name)).await
    }

    pub async fn get_character_basic(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterBasic, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/basic", params!(ocid, date))
            .await
    }

    pub async fn get_character_popularity(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterPopularity, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/popularity", params!(ocid, date))
            .await
    }

    pub async fn get_character_stat(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterStat, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/stat", params!(ocid, date))
            .await
    }

    pub async fn get_character_hyperstat(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterHyperStat, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/hyper-stat", params!(ocid, date))
            .await
    }

    pub async fn get_character_propensity(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterPropensity, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/propensity", params!(ocid, date))
            .await
    }

    pub async fn get_character_ability(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterAbility, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/ability", params!(ocid, date))
            .await
    }

    pub async fn get_character_item_equipment(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterItemEquipment, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/item-equipment", params!(ocid, date))
            .await
    }

    pub async fn get_character_cashitem_equipment(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterCashItemEquipment, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/cashitem-equipment", params!(ocid, date))
            .await
    }

    pub async fn get_character_symbol_equipment(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterSymbolEquipment, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/symbol-equipment", params!(ocid, date))
            .await
    }

    pub async fn get_character_set_effect(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterSetEffect, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/set-effect", params!(ocid, date))
            .await
    }

    pub async fn get_character_beauty_equipment(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterBeautyEquipment, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/beauty-equipment", params!(ocid, date))
            .await
    }

    pub async fn get_character_android_equipment(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterAndroidEquipment, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/android-equipment", params!(ocid, date))
            .await
    }

    pub async fn get_character_pet_equipment(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterPetEquipment, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/pet-equipment", params!(ocid, date))
            .await
    }

    pub async fn get_character_skill(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterSkill, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/skill", params!(ocid, date))
            .await
    }

    pub async fn get_character_link_skill(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterLinkSkill, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/link-skill", params!(ocid, date))
            .await
    }

    pub async fn get_character_vmatrix(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterVMatrix, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/vmatrix", params!(ocid, date))
            .await
    }

    pub async fn get_character_hexamatrix(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterHexaMatrix, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/hexamatrix", params!(ocid, date))
            .await
    }

    pub async fn get_character_hexamatrix_stat(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterHexaMatrixStat, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/hexamatrix-stat", params!(ocid, date))
            .await
    }

    pub async fn get_character_dojang(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterDojang, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/character/dojang", params!(ocid, date))
            .await
    }

    pub async fn get_character_other_stat(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<CharacterOtherStat, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let ocid = Some(ocid);
        self.make_request("v1/character/other-stat", params!(ocid, date))
            .await
    }

    pub async fn get_character_ring_exchange_skill_equipment(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<RingExchangeSkillEquipment, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let ocid = Some(ocid);
        self.make_request(
            "v1/character/ring-exchange-skill-equipment",
            params!(ocid, date),
        )
        .await
    }

    pub async fn get_user_union(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<Union, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/user/union", params!(ocid, date))
            .await
    }

    pub async fn get_user_union_raider(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<UnionRaider, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/user/union-raider", params!(ocid, date))
            .await
    }

    pub async fn get_user_union_artifact(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<UnionArtifact, ApiError> {
        let ocid = Some(ocid);
        self.make_request("v1/user/union-artifact", params!(ocid, date))
            .await
    }

    pub async fn get_user_union_champion(
        &self,
        ocid: impl Display,
        date: Option<impl Display>,
    ) -> Result<UnionChampion, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let ocid = Some(ocid);
        self.make_request("v1/user/union-champion", params!(ocid, date))
            .await
    }

    pub async fn get_guild_id(
        &self,
        guild_name: impl Display,
        world_name: impl Display,
    ) -> Result<Guild, ApiError> {
        let guild_name = Some(guild_name);
        let world_name = Some(world_name);
        self.make_request("v1/guild/id", params!(guild_name, world_name))
            .await
    }

    pub async fn get_guild_basic(
        &self,
        oguild_id: impl Display,
        date: Option<impl Display>,
    ) -> Result<GuildBasic, ApiError> {
        let oguild_id = Some(oguild_id);
        self.make_request("v1/guild/basic", params!(oguild_id, date))
            .await
    }

    pub async fn get_ouid(&self) -> Result<User, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        self.make_request("v1/ouid", params!()).await
    }

    pub async fn get_history_starforce(
        &self,
        count: impl Display,
        date: Option<impl Display>,
        cursor: Option<impl Display>,
    ) -> Result<StarforceHistory, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let count = Some(count);
        self.make_request("v1/history/starforce", params!(count, date, cursor))
            .await
    }

    pub async fn get_history_potential(
        &self,
        count: impl Display,
        date: Option<impl Display>,
        cursor: Option<impl Display>,
    ) -> Result<PotentialHistory, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let count = Some(count);
        self.make_request("v1/history/potential", params!(count, date, cursor))
            .await
    }

    pub async fn get_history_cube(
        &self,
        count: impl Display,
        date: Option<impl Display>,
        cursor: Option<impl Display>,
    ) -> Result<CubeHistory, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let count = Some(count);
        self.make_request("v1/history/cube", params!(count, date, cursor))
            .await
    }

    pub async fn get_ranking_overall(
        &self,
        date: impl Display,
        world_name: Option<impl Display>,
        world_type: Option<impl Display>,
        class: Option<impl Display>,
        ocid: Option<impl Display>,
        page: Option<impl Display>,
    ) -> Result<OverallRanking, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let date = Some(date);
        self.make_request(
            "v1/ranking/overall",
            params!(date, world_name, world_type, class, ocid, page),
        )
        .await
    }

    pub async fn get_ranking_union(
        &self,
        date: impl Display,
        world_name: Option<impl Display>,
        ocid: Option<impl Display>,
        page: Option<impl Display>,
    ) -> Result<UnionRanking, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let date = Some(date);
        self.make_request("v1/ranking/union", params!(date, world_name, ocid, page))
            .await
    }

    pub async fn get_ranking_guild(
        &self,
        date: impl Display,
        world_name: Option<impl Display>,
        ranking_type: impl Display,
        guild_name: Option<impl Display>,
        page: Option<impl Display>,
    ) -> Result<GuildRanking, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let date = Some(date);
        let ranking_type = Some(ranking_type);
        self.make_request(
            "v1/ranking/guild",
            params!(date, world_name, ranking_type, guild_name, page),
        )
        .await
    }

    pub async fn get_ranking_dojang(
        &self,
        date: impl Display,
        world_name: Option<impl Display>,
        difficulty: impl Display,
        class: Option<impl Display>,
        ocid: Option<impl Display>,
        page: Option<impl Display>,
    ) -> Result<DojangRanking, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let date = Some(date);
        let difficulty = Some(difficulty);
        self.make_request(
            "v1/ranking/dojang",
            params!(date, world_name, difficulty, class, ocid, page),
        )
        .await
    }

    pub async fn get_ranking_theseed(
        &self,
        date: impl Display,
        world_name: Option<impl Display>,
        ocid: Option<impl Display>,
        page: Option<impl Display>,
    ) -> Result<TheSeedRanking, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let date = Some(date);
        self.make_request("v1/ranking/theseed", params!(date, world_name, ocid, page))
            .await
    }

    pub async fn get_ranking_achievement(
        &self,
        date: impl Display,
        ocid: Option<impl Display>,
        page: Option<impl Display>,
    ) -> Result<AchievementRanking, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let date = Some(date);
        self.make_request("v1/ranking/achievement", params!(date, ocid, page))
            .await
    }

    pub async fn get_notice(&self) -> Result<NoticeList, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        self.make_request("v1/notice", params!()).await
    }

    pub async fn get_notice_detail(
        &self,
        notice_id: impl Display,
    ) -> Result<NoticeDetail, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let notice_id = Some(notice_id);
        self.make_request("v1/notice/detail", params!(notice_id))
            .await
    }

    pub async fn get_notice_update(&self) -> Result<UpdateNoticeList, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        self.make_request("v1/notice-update", params!()).await
    }

    pub async fn get_notice_update_detail(
        &self,
        notice_id: impl Display,
    ) -> Result<UpdateNoticeDetail, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let notice_id = Some(notice_id);
        self.make_request("v1/notice-update/detail", params!(notice_id))
            .await
    }

    pub async fn get_notice_event(&self) -> Result<EventNoticeList, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        self.make_request("v1/notice-event", params!()).await
    }

    pub async fn get_notice_event_detail(
        &self,
        notice_id: impl Display,
    ) -> Result<EventNoticeDetail, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let notice_id = Some(notice_id);
        self.make_request("v1/notice-event/detail", params!(notice_id))
            .await
    }

    pub async fn get_notice_cashshop(&self) -> Result<CashshopNoticeList, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        self.make_request("v1/notice-cashshop", params!()).await
    }

    pub async fn get_notice_cashshop_detail(
        &self,
        notice_id: impl Display,
    ) -> Result<CashshopNoticeDetail, ApiError> {
        if self.region != Region::KMS {
            return Err(ApiError::NotSupported);
        }

        let notice_id = Some(notice_id);
        self.make_request("v1/notice-cashshop/detail", params!(notice_id))
            .await
    }
}

/// Generate [`MaplestoryApi`] using Builder Pattern.
///
/// `region` represents the region(KMS/MSEA/TMS). Default is [`Region::KMS`].
///
/// `api_key` is required to access [NEXON Open API](https://openapi.nexon.com/). If `None`, "x-nxopen-api-key" header will not be included in API request.
///
/// `origin` indicates the base URL of maplestory API server. If you want [Official API server](https://open.api.nexon.com)(default), leave this field as `None`. If you want proxy server, take a look an example at [nexon-open-api-proxy](https://github.com/psvm203/nexon-open-api-proxy).
///
/// # Examples
///
/// ```rust
/// use maplestory::prelude::*;
///
/// # fn main() {
/// let builder = MaplestoryApi::builder()
///     .region(Region::KMS)
///     .api_key("YOUR_API_KEY");
///
/// assert_eq!(builder.region, Some(Region::KMS));
/// assert_eq!(builder.api_key, Some("YOUR_API_KEY".to_owned()));
/// assert_eq!(builder.origin, None);
///
/// let api = builder.build();
/// # }
/// ```

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

    pub fn build(self) -> MaplestoryApi {
        let region = self.region.unwrap_or(Region::KMS);

        let api_key = self.api_key;

        let origin = self
            .origin
            .unwrap_or_else(|| "https://open.api.nexon.com".to_owned());

        MaplestoryApi {
            region,
            api_key,
            origin,
        }
    }
}
