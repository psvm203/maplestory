use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Union {
    pub date: Option<String>,
    pub union_level: Option<i64>,
    pub union_grade: Option<String>,
    pub union_artifact_level: Option<i64>,
    pub union_artifact_exp: Option<i64>,
    pub union_artifact_point: Option<i64>,
}
