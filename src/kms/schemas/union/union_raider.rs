use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct UnionRaider {
    pub date: Option<String>,
    pub union_raider_stat: Vec<String>,
    pub union_occupied_stat: Vec<String>,
    pub union_block: Vec<UnionBlock>,
    pub union_inner_stat: Vec<UnionInnerStat>,
    pub use_preset_no: i64,
    pub union_raider_preset_1: Option<UnionRaiderPreset>,
    pub union_raider_preset_2: Option<UnionRaiderPreset>,
    pub union_raider_preset_3: Option<UnionRaiderPreset>,
    pub union_raider_preset_4: Option<UnionRaiderPreset>,
    pub union_raider_preset_5: Option<UnionRaiderPreset>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct UnionBlock {
    pub block_type: String,
    pub block_class: String,
    pub block_level: String,
    pub block_control_point: Block,
    pub block_position: Vec<Block>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct UnionInnerStat {
    pub stat_field_id: String,
    pub stat_field_effect: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct UnionRaiderPreset {
    pub union_raider_stat: Vec<String>,
    pub union_occupied_stat: Vec<String>,
    pub union_block: Vec<UnionBlock>,
    pub union_inner_stat: Vec<UnionInnerStat>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Block {
    pub x: i64,
    pub y: i64,
}
