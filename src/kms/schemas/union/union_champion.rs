use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UnionChampion {
    pub date: Option<String>,
    pub union_champion: Vec<UnionChampionElement>,
    pub champion_badge_total_info: Vec<ChampionBadgeInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ChampionBadgeInfo {
    pub stat: String,
}

#[derive(Debug, Deserialize)]
pub struct UnionChampionElement {
    pub champion_name: String,
    pub champion_slot: i64,
    pub champion_grade: String,
    pub champion_class: String,
    pub champion_badge_info: Vec<ChampionBadgeInfo>,
}
