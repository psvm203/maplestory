use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct NoticeDetail {
    pub title: String,
    pub url: String,
    pub contents: String,
    pub date: String,
}
