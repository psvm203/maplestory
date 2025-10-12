use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct NoticeList {
    pub notice: Vec<Notice>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Notice {
    pub title: String,
    pub url: String,
    pub notice_id: i64,
    pub date: String,
}
