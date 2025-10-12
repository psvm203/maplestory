use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UnionArtifact {
    pub date: Option<String>,
    pub union_artifact_effect: Vec<UnionArtifactEffect>,
    pub union_artifact_crystal: Vec<UnionArtifactCrystal>,
    pub union_artifact_remain_ap: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UnionArtifactCrystal {
    pub name: String,
    pub validity_flag: String,
    pub date_expire: String,
    pub level: i64,
    pub crystal_option_name_1: String,
    pub crystal_option_name_2: String,
    pub crystal_option_name_3: String,
}

#[derive(Debug, Deserialize)]
pub struct UnionArtifactEffect {
    pub name: String,
    pub level: i64,
}
