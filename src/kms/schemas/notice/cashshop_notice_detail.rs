use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CashshopNoticeDetail {
    pub title: String,
    pub url: String,
    pub contents: String,
    pub date: String,
    pub date_sale_start: Option<String>,
    pub date_sale_end: Option<String>,
    pub ongoing_flag: String,
}
