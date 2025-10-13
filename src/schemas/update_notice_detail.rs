use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct UpdateNoticeDetail {
    pub title: String,
    pub url: String,
    pub contents: String,
    pub date: String,
}
