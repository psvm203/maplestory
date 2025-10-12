use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NoticeDetail {
    pub title: String,
    pub url: String,
    pub contents: String,
    pub date: String,
}
