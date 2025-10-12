use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct UpdateNoticeList {
    pub update_notice: Vec<UpdateNotice>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct UpdateNotice {
    pub title: String,
    pub url: String,
    pub notice_id: i64,
    pub date: String,
}
